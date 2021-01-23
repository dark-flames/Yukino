use crate::repository::Repository;
use crate::transaction::repository_container::RepositoryContainer;
use crate::Entity;

mod repository_container;

pub struct Transaction {
    repository_container: RepositoryContainer,
}

impl Transaction {
    pub fn get_repository<E: Entity + Clone>(&self) -> &Repository<E> {
        self.repository_container.get_repository()
    }
}
