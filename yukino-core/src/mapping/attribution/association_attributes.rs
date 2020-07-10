use yui::YuiAttribute;

/// DataStructure of AssociateColumn.
/// Only used on the owner side.
/// If field type is a List of associated Entity, Yukino will join a table to manage relationship.
#[derive(YuiAttribute, Clone)]
#[mod_path = "yukino::mapping"]
pub struct Association {
    /// Referenced field names, default is primary key of referenced entity.
    pub mapped_by: Option<Vec<String>>,
}

/// DataStructure of InverseAssociateColumn
/// Only used on the attachment side
#[derive(YuiAttribute, Clone)]
#[mod_path = "yukino::mapping"]
pub struct InverseAssociation {
    /// Owner side field name.
    pub inversed_by: String,
}
