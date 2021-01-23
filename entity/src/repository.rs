use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use crate::{Entity, EntityProxy, EntityUniqueID};
use rand::random;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Repository<'r, P, E>
where
    E: 'r + Entity<'r> + Clone,
    P: EntityProxy<'r, E>,
{
    pool: RefCell<HashMap<EntityUniqueID, E>>,
    _marker: PhantomData<&'r P>,
}

impl<'r, E: Entity<'r> + Clone, P: EntityProxy<'r, E>> Repository<'r, P, E> {
    fn insert_entity(&self, entity: E) {
        let mut pool = self.pool.borrow_mut();
        let id = self.generate_unique_id();

        pool.insert(id, entity);
    }
    pub fn get_entity(&self, id: &EntityUniqueID) -> Option<E> {
        let pool = self.pool.borrow();
        pool.get(id).cloned()
    }

    pub fn drop_entity(&self, id: &EntityUniqueID) -> Option<E> {
        let mut pool = self.pool.borrow_mut();

        pool.remove(id)
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

    pub fn create<F: FnOnce() -> E>(&'r self, entity: F) -> P {
        P::create_proxy(entity(), self)
    }

    pub fn commit(&mut self, entity_proxy: P) {
        let _id = entity_proxy
            .unique_id()
            .unwrap_or_else(|| self.generate_unique_id());

        let entity = entity_proxy.inner();
        // todo: compare_value

        let _value = entity.to_database_values();
        // todo: commit to db
    }
}

pub(crate) trait RepositoryInternal<'r, P, E>
where
    E: 'r + Entity<'r> + Clone,
    P: EntityProxy<'r, E>,
{
    fn deserialize_value(
        &'r self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<P, DataConvertError>;
}

impl<'r, E, P> RepositoryInternal<'r, P, E> for Repository<'r, P, E>
where
    E: 'r + Entity<'r> + Clone,
    P: EntityProxy<'r, E>,
{
    fn deserialize_value(
        &'r self,
        values: &HashMap<String, DatabaseValue, RandomState>,
    ) -> Result<P, DataConvertError> {
        let entity = E::from_database_value(values)?;

        self.insert_entity(entity.clone());
        let mut proxy = P::create_proxy(entity, &self);

        proxy.set_unique_id(self.generate_unique_id());

        Ok(proxy)
    }
}
