use crate::query::ast::error::SyntaxErrorWithPos;
use crate::query::ast::ColumnIdent;
use crate::query::type_check::{TypeCheck, TypeChecker, TypeKind};
use iroha::ToTokens;

#[derive(ToTokens, Clone)]
#[Iroha(mod_path = "yukino::query::type_check")]
pub struct FieldInfo {
    pub type_kind: TypeKind,
    pub nullable: bool,
    pub ty: String,
}

impl TypeCheck for ColumnIdent {
    fn assert_type(
        &self,
        _type_kind: TypeKind,
        _type_checker: &mut TypeChecker,
    ) -> Result<(), SyntaxErrorWithPos> {
        unimplemented!()
    }

    fn infer_type(&self, _type_checker: &mut TypeChecker) -> Option<TypeKind> {
        unimplemented!()
    }
}
