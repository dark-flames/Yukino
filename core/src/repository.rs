use crate::resolver::error::DataConvertError;
use crate::types::DatabaseValue;
use crate::{Entity, EntityProxy, EntityUniqueID};
use rand::random;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Repository<'t, E>
where
    E: 't + Entity<'t> + Clone,
{
    pool: RefCell<HashMap<EntityUniqueID, E>>,
    _marker: PhantomData<&'t E>,
}

impl<'t, E: Entity<'t> + Clone> Repository<'t, E> {
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

    pub fn create<F: FnOnce() -> E>(&'t self, entity: F) -> E::Proxy {
        E::Proxy::create_proxy(entity(), self)
    }

    pub fn commit(&mut self, entity_proxy: E::Proxy) {
        let _id = entity_proxy
            .unique_id()
            .unwrap_or_else(|| self.generate_unique_id());

        let entity = entity_proxy.inner();
        // todo: compare_value

        let _value = entity.to_database_values();
        // todo: commit to db
    }
}

pub(crate) trait RepositoryInternal<'t, E>
where
    E: 't + Entity<'t> + Clone,
{
    fn deserialize_value(
        &'t self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<E::Proxy, DataConvertError>;
}

impl<'r, E> RepositoryInternal<'r, E> for Repository<'r, E>
where
    E: 'r + Entity<'r> + Clone,
{
    fn deserialize_value(
        &'r self,
        values: &HashMap<String, DatabaseValue, RandomState>,
    ) -> Result<E::Proxy, DataConvertError> {
        let entity = E::from_database_value(values)?;

        self.insert_entity(entity.clone());
        let mut proxy = E::Proxy::create_proxy(entity, &self);

        proxy.set_unique_id(self.generate_unique_id());

        Ok(proxy)
    }
}
