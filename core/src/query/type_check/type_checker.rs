use crate::query::ast::error::SyntaxError;
use crate::query::ast::ColumnIdent;
use crate::query::type_check::{FieldInfo, TypeKind};
use std::collections::HashMap;

pub struct TypeChecker {
    external_value_assertion: HashMap<String, TypeKind>,
}

#[allow(clippy::map_entry)]
impl TypeChecker {
    pub fn add_external_value_assertion(
        &mut self,
        ident: String,
        ty: TypeKind,
    ) -> Result<(), SyntaxError> {
        if self.external_value_assertion.contains_key(&ident) {
            Err(SyntaxError::ConflictValueAssertion(ident))
        } else {
            self.external_value_assertion.insert(ident, ty);
            Ok(())
        }
    }

    pub fn get_field_info(&self, _ident: ColumnIdent) -> FieldInfo {
        unimplemented!()
    }
}
