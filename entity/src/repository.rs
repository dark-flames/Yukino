use crate::{Entity, EntityProxy, EntityUniqueID};
use rand::random;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct Repository<'r, P, E>
where
    E: 'r + Entity + Clone,
    P: EntityProxy<'r, E>,
{
    pool: RefCell<HashMap<EntityUniqueID, E>>,
    _marker: PhantomData<&'r P>,
}

impl<'r, E: 'r + Entity + Clone, P: EntityProxy<'r, E>> Repository<'r, P, E> {
    pub fn get_entity(&self, id: &EntityUniqueID) -> Option<E> {
        let pool = self.pool.borrow();
        pool.get(id).cloned()
    }

    pub fn drop_entity(&self, id: &EntityUniqueID) -> Option<E> {
        let mut pool = self.pool.borrow_mut();

        pool.remove(id)
    }

    pub fn generate_unique_id(&self) -> EntityUniqueID {
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
}
