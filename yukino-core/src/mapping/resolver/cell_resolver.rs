use std::collections::{HashMap};
use crate::mapping::resolver::entity_resolve_cell::{EntityResolveCell, EntityResolveStatus};
use crate::mapping::resolver::field_resolve_cell::{FieldPath, FieldResolveCell, FieldResolveStatus};
use syn::{DeriveInput, Error, Data, Fields};
use crate::mapping::error::{ CompileError};
use crate::mapping::attribution::{Table, FieldAttribute};
use crate::mapping::resolver::error::{ResolveError, UnresolvedError};
use syn::export::ToTokens;
use yui::AttributeStructure;
use crate::mapping::resolver::helper::compare_path_vector;
use crate::mapping::definition::TableDefinition;
use proc_macro2::TokenStream;

pub struct CellResolver {
    entity_cells: HashMap<String, EntityResolveCell>,
    wait_for_field: HashMap<FieldPath, Vec<FieldPath>>,
    wait_for_entity: HashMap<String, Vec<FieldPath>>,
    field_cells: HashMap<String, HashMap<String, Box<dyn FieldResolveCell>>>,
    field_cell_seeds: Vec<Box<dyn FieldResolveCell>>
}

#[allow(dead_code)]
impl CellResolver {
    pub fn new(seeds: Vec<Box<dyn FieldResolveCell>>) -> Self {
        CellResolver {
            entity_cells: HashMap::new(),
            wait_for_field: HashMap::new(),
            wait_for_entity: HashMap::new(),
            field_cells: HashMap::new(),
            field_cell_seeds: seeds
        }
    }

    fn add_field_cell(&mut self, cell: Box<dyn FieldResolveCell>) -> Result<&mut CellResolver, ResolveError> {
        let field_path = cell.field_path().unwrap();
        let status = cell.get_status();
        if let Some(map) =  self.field_cells.get_mut(&field_path.0) {
            map.insert(field_path.1.clone(), cell);
        } else {
            let mut map = HashMap::new();
            map.insert(field_path.1.clone(), cell);
            self.field_cells.insert(field_path.0.clone(), map);
        };

        self.process_field_resolve_status(&field_path, &status)
    }

    fn add_entity_cell(&mut self, cell: EntityResolveCell) -> Result<&mut CellResolver, ResolveError> {
        let name = cell.entity_name();
        let status = cell.get_status();
        self.entity_cells.insert(name.clone(), cell);

        self.process_entity_resolve_status(name, &status)
    }

    fn get_field_cell(&self, field_path: &FieldPath) -> Option<&dyn FieldResolveCell> {
        match self.field_cells.get(&field_path.0).map(
            |entity_map| {
                entity_map.get(&field_path.1)
            }
        ) {
            Some(Some(v)) => Some(v.as_ref()),
            _ => match self.entity_cells.get(&field_path.0).map(
                |entity| entity.get_field(&field_path.1)
            ) {
                Some(Some(v)) => Some(v),
                _ => None
            }
        }
    }

