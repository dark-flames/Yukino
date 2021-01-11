use crate::resolver::error::ResolveError;
use crate::resolver::{EntityPath, EntityResolver, FieldPath, ResolverBox};

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

    pub fn is_finished(&self) -> bool {
        matches!(self, FieldResolverStatus::Finished)
    }
}

pub trait FieldResolver {
    fn status(&self) -> FieldResolverStatus;

    fn field_path(&self) -> FieldPath;

    fn resolve_by_waiting_entity(
        &mut self,
        resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError>;

    fn resolve_by_waiting_fields(
        &mut self,
        resolvers: Vec<&ResolverBox>,
    ) -> Result<FieldResolverStatus, ResolveError>;
}
