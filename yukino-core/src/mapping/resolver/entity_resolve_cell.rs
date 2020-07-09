use std::str::FromStr;
use std::collections::HashMap;
use heck::SnakeCase;
use quote::{quote, format_ident};
use proc_macro2::{Ident, TokenStream};
use crate::mapping::{Table, DatabaseType};
use crate::mapping::definition::{ColumnDefinition, IndexDefinition, TableDefinition, ForeignKeyDefinition};
use super::field_resolve_cell::FieldResolveCell;
use super::error::{UnresolvedError, ResolveError};



#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum EntityResolveStatus {
    Achieved,
    Finished,
    Assemble,
    Unresolved
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
    table_definitions: Option<Vec<TableDefinition>>
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
        field_count: usize
    ) -> Result<Self, ResolveError> {
        let indexes = if let Some(table_attr) = attr {
            let default = HashMap::new();
            table_attr.indexes.as_ref().unwrap_or(&default).iter().map(
                |(name, index)| IndexDefinition::from_attr(name, index)
            ).collect()
        } else {
            Vec::new()
        };
        let ident_name = ident.to_string().to_snake_case();
        let name = if let Some(table_attr) = attr {
            table_attr.name.clone()
                .unwrap_or(ident_name)
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
            table_definitions: None
        })
    }

    pub fn entity_name(&self) -> String {
        format!("{}::{}", &self.mod_path, &self.ident)
    }

    pub fn get_field(&self, name: &str) -> Option<&dyn FieldResolveCell> {
        self.fields.get(name).map(
            |field| field.as_ref()
        )
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
            _ => Err(UnresolvedError::new(&self.ident))
        }
    }

    pub fn achieve(&mut self) -> Result<(), UnresolvedError> {
        if self.status != EntityResolveStatus::Finished {
            return Err(UnresolvedError::new(&self.ident.to_string()))
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
                is_primary_key: true
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
            foreign_keys
        });

        self.status = EntityResolveStatus::Achieved;

        self.table_definitions = Some(tables);

        Ok(())
    }

    pub fn get_definitions(&self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        self.table_definitions.as_ref().cloned().ok_or_else(
            || UnresolvedError::new(&self.ident)
        )
    }

    fn convert_to_database_value_token_stream(&self) -> Result<TokenStream, UnresolvedError> {
        let value_ident = format_ident!("database_value");
        let fields = self.fields.iter().map(
            |(_, cell)| cell.convert_to_database_value_token_stream(&value_ident)
        ).collect::<Result<Vec<TokenStream>, UnresolvedError>>()?;

        Ok(quote! {
            fn to_raw_value(&self) -> Result<std::collections::HashMap<String, yukino::mapping::DatabaseValue>, yukino::ParseError> {
                let mut #value_ident = std::collections::HashMap::new();

                #(#fields;)*

                Ok(#value_ident)
            }
        })
    }

    fn convert_to_value_token_stream(&self) -> Result<TokenStream, UnresolvedError> {
        let value_ident = format_ident!("result");
        let full_ident = TokenStream::from_str(
            self.entity_name().as_str()
        ).map_err(
            |_| UnresolvedError::new(&self.ident)
        )?;

        let fields = self.fields.iter().map(
            |(field_name, cell)| {
                cell.convert_to_value_token_stream(
                    &value_ident
                ).map(|value| {
                    let field_ident = format_ident!("{}", field_name);
                    quote::quote! {
                        let #field_ident = #value
                    }
                })
            }
        ).collect::<Result<Vec<TokenStream>, UnresolvedError>>()?;

        let construct_params: Vec<Ident> = self.fields.iter().map(
            |(field_name, _)| {
                format_ident!("{}", field_name)
            }
        ).collect();

        Ok(quote! {
            fn from_raw_result(
                result: &std::collections::HashMap<String, yukino::mapping::DatabaseValue>
            ) -> Result<Box<Self>, yukino::ParseError> {
                #(#fields;)*

                Ok(Box::new(#full_ident {
                    #(#construct_params),*
                }))
            }
        })
    }

    pub fn get_implement_token_stream(&self) -> Result<TokenStream, UnresolvedError> {
        let to_raw_value = self.convert_to_database_value_token_stream()?;
        let from_raw_result = self.convert_to_value_token_stream()?;

        let ident = TokenStream::from_str(
            self.entity_name().as_str()
        ).map_err(
            |_| UnresolvedError::new(&self.ident)
        )?;

        let definitions = self.get_definitions()?;

        Ok(quote! {
            impl yukino::Entity for #ident {
                #from_raw_result

                #to_raw_value

                fn get_definitions(&self) -> Vec<yukino::mapping::definition::TableDefinition> {
                    vec![
                        #(#definitions),*
                    ]
                }
            }
        })
    }
}