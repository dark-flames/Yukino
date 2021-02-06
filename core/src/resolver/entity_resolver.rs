use crate::annotations::{Entity, IndexMethod};
use crate::definitions::{
    ColumnDefinition, ColumnType, IndexDefinition, TableDefinition, TableType,
};
use crate::resolver::error::ResolveError;
use crate::resolver::{AchievedFieldResolver, EntityName, FieldName, TypePathResolver};
use crate::types::DatabaseType;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::{HashMap, HashSet};
use syn::ItemStruct;

pub type EntityResolverPassBox = Box<dyn EntityResolverPass>;

pub trait EntityResolverPass {
    fn new() -> Self
    where
        Self: Sized;

    fn boxed(&self) -> EntityResolverPassBox;

    fn get_implement_token_stream(
        &self,
        _entity_name: String,
        _definitions: &[TableDefinition],
        _field_resolvers: &HashMap<FieldName, AchievedFieldResolver>,
        _input: &ItemStruct,
        _type_path_resolver: &TypePathResolver,
    ) -> Option<Result<TokenStream, ResolveError>> {
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
    ident: Ident,
    field_count: usize,
    annotation: Entity,
    field_resolvers: HashMap<FieldName, AchievedFieldResolver>,
    primary_keys: Vec<String>,
    resolver_passes: Vec<Box<dyn EntityResolverPass>>,
    input: ItemStruct,
}

impl EntityResolver {
    pub fn new(
        ident: Ident,
        field_count: usize,
        annotation: Option<Entity>,
        resolver_passes: Vec<Box<dyn EntityResolverPass>>,
        input: ItemStruct,
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
            ident,
            field_count,
            annotation: resolved_annotation,
            field_resolvers: HashMap::new(),
            primary_keys: vec![],
            resolver_passes,
            input,
        }
    }

    pub fn entity_name(&self) -> EntityName {
        self.ident.to_string()
    }

    pub fn table_name(&self) -> String {
        self.annotation.name.clone().unwrap()
    }

    pub fn status(&self) -> EntityResolveStatus {
        self.status
    }

    pub fn get_field_resolver(&self, field: &str) -> Result<&AchievedFieldResolver, ResolveError> {
        self.field_resolvers.get(field).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(self.entity_name(), field.to_string())
        })
    }

    fn assert_finished(&self) -> Result<(), ResolveError> {
        if self.status != EntityResolveStatus::Finished {
            Err(ResolveError::EntityResolverIsNotFinished(
                self.entity_name(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn get_primary_columns(&self) -> Result<Vec<String>, ResolveError> {
        self.assert_finished()?;

        Ok(self.primary_keys.clone())
    }

    pub fn is_unique_fields(&self, fields: &[String]) -> Result<bool, ResolveError> {
        self.assert_finished()?;

        for field_name in fields {
            if self.get_field_resolver(field_name)?.unique() {
                return Ok(true);
            }
        }

        let fields_set: HashSet<_> = fields.iter().cloned().collect();

        for (_, index) in self.annotation.indexes.clone().unwrap_or_else(HashMap::new) {
            if index.unique
                && index
                    .fields
                    .iter()
                    .map(|field_name| fields_set.contains(field_name))
                    .all(|result| result)
            {
                return Ok(true);
            }
        }

        if self
            .primary_keys
            .iter()
            .map(|field_name| fields_set.contains(field_name))
            .all(|result| result)
        {
            return Ok(true);
        }

        Ok(false)
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

    pub fn achieve(
        mut self,
        type_path_resolver: &TypePathResolver,
    ) -> Result<AchievedEntityResolver, ResolveError> {
        self.assert_finished()?;

        let mut columns = vec![];
        let mut tables = vec![];
        let mut foreign_keys = vec![];

        let mut indexes = self
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

        for resolver in self.field_resolvers.values() {
            let mut field_columns = resolver.columns.clone();
            let mut joined_tables = resolver.joined_table.clone();
            let mut field_foreign_keys = resolver.foreign_keys.clone();
            let mut field_indexes = resolver.indexes.clone();
            for column in field_columns.iter() {
                if column.primary_key && !column.data_type.suitable_for_primary_key() {
                    return Err(ResolveError::UnsuitableColumnDataTypeForPrimaryKey(
                        resolver.field_path.0.clone(),
                        resolver.field_path.1.clone(),
                    ));
                }
            }
            columns.append(&mut field_columns);
            tables.append(&mut joined_tables);
            foreign_keys.append(&mut field_foreign_keys);
            indexes.append(&mut field_indexes)
        }

        let primary_key_count = columns.iter().filter(|item| item.primary_key).count();

        if primary_key_count == 0 {
            let auto_primary_keys = ColumnDefinition {
                name: format!("__{}_id", self.ident.to_string().to_snake_case()),
                ty: ColumnType::VisualColumn,
                data_type: DatabaseType::String,
                unique: true,
                auto_increase: false,
                primary_key: true,
                nullable: false,
            };
            self.primary_keys.push(auto_primary_keys.name.clone());
            columns.push(auto_primary_keys);
        } else if primary_key_count == 1 {
            columns
                .iter_mut()
                .find(|item| item.primary_key)
                .unwrap()
                .unique = true;
        } else {
            indexes.push(IndexDefinition {
                name: "__primary_keys_index".to_string(),
                columns: columns
                    .iter()
                    .filter(|item| item.primary_key)
                    .map(|item| item.name.clone())
                    .collect(),
                method: IndexMethod::BTree,
                unique: true,
            });
        }

        tables.push(TableDefinition {
            name: self.annotation.name.clone().unwrap(),
            ty: TableType::NormalEntityTable(self.entity_name()),
            columns,
            indexes,
            foreign_keys,
        });

        let implements = self
            .resolver_passes
            .iter()
            .filter_map(|pass| {
                pass.get_implement_token_stream(
                    self.entity_name(),
                    &tables,
                    &self.field_resolvers,
                    &self.input,
                    type_path_resolver,
                )
            })
            .fold(Ok(TokenStream::new()), |carry_result, item_result| {
                if let Ok(mut carry) = carry_result {
                    if let Ok(item) = item_result {
                        item.to_tokens(&mut carry);

                        Ok(carry)
                    } else {
                        item_result
                    }
                } else {
                    carry_result
                }
            })?;

        Ok(AchievedEntityResolver {
            definitions: tables,
            implement: quote! {
                #implements
            },
        })
    }
}

pub struct AchievedEntityResolver {
    pub definitions: Vec<TableDefinition>,
    pub implement: TokenStream,
}
