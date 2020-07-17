use crate::mapping::DefinitionManager;
use crate::query::QueryBuilderInitializer;

#[allow(dead_code)]
pub struct EntityManger {
    definition_manager: &'static DefinitionManager,
}
#[allow(dead_code)]
impl EntityManger {
    pub fn create(definition_manager: &'static DefinitionManager) -> Self {
        EntityManger { definition_manager }
    }

    pub fn create_query_builder(&self) -> QueryBuilderInitializer {
        QueryBuilderInitializer::create(self.definition_manager)
    }
}
