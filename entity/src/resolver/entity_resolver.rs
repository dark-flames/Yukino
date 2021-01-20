use crate::annotations::Entity;
use crate::definitions::{
    ColumnDefinition, ColumnType, IndexDefinition, TableDefinition, TableType,
};
use crate::resolver::error::ResolveError;
use crate::resolver::{AchievedFieldResolver, EntityPath, FieldName};
use crate::types::DatabaseType;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::DeriveInput;

pub trait EntityResolverPass {
    fn new() -> Self
    where
        Self: Sized;

    fn boxed(&self) -> Box<dyn EntityResolverPass>;

    fn get_implement_token_stream(
        &self,
        _entity_path: String,
        _ident: &Ident,
        _definitions: &[TableDefinition],
        _field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        _derive_input: &DeriveInput,
    ) -> Option<TokenStream> {
        None
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum EntityResolveStatus {
    Finished,
    Assemble,
    Unresolved,
}

pub struct EntityResolver {
    status: EntityResolveStatus,
    mod_path: &'static str,
    ident: Ident,
    field_count: usize,
    annotation: Entity,
    field_resolvers: HashMap<FieldName, AchievedFieldResolver>,
    primary_keys: Vec<String>,
    resolver_passes: Vec<Box<dyn EntityResolverPass>>,
    derive_input: DeriveInput,
}

impl EntityResolver {
    pub fn new(
        ident: Ident,
        mod_path: &'static str,
        field_count: usize,
        annotation: Option<Entity>,
        resolver_passes: Vec<Box<dyn EntityResolverPass>>,
        derive_input: DeriveInput,
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
            resolver_passes,
            derive_input,
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

            let implements = self
                .resolver_passes
                .iter()
                .filter_map(|pass| {
                    pass.get_implement_token_stream(
                        self.entity_path(),
                        &self.ident,
                        &tables,
                        &self.field_resolvers,
                        &self.derive_input,
                    )
                })
                .fold(TokenStream::new(), |mut previous, current| {
                    current.to_tokens(&mut previous);

                    previous
                });

            Ok(AchievedEntityResolver {
                definitions: tables,
                implement: quote! {
                    #implements
                },
            })
        }
    }
}

pub struct AchievedEntityResolver {
    pub definitions: Vec<TableDefinition>,
    pub implement: TokenStream,
}
