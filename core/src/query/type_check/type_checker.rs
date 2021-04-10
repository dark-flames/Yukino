use crate::definitions::FieldDefinition;
use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{JoinClause, Locatable};
use crate::types::{ExprWrapper, TypeInfo, TypeResolver};
use std::collections::HashMap;

pub struct TypeChecker<F>
where
    F: Fn(&str, &str) -> Option<FieldDefinition>,
{
    external_value_assertion: HashMap<String, String>,
    result_type: HashMap<String, String>,
    generated_join: HashMap<String, JoinClause>,
    resolvers: HashMap<String, Box<dyn TypeResolver>>,
    alias: HashMap<String, String>,
    definition_getter: F,
}

#[allow(clippy::map_entry)]
impl<F> TypeChecker<F>
where
    F: Fn(&str, &str) -> Option<FieldDefinition>,
{
    pub fn new(
        resolvers: Vec<Box<dyn TypeResolver>>,
        alias: HashMap<String, String>,
        definition_getter: F,
    ) -> Self {
        TypeChecker {
            external_value_assertion: Default::default(),
            result_type: Default::default(),
            generated_join: Default::default(),
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
        ty: String,
    ) -> Result<(), SyntaxError> {
        if self.external_value_assertion.contains_key(&ident) {
            Err(SyntaxError::ConflictValueAssertion(ident))
        } else {
            self.external_value_assertion.insert(ident, ty);
            Ok(())
        }
    }

    pub fn add_join_clause(&mut self, join: JoinClause) -> Result<(), SyntaxError> {
        let alias = match &join {
            JoinClause::JoinOn(join_on) => join_on.table.name.clone(),
            JoinClause::NaturalJoin(natural_join) => natural_join.table.name.clone(),
            JoinClause::CrossJoin(cross_join) => cross_join.table.name.clone(),
        };

        if self.generated_join.contains_key(&alias) {
            Err(SyntaxError::ConflictAlias(alias))
        } else {
            self.generated_join.insert(alias, join);

            Ok(())
        }
    }

    pub fn add_result_ty(&mut self, index: String, ty: String) -> Result<(), SyntaxError> {
        if self.generated_join.contains_key(&index) {
            Err(SyntaxError::ConflictResultIndex(index))
        } else {
            self.result_type.insert(index, ty);

            Ok(())
        }
    }

    pub fn get_resolver(&self, name: &str) -> Option<&dyn TypeResolver> {
        self.resolvers.get(name).map(|boxed| boxed.as_ref())
    }

    pub fn get_table_name(&self, alias: &str) -> Option<&str> {
        self.alias.get(alias).map(|string| string.as_str())
    }

    pub fn get_field_definition(&self, entity: &str, field: &str) -> Option<FieldDefinition> {
        (self.definition_getter)(entity, field)
    }
}

pub trait TypeInfer: Locatable {
    fn try_wrap<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
    ) -> Result<Option<ExprWrapper>, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>;

    fn wrap_with_ty<F>(
        &self,
        ty_checker: &mut TypeChecker<F>,
        type_info: TypeInfo,
    ) -> Result<ExprWrapper, SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>,
    {
        self.try_wrap(ty_checker)?.ok_or_else(|| {
            self.location()
                .error(SyntaxError::CannotBeWrappedInto(type_info.field_type))
        })
    }
}

pub trait TypeCheck: Locatable {
    fn check_type<F>(&mut self, ty_checker: &mut TypeChecker<F>) -> Result<(), SyntaxErrorWithPos>
    where
        F: Fn(&str, &str) -> Option<FieldDefinition>;
}
