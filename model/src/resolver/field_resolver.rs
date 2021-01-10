use crate::resolver::error::ResolveError;
use crate::resolver::{EntityPath, EntityResolver, FieldPath};

pub enum FieldResolverStatus {
    Seed,
    WaitingForFields(Vec<FieldPath>),
    WaitingForEntity(EntityPath),
    WaitingAssemble,
    Finished,
}

impl FieldResolverStatus {
    pub fn is_waiting_for_entity(&self, entity_path: &str) -> bool {
        matches!(self, FieldResolverStatus::WaitingForEntity(path) if path == entity_path)
    }
}

pub trait FieldResolver {
    fn status(&self) -> FieldResolverStatus;

    fn field_path(&self) -> FieldPath;

    fn resolve_by_waiting_entity(
        &mut self,
        resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError>;
}
