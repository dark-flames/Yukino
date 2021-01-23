use crate::repository::Repository;
use crate::types::DatabaseValue;
use crate::{Entity, EntityProxy};
use serde::export::PhantomData;
use std::collections::HashMap;

pub enum AssociatedEntity<'r, P, E>
where
    E: Entity<'r> + Clone,
    P: EntityProxy<'r, E>,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(P),
    _Marker(PhantomData<&'r E>),
}

pub struct AssociatedValue<'r, P, E>
where
    E: Entity<'r> + Clone,
    P: EntityProxy<'r, E>,
{
    _entity: AssociatedEntity<'r, P, E>,
    _repository: &'r Repository<'r, P, E>,
}
