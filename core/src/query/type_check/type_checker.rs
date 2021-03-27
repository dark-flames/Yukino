use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::type_check::TypeKind;
use crate::types::{ExprWrapper, TypeResolver};
use std::collections::HashMap;

pub struct TypeChecker<F>
where
    F: Fn(&str, &str) -> FieldDefinition,
{
    external_value_assertion: HashMap<String, TypeKind>,
    resolvers: HashMap<String, Box<dyn TypeResolver>>,
    alias: HashMap<String, String>,
    definition_getter: F,
}

#[allow(clippy::map_entry)]
impl<F> TypeChecker<F>
where
    F: Fn(&str, &str) -> FieldDefinition,
{
    pub fn new(
        resolvers: Vec<Box<dyn TypeResolver>>,
        alias: HashMap<String, String>,
        definition_getter: F,
    ) -> Self {
        TypeChecker {
            external_value_assertion: Default::default(),
            resolvers: resolvers
                .into_iter()
                .map(|resolver| (resolver.name(), resolver))
                .collect(),
            alias,
            definition_getter,
        }
    }

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

    pub fn get_resolver(&self, name: &str) -> Option<&dyn TypeResolver> {
        self.resolvers.get(name).map(|boxed| boxed.as_ref())
    }

    pub fn get_table_name(&self, alias: &str) -> Option<&str> {
        self.alias.get(alias).map(|string| string.as_str())
    }

    pub fn get_field_definition(&self, entity: &str, field: &str) -> FieldDefinition {
        (self.definition_getter)(entity, field)
    }
}

pub trait TypeCheck {
    fn warp<F>(&self, ty_checker: &mut TypeChecker<F>) -> Result<ExprWrapper, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> FieldDefinition;
}
