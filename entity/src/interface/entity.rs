use crate::definitions::TableDefinition;
use crate::repository::Repository;
use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use std::collections::HashMap;

pub type EntityUniqueID = usize;

pub trait Entity {
    fn from_database_value(
        result: &HashMap<String, DatabaseValue>,
    ) -> Result<Box<Self>, DataConvertError>;

    fn to_database_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;

    fn get_definitions() -> Vec<TableDefinition>;

    fn primary_key_values(&self) -> Result<HashMap<String, DatabaseValue>, DataConvertError>;
}

pub trait EntityProxy<'r, E: 'r + Entity> {
    fn unique_id(&self) -> Option<EntityUniqueID>;

    fn set_unique_id(&mut self, unique_id: EntityUniqueID);

    fn get_repository(&self) -> &'r Repository<E>;

    fn create(inner: E, repo: &'r Repository<E>) -> Self
    where
        Self: Sized;

    fn unwrap(self) -> E;

    fn drop(&mut self) {
        if let Some(id) = self.unique_id() {
            self.get_repository().pool_mut().drop_entity(&id);
        }
    }
}

#[macro_export]
macro_rules! impl_entity_proxy {
    ($entity: ident) => {
        pub struct $entity<'r> {
            inner: concat_idents!($entity, Inner),
            unique_id: Option<yukino::EntityUniqueID>,
            repository: &'r yukino::repository::Repository<concat_idents!($entity, Inner)>,
        }

        impl<'r> yukino::EntityProxy<'r, concat_idents!($entity, Inner)> for $entity<'r> {
            fn unique_id(&self) -> Option<yukino::EntityUniqueID> {
                self.unique_id.clone()
            }

            fn set_unique_id(&mut self, unique_id: yukino::EntityUniqueID) {
                self.unique_id = Some(unique_id);
            }

            fn get_repository(
                &self,
            ) -> &'r yukino::repository::Repository<concat_idents!($entity, Inner)> {
                self.repository
            }

            fn create(
                inner: concat_idents!($entity, Inner),
                repository: &'r yukino ::repository::Repository<concat_idents!($entity, Inner)>,
            ) -> Self
            where
                Self: Sized,
            {
                $entity {
                    inner,
                    unique_id: None,
                    repository,
                }
            }

            fn unwrap(self) -> concat_idents!($entity, Inner) {
                self.inner
            }
        }
    };
}
