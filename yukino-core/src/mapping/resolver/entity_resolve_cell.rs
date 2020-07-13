use super::error::{ResolveError, UnresolvedError};
use super::field_resolve_cell::FieldResolveCell;
use crate::mapping::definition::{
    ColumnDefinition, ForeignKeyDefinition, IndexDefinition, TableDefinition,
};
use crate::mapping::{DatabaseType, Table};
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum EntityResolveStatus {
    Achieved,
    Finished,
    Assemble,
    Unresolved,
}

pub struct EntityResolveCell {
    status: EntityResolveStatus,
    mod_path: &'static str,
    ident: Ident,
    field_count: usize,
    name: String,
    indexes: Vec<IndexDefinition>,
    primary_keys: Vec<String>,
    fields: HashMap<String, Box<dyn FieldResolveCell>>,
    table_definitions: Option<Vec<TableDefinition>>,
}

#[allow(dead_code)]
impl<'a> EntityResolveCell {
    pub fn get_status(&self) -> EntityResolveStatus {
        self.status.clone()
    }

    pub fn new(
        ident: &Ident,
        mod_path: &'static str,
        attr: &Option<Table>,
        field_count: usize,
    ) -> Result<Self, ResolveError> {
        let indexes = if let Some(table_attr) = attr {
            let default = HashMap::new();
            table_attr
                .indexes
                .as_ref()
                .unwrap_or(&default)
                .iter()
                .map(|(name, index)| IndexDefinition::from_attr(name, index))
                .collect()
        } else {
            Vec::new()
        };
        let ident_name = ident.to_string().to_snake_case();
        let name = if let Some(table_attr) = attr {
            table_attr.name.clone().unwrap_or(ident_name)
        } else {
            ident_name
        };

        Ok(EntityResolveCell {
            status: EntityResolveStatus::Assemble,
            ident: ident.clone(),
            mod_path,
            field_count,
            name,
            indexes,
            primary_keys: Vec::new(),
            fields: HashMap::new(),
            table_definitions: None,
        })
    }

    pub fn entity_name(&self) -> String {
        format!("{}::{}", &self.mod_path, &self.ident)
    }

    pub fn get_field(&self, name: &str) -> Option<&dyn FieldResolveCell> {
        self.fields.get(name).map(|field| field.as_ref())
    }

    pub fn assemble_column(&mut self, cell: Box<dyn FieldResolveCell>) -> EntityResolveStatus {
        if cell.is_primary_key().unwrap() {
            let mut names = cell.column_names().unwrap();
            self.primary_keys.append(&mut names)
        }

        self.fields.insert(cell.field_name().unwrap(), cell);

        self.status = if self.fields.len() == self.field_count {
            EntityResolveStatus::Finished
        } else {
            EntityResolveStatus::Assemble
        };

        self.status.clone()
    }

    pub fn get_primary_keys(&self) -> Result<&Vec<String>, UnresolvedError> {
        match self.get_status() {
            EntityResolveStatus::Finished => Ok(&self.primary_keys),
            _ => Err(UnresolvedError::new(&self.ident)),
        }
    }

    pub fn achieve(&mut self) -> Result<(), UnresolvedError> {
        if self.status != EntityResolveStatus::Finished {
            return Err(UnresolvedError::new(&self.ident.to_string()));
        };

        let mut columns: Vec<ColumnDefinition> = Vec::new();
        let mut tables: Vec<TableDefinition> = Vec::new();
        let mut foreign_keys: Vec<ForeignKeyDefinition> = Vec::new();
        if self.primary_keys.is_empty() {
            let auto_primary_keys = ColumnDefinition {
                name: format!("__{}_id", self.ident.to_string().to_snake_case()),
                column_type: DatabaseType::String,
                unique: true,
                auto_increase: false,
                is_primary_key: true,
            };
            self.primary_keys.push(auto_primary_keys.name.clone());
            columns.push(auto_primary_keys);
        }

        for (_, cell) in self.fields.iter() {
            let mut cell_columns = cell.get_column_definitions()?;
            let mut cell_tables = cell.get_joined_table_definitions()?;
            let mut cell_foreign_keys = cell.get_foreigner_keys()?;
            columns.append(&mut cell_columns);
            tables.append(&mut cell_tables);
            foreign_keys.append(&mut cell_foreign_keys);
        }

        tables.push(TableDefinition {
            name: self.name.clone(),
            indexes: self.indexes.clone(),
            columns,
            foreign_keys,
        });

        self.status = EntityResolveStatus::Achieved;

        self.table_definitions = Some(tables);

        Ok(())
    }

    pub fn get_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        self.table_definitions
            .as_ref()
            .cloned()
            .ok_or_else(|| UnresolvedError::new(&self.ident))
    }

    pub fn get_implement_token_stream(&self) -> Result<TokenStream, UnresolvedError> {
        let ident = TokenStream::from_str(self.entity_name().as_str())
            .map_err(|_| UnresolvedError::new(&self.ident))?;

        let definitions = self.get_definitions()?;

        let converters = self
            .fields
            .values()
            .map(|cell| cell.get_data_converter_token_stream())
            .collect::<Result<Vec<TokenStream>, UnresolvedError>>()?;

        let temp_values = self
            .fields
            .values()
            .map(|cell| {
                let method = match cell.get_data_converter_getter_ident() {
                    Ok(ident) => ident,
                    Err(e) => return Err(e),
                };

                let field_ident = match cell.field_name() {
                    Ok(name) => format_ident!("{}", name),
                    Err(e) => return Err(e),
                };

                Ok(quote::quote! {
                    let #field_ident = Self::#method().to_value(result)?
                })
            })
            .collect::<Result<Vec<TokenStream>, UnresolvedError>>()?;

        let fields = self
            .fields
            .values()
            .map(|cell| match cell.field_name() {
                Ok(name) => Ok(format_ident!("{}", name)),
                Err(e) => return Err(e),
            })
            .collect::<Result<Vec<Ident>, UnresolvedError>>()?;

        let inserts = self
            .fields
            .values()
            .map(|cell| {
                let method = match cell.get_data_converter_getter_ident() {
                    Ok(ident) => ident,
                    Err(e) => return Err(e),
                };

                let field_ident = match cell.field_name() {
                    Ok(name) => format_ident!("{}", name),
                    Err(e) => return Err(e),
                };

                Ok(quote::quote! {
                    map.extend(Self::#method().to_database_value(&self.#field_ident)?)
                })
            })
            .collect::<Result<Vec<TokenStream>, UnresolvedError>>()?;

        Ok(quote! {
            impl #ident {
                #(#converters)*
            }

            impl yukino::Entity for #ident {
                fn from_database_value(
                    result: &std::collections::HashMap<String, yukino::mapping::DatabaseValue>
                ) -> Result<Box<Self>, yukino::ParseError> {
                    use yukino::mapping::resolver::ValueConverter;

                    #(#temp_values;)*

                    Ok(Box::new(
                        #ident {
                            #(#fields),*
                        }
                    ))
                }

                fn to_database_value(&self)
                    -> Result<std::collections::HashMap<String, yukino::mapping::DatabaseValue>, yukino::ParseError> {
                    let mut map = std::collections::HashMap::new();
                    use yukino::mapping::resolver::ValueConverter;

                    #(#inserts;)*

                    Ok(map)
                }

                fn get_definitions(&self) -> Vec<yukino::mapping::definition::TableDefinition> {
                    vec![
                        #(#definitions),*
                    ]
                }
            }
        })
    }
}
