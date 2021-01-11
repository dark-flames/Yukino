use crate::annotations::FieldAnnotation;
use crate::resolver::error::ResolveError;
use crate::resolver::{EntityPath, EntityResolver, FieldPath, FieldResolverBox};
use proc_macro2::Ident;
use syn::Type;

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
    fn breed(
        &self,
        entity_path: EntityPath,
        ident: &Ident,
        annotations: &[FieldAnnotation],
        field_type: &Type,
    ) -> Result<FieldResolverBox, ResolveError>;

    fn match_field(&self, annotations: &[FieldAnnotation], field_type: &Type) -> bool;

    fn status(&self) -> FieldResolverStatus;

    fn field_path(&self) -> FieldPath;

    fn entity_path(&self) -> EntityPath;

    fn resolve_by_waiting_entity(
        &mut self,
        resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError>;

    fn resolve_by_waiting_fields(
        &mut self,
        resolvers: Vec<&FieldResolverBox>,
    ) -> Result<FieldResolverStatus, ResolveError>;

    fn assemble(
        &mut self,
        entity_resolver: &EntityResolver,
    ) -> Result<FieldResolverStatus, ResolveError>;

    fn column_names(&self) -> Result<Vec<String>, ResolveError>;

    fn primary_key_column_names(&self) -> Result<Vec<String>, ResolveError> {
        if self.status().is_finished() {
            Ok(vec![])
        } else {
            let field_path = self.field_path();
            Err(ResolveError::FieldResolverIsNotFinished(
                field_path.0.clone(),
                field_path.1,
            ))
        }
    }
}
