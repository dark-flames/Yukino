use annotation_rs::Annotation;

/// Annotation of Association.
/// Used on owning side field.
/// Create a foreign key for this field that can be used to associate with other entities.The type of
/// the field specifies the attachment side entity. Yukino will generate a number of columns in
/// database to associate with the target object.
///
/// If the type of the field is a entity struct, the association type will be `one to one` or
/// `one to many`, it depends on whether the specified mapping fields are unique; but if the
/// type of the field is a collection type, the association type will be `many to one` or
///`many to many`. In addition, the association type can be inferred from the type of
/// `InverseAssociation` field, but if it conflicts with the type of the `Association` field, an
/// error will be thrown.
#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct Association {
    /// Field names of attachment side entity. Specifies how the object will be mapped, using the
    /// primary key of the target object by default.
    pub mapped_by: Option<Vec<String>>,
}

/// Annotation of InverseAssociation.
/// Used one attachment side field.
/// The other side of the field must be marked as Association. This field will not generate a Column
/// in the database. `InverseAssociation` field is optional, if you don't need to get the owning side
/// entity by the attachment side entity, then you don't need to add a `InverseAssociation` field.
#[derive(Annotation, Clone)]
#[mod_path = "yukino::annotations"]
pub struct InverseAssociation {
    /// Owning side field name.
    pub inversed_by: String,
}
