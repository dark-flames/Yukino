mod enums;
mod table_attributes;
mod column_attributes;
mod association_attributes;

pub use enums::*;
pub use table_attributes::*;
pub use column_attributes::*;
pub use association_attributes::*;
use syn::{Attribute, Error};
use syn::export::ToTokens;
use yui::AttributeStructure;

pub enum FieldAttribute {
    Id(Id),
    Ignore(Ignore),
    Column(Column),
    Association(Association),
    InverseAssociation(InverseAssociation)
}

impl FieldAttribute {
    pub fn from_attr(attr: &Attribute) -> Result<Self, Error> {
        if attr.path == Id::get_path() {
            Ok(FieldAttribute::Id(Id{}))
        } else if attr.path == Column::get_path() {
            Ok(FieldAttribute::Column(
                Column::from_meta(&attr.parse_meta()?)?
            ))
        } else if attr.path == Association::get_path() {
            Ok(FieldAttribute::Association(
                Association::from_meta(&attr.parse_meta()?)?
            ))
        } else if attr.path == InverseAssociation::get_path() {
            Ok(FieldAttribute::InverseAssociation(
                InverseAssociation::from_meta(&attr.parse_meta()?)?
            ))
        } else {
            Err(Error::new_spanned(attr, format!(
                "Unexpected attribute: {}",
                attr.path.to_token_stream().to_string()
            )))
        }
    }
}