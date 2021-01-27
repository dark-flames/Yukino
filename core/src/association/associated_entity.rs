use crate::resolver::error::DataConvertError;
use crate::resolver::ValueConverter;
use crate::types::DatabaseValue;
use crate::Entity;
use iroha::ToTokens;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::marker::PhantomData;

pub enum AssociatedEntity<E>
where
    E: Entity + Clone,
{
    Unresolved(HashMap<String, DatabaseValue>),
    Resolved(E),
}

#[derive(ToTokens)]
pub struct AssociatedEntityValueConverter<E: Entity + Clone> {
    entity_name: String,
    field_name: String,
    column_map: HashMap<String, String>,
    _marker: PhantomData<E>,
}

impl<E: Entity + Clone> ValueConverter<AssociatedEntity<E>> for AssociatedEntityValueConverter<E> {
    fn to_field_value(
        &self,
        values: &HashMap<String, DatabaseValue>,
    ) -> Result<AssociatedEntity<E>, DataConvertError> {
        let value_map: HashMap<String, DatabaseValue> = values
            .iter()
            .filter_map(|(name, value)| {
                if self.column_map.contains_key(name.as_str()) {
                    Some((name.clone(), value.clone()))
                } else {
                    None
                }
            })
            .collect();

        if value_map.len() == self.column_map.len() {
            Ok(AssociatedEntity::Unresolved(value_map))
        } else {
            Err(DataConvertError::UnexpectedDatabaseValueType(
                self.entity_name.clone(),
                self.field_name.clone(),
            ))
        }
    }

    fn to_database_values_by_ref(
        &self,
        value: &AssociatedEntity<E>,
    ) -> Result<HashMap<String, DatabaseValue, RandomState>, DataConvertError> {
        match value {
            AssociatedEntity::Unresolved(map) => Ok(map.clone()),
            AssociatedEntity::Resolved(entity) => {
                let associated_result = entity.to_database_values()?;

                let reverse_map: HashMap<String, String> = self
                    .column_map
                    .iter()
                    .map(|(column, associated_column)| (associated_column.clone(), column.clone()))
                    .collect();

                Ok(associated_result
                    .into_iter()
                    .filter_map(|(column, value)| {
                        if let Some(current_column_name) = reverse_map.get(&column) {
                            Some((current_column_name.clone(), value))
                        } else {
                            None
                        }
                    })
                    .collect())
            }
        }
    }

    fn primary_column_values_by_ref(
        &self,
        _value: &AssociatedEntity<E>,
    ) -> Result<HashMap<String, DatabaseValue, RandomState>, DataConvertError> {
        Ok(HashMap::new())
    }
}
