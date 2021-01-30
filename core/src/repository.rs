use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use crate::{Entity, EntityProxy, EntityUniqueID};
use rand::random;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

pub struct Repository<E>
where
    E: Entity + Clone,
{
    pool: RefCell<HashMap<EntityUniqueID, E>>,
}

impl<E: Entity + Clone> Repository<E> {
    fn insert_entity(&self, entity: E) -> EntityUniqueID {
        let mut pool = self.pool.borrow_mut();
        let id = self.generate_unique_id();

        pool.insert(id, entity);

        id
    }

    fn generate_unique_id(&self) -> EntityUniqueID {
        // todo: generate_by_primary_key
        let pool = self.pool.borrow();
        loop {
            let id = random();

            if !pool.contains_key(&id) {
                break id;
            }
        }
    }

    pub fn get_entity(&self, id: &EntityUniqueID) -> Option<E> {
        let pool = self.pool.borrow();
        pool.get(id).cloned()
    }

    pub fn drop_entity(&self, id: &EntityUniqueID) -> Option<E> {
        let mut pool = self.pool.borrow_mut();

        pool.remove(id)
    }

    pub fn commit<'t, P: EntityProxy<'t, E>>(&mut self, entity_proxy: P) {
        let _id = entity_proxy
            .unique_id()
            .unwrap_or_else(|| self.generate_unique_id());

        let entity = entity_proxy.inner();
        // todo: compare_value

        let _value = entity.to_database_values();
        // todo: commit to db
    }

    pub fn find(&self, _primary_key_values: &HashMap<String, DatabaseValue>)
            -> Option<E> {
        unimplemented!()
    }
}

pub(crate) trait RepositoryInternal<E>
where
    E: Entity + Clone,
    Self: Sized,
{
    fn deserialize_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<(E, EntityUniqueID), DataConvertError>;

    fn create() -> Self;
}

impl<E> RepositoryInternal<E> for Repository<E>
where
    E: Entity + Clone,
{
    fn deserialize_value(
        &self,
        values: &HashMap<String, DatabaseValue, RandomState>,
    ) -> Result<(E, EntityUniqueID), DataConvertError> {
        let entity = E::from_database_value(values)?;

        Ok((entity.clone(), self.insert_entity(entity)))
    }

    fn create() -> Repository<E> where {
        Repository {
            pool: RefCell::new(Default::default()),
        }
    }
}
