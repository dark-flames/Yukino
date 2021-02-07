use crate::repository::Repository;
use crate::transaction::repository_container::RepositoryContainer;
use crate::{Entity, EntityProxy};
use crate::query::QueryBuilderFactory;

mod repository_container;

pub struct Transaction {
    repository_container: RepositoryContainer,
}

impl Transaction {
    pub fn get_repository<E: Entity + Clone>(&self) -> &Repository<E> {
        self.repository_container.get_repository()
    }

    pub fn create_entity<'t, E: Entity + Clone, P: EntityProxy<'t, E>>(
        &'t self,
        value: impl FnOnce() -> E,
    ) -> P {
        P::create_proxy(value(), self)
    }

    pub fn create_query_builder(&self) -> QueryBuilderFactory {
        QueryBuilderFactory::create(self)
    }
}
