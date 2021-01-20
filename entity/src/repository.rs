use crate::{Entity, EntityUniqueID};
use rand::random;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

pub struct EntityPool<E: Entity> {
    entities: HashMap<EntityUniqueID, E>,
}

impl<E: Entity> EntityPool<E> {
    pub fn get_entity(&self, id: &EntityUniqueID) -> Option<&E> {
        self.entities.get(id)
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

pub struct Repository<E: Entity> {
    pool: RefCell<EntityPool<E>>,
}

impl<E: Entity> Repository<E> {
    pub fn pool(&self) -> Ref<EntityPool<E>> {
        self.pool.borrow()
    }

    pub fn pool_mut(&self) -> RefMut<EntityPool<E>> {
        self.pool.borrow_mut()
    }
}
