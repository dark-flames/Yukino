use crate::annotations::Entity;
use crate::resolver::EntityPath;
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

#[allow(dead_code)]
pub struct EntityResolver {
    status: EntityResolveStatus,
    mod_path: &'static str,
    ident: Ident,
    field_count: usize,
    annotation: Entity,
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
        }
    }

    pub fn entity_name(&self) -> EntityPath {
        format!("{}::{}", &self.mod_path, &self.ident)
    }

    pub fn status(&self) -> EntityResolveStatus {
        self.status.clone()
    }
}
