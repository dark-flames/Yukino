use crate::{Entity, EntityProxy, EntityUniqueID};
use rand::random;
use std::borrow::Cow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct EntityPool<E: Entity + Clone> {
    entities: HashMap<EntityUniqueID, E>,
}

impl<E: Entity + Clone> EntityPool<E> {
    pub fn get_entity(&self, id: &EntityUniqueID) -> Option<Cow<E>> {
        self.entities.get(id).map(Cow::Borrowed)
    }

    pub fn drop_entity(&mut self, id: &EntityUniqueID) -> Option<E> {
        self.entities.remove(id)
    }

    pub fn generate_unique_id(&self) -> EntityUniqueID {
        loop {
            let id = random();

            if !self.entities.contains_key(&id) {
                break id;
            }
        }
    }
}

pub struct Repository<'r, P, E>
where
    E: 'r + Entity + Clone,
    P: EntityProxy<'r, E>,
{
    pool: RefCell<EntityPool<E>>,
    _marker: PhantomData<&'r P>,
}

impl<'r, E: 'r + Entity + Clone, P: EntityProxy<'r, E>> Repository<'r, P, E> {
    pub fn pool(&self) -> Ref<EntityPool<E>> {
        self.pool.borrow()
    }

    pub fn pool_mut(&self) -> RefMut<EntityPool<E>> {
        self.pool.borrow_mut()
    }

    pub fn create<F: FnOnce() -> E>(&'r self, entity: F) -> P {
        P::create_proxy(Cow::Owned(entity()), self)
    }
}
