use crate::types::DatabaseValue;
use crate::Entity;
use std::collections::HashMap;

pub enum AssociatedEntity<E>
where
    E: Entity + Clone,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(E),
}
