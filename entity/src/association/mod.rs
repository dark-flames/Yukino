use crate::types::DatabaseValue;
use crate::{Entity, EntityProxy};
use serde::export::PhantomData;
use std::collections::HashMap;

pub enum AssociatedEntity<'r, T, E>
where
    E: Entity<'r> + Clone,
    T: EntityProxy<'r, E>,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(T),
    _Marker(PhantomData<&'r E>),
}

pub struct AssociatedValue<'r, T, E>
where
    E: Entity<'r> + Clone,
    T: EntityProxy<'r, E>,
{
    _entity: AssociatedEntity<'r, T, E>,
}
