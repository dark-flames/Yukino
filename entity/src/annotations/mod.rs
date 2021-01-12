mod association;
mod entity;
mod field;

use annotation_rs::AnnotationStructure;
pub use association::*;
pub use entity::*;
pub use field::*;
use quote::ToTokens;
use syn::{Attribute, Error};

pub enum FieldAnnotation {
    ID(ID),
    Ignore(Ignore),
    Field(Field),
    Association(Association),
    InverseAssociation(InverseAssociation),
}

impl FieldAnnotation {
    pub fn from_attr(attr: &Attribute) -> Result<Self, Error> {
        if attr.path == ID::get_path() {
            Ok(FieldAnnotation::ID(ID))
        } else if attr.path == Field::get_path() {
            Ok(FieldAnnotation::Field(Field::from_meta(
                &attr.parse_meta()?,
            )?))
        } else if attr.path == Association::get_path() {
            Ok(FieldAnnotation::Association(Association::from_meta(
                &attr.parse_meta()?,
            )?))
        } else if attr.path == InverseAssociation::get_path() {
            Ok(FieldAnnotation::InverseAssociation(
                InverseAssociation::from_meta(&attr.parse_meta()?)?,
            ))
        } else {
            Err(Error::new_spanned(
                attr,
                format!(
                    "Unexpected attribute: {}",
                    attr.path.to_token_stream().to_string()
                ),
            ))
        }
    }
}
