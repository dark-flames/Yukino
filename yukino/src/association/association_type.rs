use crate::association::maintainer::{Cascade, Maintainer};
use crate::association::fetch_strategy::{Auto, FetchStrategy};
use crate::entity::Entity;
use std::collections::HashMap;
use crate::mapping::DatabaseValue;

#[allow(dead_code)]
pub struct DirectlyAssociation {
    /// Remote column name mapped by local column name
    column_map: HashMap<&'static str, &'static str>
}

#[allow(dead_code)]
pub struct ViaMembershipTableAssociation {
    /// Name of membership table
    membership_table_name: &'static str,
    ///  Remote column name mapped by membership column name
    remote_column_map: HashMap<&'static str, &'static str>,
    /// Membership column name mapped by Local column name
    column_map: HashMap<&'static str, &'static str>
}

/// Type of inverse association
#[allow(dead_code)]
pub enum InverseAssociationType {
    /// Many to One or One to One
    Directly(DirectlyAssociation),
    /// One to Many or Many to Many
    ViaMembershipTable(ViaMembershipTableAssociation)
}


/// Create a associated field
/// Associated to entity M
/// If you want to specific mapped by field (default is primary keys), use Association attribute.
#[allow(dead_code)]
pub struct Association<M, UPDATE = Cascade, DELETE = Cascade, FETCH= Auto>
    where
        M: Entity,
        UPDATE: Maintainer,
        DELETE: Maintainer,
        FETCH: FetchStrategy
{
    /// Attachment side fields
    mapped_by: Vec<&'static str>,
    /// Query result of columns
    column_values: HashMap<&'static str, DatabaseValue>,
    /// Fetched entity
    fetched_value: Option<Box<M>>,
    /// Fetch strategy
    fetch_strategy: FETCH,
    /// On delete maintainer
    on_delete: DELETE,
    /// On update maintainer
    on_update: UPDATE
}

#[allow(dead_code)]
pub struct InverseAssociation<'a, M, UPDATE = Cascade, DELETE = Cascade, FETCH= Auto>
    where
        M: Entity,
        UPDATE: Maintainer,
        DELETE: Maintainer,
        FETCH: FetchStrategy
{
    association_type: InverseAssociationType,
    column_values: HashMap<&'static str, DatabaseValue>,
    fetched_value: Option<Box<M>>,
    fetch_strategy: &'a FETCH,
    on_delete: &'a DELETE,
    on_update: &'a UPDATE
}

#[allow(dead_code)]
pub struct AssociationCollection<'a, M, UPDATE = Cascade, DELETE = Cascade, FETCH= Auto>
    where
        M: Entity,
        UPDATE: Maintainer,
        DELETE: Maintainer,
        FETCH: FetchStrategy
{
    fetched_values: Vec<M>, // todo: use other collection
    membership_table: &'static str,
    fetch_strategy: &'a FETCH,
    on_delete: &'a DELETE,
    on_update: &'a UPDATE
}

#[allow(dead_code)]
pub struct InverseAssociationCollection<'a, M, UPDATE = Cascade, DELETE = Cascade, FETCH= Auto>
    where
        M: Entity,
        UPDATE: Maintainer,
        DELETE: Maintainer,
        FETCH: FetchStrategy
{
    fetched_values: Vec<M>, // todo: use other collection
    association_type: InverseAssociationType,
    column_values: HashMap<&'static str, DatabaseValue>,
    fetch_strategy: &'a FETCH,
    on_delete: &'a DELETE,
    on_update: &'a UPDATE
}