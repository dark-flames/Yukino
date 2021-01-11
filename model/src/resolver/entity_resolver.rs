use crate::annotations::Entity;
use crate::resolver::error::ResolveError;
use crate::resolver::{EntityPath, FieldName, ResolverBox};
use heck::SnakeCase;
use proc_macro2::Ident;
use std::collections::HashMap;

#[derive(Clone)]
pub enum EntityResolveStatus {
    Achieved,
    Finished,
    Assemble,
    Unresolved,
}

impl EntityResolveStatus {
    pub fn is_finished(&self) -> bool {
        matches!(&self, EntityResolveStatus::Finished)
    }
}

#[allow(dead_code)]
pub struct EntityResolver {
    status: EntityResolveStatus,
    mod_path: &'static str,
    ident: Ident,
    field_count: usize,
    annotation: Entity,
    field_resolvers: HashMap<FieldName, ResolverBox>,
    primary_keys: Vec<String>,
}

impl EntityResolver {
    pub fn new(
        ident: Ident,
        mod_path: &'static str,
        field_count: usize,
        annotation: Option<Entity>,
    ) -> Self {
        EntityResolver {
            status: EntityResolveStatus::Unresolved,
            mod_path,
            ident: ident.clone(),
            field_count,
            annotation: annotation.unwrap_or(Entity {
                name: Some(ident.to_string().to_snake_case()),
                indexes: Some(HashMap::new()),
            }),
            field_resolvers: HashMap::new(),
            primary_keys: vec![],
        }
    }

    pub fn entity_name(&self) -> EntityPath {
        format!("{}::{}", &self.mod_path, &self.ident)
    }

    pub fn status(&self) -> EntityResolveStatus {
        self.status.clone()
    }

    pub fn get_field_resolver(&self, field: &str) -> Result<&ResolverBox, ResolveError> {
        self.field_resolvers.get(field).ok_or_else(|| {
            ResolveError::FieldResolverNotFound(self.entity_name(), field.to_string())
        })
    }

    pub fn assemble_field(
        &mut self,
        field: ResolverBox,
    ) -> Result<EntityResolveStatus, ResolveError> {
        if field.status().is_finished() {
            let mut primary_key_column_names = field.primary_key_column_names()?;

            self.primary_keys.append(&mut primary_key_column_names);

            self.field_resolvers.insert(field.field_path().1, field);

            self.status = if self.field_resolvers.len() == self.field_count {
                EntityResolveStatus::Finished
            } else {
                EntityResolveStatus::Assemble
            };

            Ok(self.status.clone())
        } else {
            let field_path = field.field_path();
            Err(ResolveError::UnfinishedFieldCanNotAssembleToEntity(
                field_path.0.clone(),
                field_path.1,
            ))
        }
    }
}
