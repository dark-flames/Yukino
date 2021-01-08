use annotation_rs::{Annotation, AnnotationEnumValue};

/// Annotation of Association.
/// Used on owner side field.
/// Create a foreign key for this field that can be used to associate with other models.The type of
/// the field specifies the attachment side model. Yukino will generate a number of columns in
/// database to associate with the target object.
///
/// If the type of the field is a model struct, the association type will be `one to one` or
/// `one to many`, it depends on whether the specified mapping fields are unique; but if the
/// type of the field is a collection type, the association type will be `many to one` or
///`many to many`. In addition, the association type can be inferred from the type of
/// `InverseAssociation` field, but if it conflicts with the type of the `Association` field, an
/// error will be thrown.
#[derive(Annotation, Clone)]
pub struct Association {
    /// Field names of attachment side model. Specifies how the object will be mapped, using the
    /// primary key of the target object by default.
    pub mapped_by: Option<Vec<String>>,

    /// Association reference action on updating, default to be `Cascade`
    #[field(enum_value = true, default = "cascade")]
    pub update_action: ReferenceAction,

    /// Association reference action on deleting, default to be `Cascade`
    #[field(enum_value = true, default = "cascade")]
    pub delete_action: ReferenceAction,
}

/// Annotation of InverseAssociation.
/// Used one attachment side field.
/// The other side of the field must be marked as Association. This field will not generate a Column
/// in the database. `InverseAssociation` field is optional, if you don't need to get the owner side
/// model by the attachment side model, then you don't need to add a `InverseAssociation` field.
#[derive(Annotation, Clone)]
pub struct InverseAssociation {
    /// Owner side field name.
    pub inversed_by: String,

    /// Association reference action on updating, default to be `Cascade`
    #[field(enum_value = true, default = "cascade")]
    pub update_action: ReferenceAction,

    /// Association reference action on deleting, default to be `Cascade`
    #[field(enum_value = true, default = "cascade")]
    pub delete_action: ReferenceAction,
}

/// Association reference action type
/// The implementation may different depending on the platform and configuration, may be implemented
/// by framework trigger, or the functionality of the database platform.
#[derive(AnnotationEnumValue, Clone, Debug)]
pub enum ReferenceAction {
    /// Do nothing
    NoAction,
    /// Rejects the delete or update operation for the other side table
    Restrict,
    /// Delete or update the object from the table and set the field in the other side to None. Only
    /// can be used if the type of the field is Option or collection
    SetNull,
    /// Delete or update the object from the table and automatically delete or update the
    /// matching field in the other side table
    Cascade,
}
