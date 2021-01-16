use crate::annotations::Entity;
use crate::definitions::{
    ColumnDefinition, ColumnType, IndexDefinition, TableDefinition, TableType,
};
use crate::resolver::error::ResolveError;
use crate::resolver::{AchievedFieldResolver, EntityPath, FieldName};
use crate::types::DatabaseType;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum EntityResolveStatus {
    Finished,
    Assemble,
    Unresolved,
}

#[allow(dead_code)]
pub struct EntityResolver {
    status: EntityResolveStatus,
    mod_path: &'static str,
    ident: Ident,
    field_count: usize,
    annotation: Entity,
    field_resolvers: HashMap<FieldName, AchievedFieldResolver>,
    primary_keys: Vec<String>,
}

impl EntityResolver {
    pub fn new(
        ident: Ident,
        mod_path: &'static str,
        field_count: usize,
        annotation: Option<Entity>,
    ) -> Self {
        let mut resolved_annotation = annotation.unwrap_or(Entity {
            name: None,
            indexes: None,
        });

        if resolved_annotation.name.is_none() {
            resolved_annotation.name = Some(ident.to_string().to_snake_case())
        };
        if resolved_annotation.indexes.is_none() {
            resolved_annotation.indexes = Some(HashMap::new())
        };

        EntityResolver {
            status: EntityResolveStatus::Unresolved,
            mod_path,
            ident,
            field_count,
            annotation: resolved_annotation,
            field_resolvers: HashMap::new(),
            primary_keys: vec![],
        }
    }

    pub fn entity_path(&self) -> EntityPath {
        format!("{}::{}", &self.mod_path, &self.ident)
    }

    pub fn status(&self) -> EntityResolveStatus {
        self.status
    }

    pub fn get_field_resolver(&self, field: &str) -> Result<&AchievedFieldResolver, ResolveError> {
        self.field_resolvers.get(field).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(self.entity_path(), field.to_string())
        })
    }

    pub fn assemble_field(
        &mut self,
        field: AchievedFieldResolver,
    ) -> Result<EntityResolveStatus, ResolveError> {
        let mut primary_key_column_names = field.primary_key_column_names();

        self.primary_keys.append(&mut primary_key_column_names);

        self.field_resolvers
            .insert(field.field_path.1.clone(), field);

        self.status = if self.field_resolvers.len() == self.field_count {
            EntityResolveStatus::Finished
        } else {
            EntityResolveStatus::Assemble
        };

        Ok(self.status)
    }

    pub fn achieve(mut self) -> Result<AchievedEntityResolver, ResolveError> {
        if self.status != EntityResolveStatus::Finished {
            Err(ResolveError::EntityResolverIsNotFinished(
                self.entity_path(),
            ))
        } else {
            let mut columns = vec![];
            let mut tables = vec![];
            let mut foreign_keys = vec![];

            if self.primary_keys.is_empty() {
                let auto_primary_keys = ColumnDefinition {
                    name: format!("__{}_id", self.ident.to_string().to_snake_case()),
                    ty: ColumnType::VisualColumn,
                    data_type: DatabaseType::String,
                    unique: true,
                    auto_increase: false,
                    primary_key: true,
                };
                self.primary_keys.push(auto_primary_keys.name.clone());
                columns.push(auto_primary_keys);
            }

            for resolver in self.field_resolvers.values() {
                let mut field_columns = resolver.columns.clone();
                let mut joined_tables = resolver.joined_table.clone();
                let mut field_foreign_keys = resolver.foreign_keys.clone();
                columns.append(&mut field_columns);
                tables.append(&mut joined_tables);
                foreign_keys.append(&mut field_foreign_keys);
            }

            let indexes = self
                .annotation
                .indexes
                .as_ref()
                .unwrap()
                .iter()
                .map(|(name, index)| {
                    let mut columns = vec![];
                    for field_name in index.fields.iter() {
                        let mut column_names = self.get_field_resolver(field_name)?.column_names();
                        columns.append(&mut column_names)
                    }
                    Ok(IndexDefinition {
                        name: name.clone(),
                        columns,
                        method: index.method,
                        unique: index.unique,
                    })
                })
                .collect::<Result<Vec<_>, ResolveError>>()?;

            tables.push(TableDefinition {
                name: self.annotation.name.clone().unwrap(),
                ty: TableType::NormalEntityTable(self.entity_path()),
                columns,
                indexes,
                foreign_keys,
            });

            Ok(AchievedEntityResolver {
                definitions: tables.clone(),
                implement: self.get_implement_token_stream(tables),
            })
        }
    }

    fn get_implement_token_stream(&self, definitions: Vec<TableDefinition>) -> TokenStream {
        assert_eq!(self.status(), EntityResolveStatus::Assemble);
        let ident = TokenStream::from_str(self.entity_path().as_str()).unwrap();

        let converters: Vec<_> = self
            .field_resolvers
            .values()
            .into_iter()
            .map(|resolver| resolver.data_converter_token_stream.clone())
            .collect();

        let temp_values: Vec<_> = self
            .field_resolvers
            .values()
            .map(|resolver| {
                let method = resolver.converter_getter_ident.clone();

                let field_ident = format_ident!("{}", &resolver.field_path.1);

                quote::quote! {
                    let #field_ident = Self::#method().to_value(result)?
                }
            })
            .collect();

        let fields: Vec<_> = self
            .field_resolvers
            .values()
            .map(|resolver| format_ident!("{}", &resolver.field_path.1))
            .collect();

        let inserts: Vec<_> = self
            .field_resolvers
            .values()
            .map(|resolver| {
                let method = resolver.converter_getter_ident.clone();

                let field_ident = format_ident!("{}", &resolver.field_path.1);

                quote::quote! {
                    map.extend(Self::#method().to_database_value_by_ref(&self.#field_ident)?)
                }
            })
            .collect();

        quote! {
            impl #ident {
                #(#converters)*
            }

            impl yukino::Entity for #ident {
                fn from_database_value(
                    result: &std::collections::HashMap<String, yukino::mapping::DatabaseValue>
                ) -> Result<Box<Self>, yukino::resolver::error::DataConvertError> {
                    use yukino::resolver::ValueConverter;

                    #(#temp_values;)*

                    Ok(Box::new(
                        #ident {
                            #(#fields),*
                        }
                    ))
                }

                fn to_database_value(&self)
                    -> Result<
                        std::collections::HashMap<String, yukino::types::DatabaseValue>,
                        yukino::DataConvertError
                    > {
                    let mut map = std::collections::HashMap::new();
                    use yukino::resolver::ValueConverter;
                    #(#inserts;)*

                    Ok(map)
                }

                fn get_definitions() -> Vec<yukino::definitions::TableDefinition> {
                    vec![
                        #(#definitions),*
                    ]
                }
            }
        }
    }
}

pub struct AchievedEntityResolver {
    pub definitions: Vec<TableDefinition>,
    pub implement: TokenStream,
}
