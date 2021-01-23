use crate::repository::{Repository, RepositoryInternal};
use crate::Entity;
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::mem::size_of;

struct RepositoryBox {
    storage: Vec<u8>,
    type_id: TypeId,
}

impl RepositoryBox {
    pub fn create<E: Entity + Clone>(mut v: Repository<E>) -> Self {
        let type_id = TypeId::of::<Repository<E>>();
        let size = size_of::<Repository<E>>();
        let ptr: *mut Repository<E> = &mut v;
        let storage = unsafe { Vec::from_raw_parts(ptr as *mut u8, size, size) };

        RepositoryBox { storage, type_id }
    }

    pub fn as_ref<E: Entity + Clone>(&self) -> &Repository<E> {
        assert_eq!(TypeId::of::<Repository<E>>(), self.type_id);

        let head = self.storage.as_ptr() as *const Repository<E>;

        unsafe { head.as_ref().unwrap() }
    }
}

pub struct RepositoryContainer {
    repositories: UnsafeCell<HashMap<TypeId, RepositoryBox>>,
}

impl RepositoryContainer {
    pub fn get_repository<E: Entity + Clone>(&self) -> &Repository<E> {
        let type_id = TypeId::of::<Repository<E>>();
        let repositories = unsafe { self.repositories.get().as_mut().unwrap() };
        repositories
            .entry(type_id)
            .or_insert_with(|| RepositoryBox::create(Repository::<E>::create()));

        repositories.get(&type_id).unwrap().as_ref()
    }
}

impl Default for RepositoryContainer {
    fn default() -> Self {
        RepositoryContainer {
            repositories: UnsafeCell::new(Default::default()),
        }
    }
}
