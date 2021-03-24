mod ident;
mod literal;
mod type_checker;
mod type_kind;

pub use ident::*;
pub use literal::*;
pub use type_checker::*;
pub use type_kind::*;

use crate::query::ast::error::SyntaxErrorWithPos;

pub trait TypeCheck {
    fn assert_type(
        &self,
        type_kind: TypeKind,
        type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos>;

    fn infer_type(&self, type_checker: &mut TypeChecker) -> Option<TypeKind>;
}
