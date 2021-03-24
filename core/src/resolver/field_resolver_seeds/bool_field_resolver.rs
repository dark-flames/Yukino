use iroha::ToTokens;
use crate::types::{FieldType, FieldTypeBox};
use syn::{Type, parse2};
use proc_macro2::TokenStream;
use crate::query::type_check::{TypeChecker, TypeKind};
use crate::query::ast::Literal;
use std::str::FromStr;
use crate::query::ast::error::SyntaxErrorWithPos;

#[derive(ToTokens)]
pub struct BoolFieldType {
    nullable: bool
}

impl FieldType for BoolFieldType {
    fn name(&self) -> &'static str where
        Self: Sized {
        "bool"
    }

    fn get_value_type(&self) -> Type {
        parse2(TokenStream::from_str("bool").unwrap()).unwrap()
    }

    fn type_kind(&self) -> TypeKind {
        TypeKind::Boolean
    }

    fn nullable(&self) -> bool {
        self.nullable
    }

    fn wrap_lit(&self, _lit: Literal, _type_checker: &mut TypeChecker) -> Result<FieldTypeBox, SyntaxErrorWithPos> {
        unimplemented!()
    }
}