use crate::mapping::definition::DefinitionManager;
use crate::query::query_builder::SelectQueryBuilder;
use crate::query::SelectItem;

#[allow(dead_code)]
pub struct QueryBuilderInitializer(&'static DefinitionManager);

#[allow(dead_code)]
impl QueryBuilderInitializer {
    pub fn create(definition_manager: &'static DefinitionManager) -> Self {
        QueryBuilderInitializer(definition_manager)
    }

    pub fn select(&self, items: Vec<SelectItem>) -> SelectQueryBuilder {
        let mut result = SelectQueryBuilder::create(self.0);
        result.select(items);

        result
    }
}
