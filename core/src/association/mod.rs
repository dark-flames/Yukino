use crate::types::DatabaseValue;
use crate::Entity;
use std::collections::HashMap;
use std::marker::PhantomData;

pub enum AssociatedEntity<'t, E>
where
    E: Entity<'t> + Clone,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(E::Proxy),
    _Marker(PhantomData<&'t E>),
}

pub struct AssociatedValue<'t, E>
where
    E: Entity<'t> + Clone,
{
    _entity: AssociatedEntity<'t, E>,
}