    fn get_mut_field_cell_from_hash_map<'a>(
        field_cells: &'a mut HashMap<String, HashMap<String, Box<dyn FieldResolveCell>>>,
        field_path: &FieldPath
    ) -> Option<&'a mut dyn FieldResolveCell>{
        match field_cells.get_mut (&field_path.0).map(
            |entity_map| {
                entity_map.get_mut(&field_path.1)
            }
        ) {
            Some(Some(v)) => Some(v.as_mut()),
            _ => None
        }
    }

    fn get_mut_field_cell(&mut self, field_path: &FieldPath) -> Option<&mut dyn FieldResolveCell> {
        Self::get_mut_field_cell_from_hash_map(&mut self.field_cells, field_path)
    }

    fn remove_field_cell(&mut self, field_path: &FieldPath) -> Option<Box<dyn FieldResolveCell>> {

        let field_map = match self.field_cells.get_mut(&field_path.0) {
            Some(m) => m,
            None => return None
        };

        let result = field_map.remove(&field_path.1);

        if field_map.is_empty() {
            self.field_cells.remove(&field_path.0);
        };

        result
    }

    fn process_resolve_fields(
        &mut self, field_path: &FieldPath
    ) -> Result<FieldResolveStatus, ResolveError> {
        let cell = self.get_field_cell(field_path).ok_or_else(
            || ResolveError::new(
                field_path,
                "Unknown field path"
            )
        )?;
        let mut result = cell.get_status();

        let can_resolve = cell.get_status().get_fields().ok_or_else(
            || ResolveError::new(
                field_path,
                "Cell is not waiting for fields"
            )
        )?.iter().any(
            |path| {
                !self.get_field_cell(path).map(
                    |cell| cell.get_status().is_finished()
                ).unwrap_or(false)
            }
        );

        if can_resolve {
            let mut cell = self.remove_field_cell(field_path).unwrap();

            let fields: HashMap<FieldPath, &dyn FieldResolveCell> = cell.get_status()
                .get_fields().unwrap().iter().map(
                    |path| (path.clone(), self.get_field_cell(path).unwrap())
                ).collect();

            result = cell.resolve_fields(fields)?;

            self.add_field_cell(cell)?;
        }

        Ok(result)
    }

    fn process_resolve_entity(
        &mut self,
        field_path: &FieldPath
    ) -> Result<FieldResolveStatus, ResolveError> {
        let cell = self.get_field_cell(&field_path).ok_or_else(
            || ResolveError::new(
                field_path,
                "Unknown field path"
            )
        )?;
        let mut result = cell.get_status();
        if cell.get_status().get_entity().ok_or_else(
            || ResolveError::new(
                &format!("{}::{}", &field_path.0, &field_path.1),
                "Cell is not waiting for entity"
            )
        ).map(
            |entity_name| self.entity_cells.get(entity_name).map(
                |entity_cell| entity_cell.get_status() == EntityResolveStatus::Finished
            ).unwrap_or(false)
        )? {
            let mut cell = self.remove_field_cell(&field_path).unwrap();
            let entity = self.entity_cells.get(result.get_entity().unwrap()).unwrap();
            result = cell.resolve_entity(entity)?;

            self.add_field_cell(cell)?;
        }

        Ok(result)
    }

    fn process_entity_resolve_status(
        &mut self, entity_name: String,
        result: &EntityResolveStatus
    ) -> Result<&mut Self, ResolveError> {
        match result {
            EntityResolveStatus::Finished => {
                let default = Vec::new();
                let paths: Vec<FieldPath> = self.wait_for_entity
                    .get(&entity_name)
                    .unwrap_or(&default)
                    .clone();

                for path in paths.iter() {
                    self.process_resolve_entity(path)?;
                }
            },
            EntityResolveStatus::Assemble => {}
            _ => return Err(ResolveError::new(&entity_name, "Unexpected resolve status"))
        };

        Ok(self)
    }

    fn process_field_resolve_status(
        &mut self, field_path: &FieldPath,
        result: &FieldResolveStatus
    ) -> Result<&mut Self, ResolveError> {
        match result {
            FieldResolveStatus::Finished => {
                let cell = self.get_field_cell(&field_path).ok_or_else(
                    || ResolveError::new(
                        &format!("{}::{}", &field_path.0, &field_path.1),
                        "Unknown field path"
                    )
                )?;

                let entity_name = cell.entity_name();

                let default = Vec::new();
                let paths = self.wait_for_field.remove(&field_path).unwrap_or(default);

                for path in paths.iter() {
                    self.process_resolve_fields(path)?;
                }

                let cell = self.remove_field_cell(&field_path).unwrap();

                let entity = self.entity_cells.get_mut(
                    entity_name.as_ref().unwrap()
                ).ok_or_else(
                    || ResolveError::new(
                        entity_name.as_ref().unwrap(),
                        format!("Unknown entity name: {}", entity_name.as_ref().unwrap()).as_str()
                    )
                )?;

                let entity_resolve_status = entity.assemble_column(cell);
                self.process_entity_resolve_status(entity_name.unwrap(), &entity_resolve_status)?;
            },
            FieldResolveStatus::WaitAssemble => {
                let fields_cells = &mut self.field_cells;
                let entity_cells = &self.entity_cells;
                let cell = Self::get_mut_field_cell_from_hash_map(
                    fields_cells,
                    &field_path
                ).ok_or_else(
                    || ResolveError::new(
                        &format!("{}::{}", &field_path.0, &field_path.1),
                        "Unknown field path"
                    )
                )?;

                let entity = entity_cells.get(
                    &cell.entity_name().unwrap()
                ).ok_or_else(
                    || ResolveError::new(&cell.entity_name().unwrap(), "Unknown entity name")
                )?;

                match cell.assemble(entity)? {
                    FieldResolveStatus::Finished => self.process_field_resolve_status(
                        field_path, &FieldResolveStatus::Finished
                    )?,
                    _ => return Err(
                        ResolveError::new(
                            &format!("{}::{}", &field_path.0, &field_path.1),
                            "Can not assembly field"
                        )
                    )
                };
            }
            FieldResolveStatus::WaitEntity(entity) => {
                let result = self.process_resolve_entity(field_path)?;

                match &result {
                    FieldResolveStatus::WaitEntity(other) if other == entity => {
                        let list_option = self.wait_for_entity.get_mut(other);

                        if let Some(list) = list_option {
                            list.push(field_path.clone())
                        } else {
                            let list = vec![field_path.clone()];

                            self.wait_for_entity.insert(other.clone(), list);
                        }
                    },
                    _ => {
                        self.process_field_resolve_status(field_path, &result)?;
                    }
                };
            }
            FieldResolveStatus::WaitFields(fields) => {
                let result = self.process_resolve_fields(field_path)?;

                match &result {
                    FieldResolveStatus::WaitFields(other) if compare_path_vector(fields, other) => {
                        for field in fields {
                            let list_option = self.wait_for_field.get_mut(field);

                            if let Some(list) = list_option {
                                list.push(field_path.clone())
                            } else {
                                let list = vec![field_path.clone()];

                                self.wait_for_field.insert(
                                    field.clone(),
                                    list
                                );
                            }
                        };
                    },
                    _ => {
                        self.process_field_resolve_status(field_path, &result)?;
                    }
                };
            }
            _ => return Err(
                ResolveError::new(field_path, "Unexpected resolve status")
            )
        }

        Ok(self)
    }

    pub fn parse(&mut self, input: DeriveInput, mod_path: &'static str) -> Result<&mut Self, syn::Error> {
        let mut table_attr= None;
        for attr in input.attrs.iter() {
            if attr.path == Table::get_path() {
                let meta = attr.parse_meta()?;
                let result: Table = Table::from_meta(&meta)?;
                table_attr = Some(result)
            }
        };

        if let Data::Struct(data_struct) = &input.data {
            if let Fields::Named(fields_named) = &data_struct.fields {
                let entity_cell = EntityResolveCell::new(
                    &input.ident,
                    mod_path,
                    &table_attr,
                    data_struct.fields.len()
                ).map_err(|e| Error::new_spanned(&input, e.get_message()))?;

                let entity_name = entity_cell.entity_name();

                self.add_entity_cell(entity_cell).map_err(
                    |e| Error::new_spanned(&input, e.get_message())
                )?;

                for field in fields_named.named.iter() {
                    let field_attrs = field.attrs.iter().map(
                        |attr| FieldAttribute::from_attr(attr)
                    ).collect::<Result<Vec<FieldAttribute>, Error>>()?;

                    let field_cell: Box<dyn FieldResolveCell> = self.field_cell_seeds.iter().fold(
                        Err(Error::new_spanned(
                            &input,
                            ResolveError::new(&field.ident.to_token_stream().to_string(), "No FieldResolveCell matched").get_message()
                        )),
                        |result, cell| {
                            if result.is_err() && cell.match_field(&field_attrs, &field.ty) {
                                cell.breed(
                                    entity_name.clone(),
                                    field.ident.as_ref().unwrap(),
                                    &field_attrs,
                                    &field.ty
                                ).map_err(|e| Error::new_spanned(field, e.get_message()))
                            } else {
                                result
                            }
                        }
                    )?;

                    self.add_field_cell(field_cell).map_err(
                        |e| Error::new_spanned(&field, e.get_message())
                    )?;
                }
                Ok(())
            } else {
                Err(Error::new_spanned(
                    &input,
                    ResolveError::new(&input.ident, "Field of struct must be named field").get_message()
                ))
            }
        } else {
            Err(Error::new_spanned(
                &input,
                ResolveError::new(&input.ident, "Enum or Union are not supported").get_message()
            ))
        }?;

        Ok(self)
    }

    pub fn get_definitions(&mut self) -> Result<Vec<TableDefinition>, UnresolvedError> {
        let mut definitions = Vec::new();

        for (_, cell) in self.entity_cells.iter_mut() {
            let mut result = match cell.get_status() {
                EntityResolveStatus::Achieved => cell.get_definitions().unwrap(),
                _ => {
                    cell.achieve()?;
                    cell.get_definitions().unwrap()
                }
            };
            definitions.append(&mut result);
        }

        Ok(definitions)
    }

    pub fn get_implements(&mut self) -> Result<TokenStream, UnresolvedError> {
        self.entity_cells.iter_mut().map(
            |(_, cell)| {
                cell.achieve()?;
                cell.get_implement_token_stream()
            }
        ).fold(
            Ok(TokenStream::new()),
            |previous, current| {
                match previous {
                    Ok(mut previous_token_stream) => {
                        match current {
                            Ok(current_token_stream) => {
                                current_token_stream.to_tokens(&mut previous_token_stream);
                                Ok(previous_token_stream)
                            },
                            Err(e) => Err(e)
                        }
                    },
                    Err(e) => Err(e)
                }
            }
        )
    }
}