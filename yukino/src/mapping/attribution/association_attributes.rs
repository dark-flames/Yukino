use yui::YuiAttribute;
use super::enums::{ReferenceAction, FetchMode};

/// DataStructure of AssociateColumn.
/// Only used on the owner side.
/// If field type is a List of associated Entity, Yukino will join a table to manage relationship.
#[derive(YuiAttribute, Clone)]
pub struct AssociateColumn {
    /// Referenced field name, default is primary key of referenced entity.
    pub mapped_by: Option<String>,
    /// On update action.
    #[attribute_field(enum_value=true, default="set_null")]
    pub on_update: ReferenceAction,
    /// On delete action.
    #[attribute_field(enum_value=true, default="set_null")]
    pub on_delete: ReferenceAction,
    /// Fetch mode.
    #[attribute_field(enum_value=true, default="auto")]
    pub fetch: FetchMode,
}

/// DataStructure of InverseAssociateColumn
/// Only used on the attachment side
#[derive(YuiAttribute, Clone)]
pub struct InverseAssociateColumn {
    /// Owner side field name.
    pub inversed_by: String,
    /// On update action.
    #[attribute_field(enum_value=true, default="set_null")]
    pub on_update: ReferenceAction,
    /// On delete action.
    #[attribute_field(enum_value=true, default="set_null")]
    pub on_delete: ReferenceAction,
    /// Fetch mode.
    #[attribute_field(enum_value=true, default="auto")]
    pub fetch: FetchMode
}