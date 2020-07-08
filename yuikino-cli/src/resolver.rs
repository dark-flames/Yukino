use yukino::mapping::{CellResolver, FieldResolveCell};
use std::fs::File;
use crate::error::{FileError, ResolveError, OutputError};
use std::collections::HashMap;
use std::io::{Read, Write};
use proc_macro2::TokenStream;
use std::str::FromStr;
use syn::{Item, DeriveInput};
use std::convert::From;


#[allow(dead_code)]
pub struct Resolver {
    pub cell_resolver: CellResolver,
    model_files: HashMap<&'static str, File>,
    output_file: File
}

#[allow(dead_code)]
impl Resolver {
    pub fn new(
        seeds: Vec<Box<dyn FieldResolveCell>>,
        model_files_path: HashMap<&'static str, &'static str>,
        output_file_path: &'static str
    ) -> Result<Self, FileError> {
        let model_files = model_files_path.into_iter().map(
            |(mod_path, path)| File::open(path).map(
                |file| (mod_path, file)
            ).map_err(
                |e| FileError::new(path, e)
            )
        ).collect::<Result<HashMap<&'static str, File>, FileError>>()?;

        let output_file  = File::open(output_file_path).map_err(
            |e| FileError::new(output_file_path, e)
        )?;


        Ok(Resolver {
            cell_resolver: CellResolver::new(seeds),
            model_files,
            output_file
        })
    }

    pub fn resolve(&mut self) -> Result<(), ResolveError> {
        for (mod_path, file) in self.model_files.iter_mut() {
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(
                |e| ResolveError::new(mod_path, e.to_string().as_str())
            )?;

            TokenStream::from_str(content.as_str()).map_err(
                |_| {
                    ResolveError::new(mod_path, "Error occur while generate token stream")
                }
            )?;

            let syntax = syn::parse_file(&content).map_err(
                |e| ResolveError::new(mod_path, e.to_string().as_str())
            )?;

            for item in syntax.items {
                if let Item::Struct(item_struct) = item {
                    self.cell_resolver.parse(DeriveInput::from(item_struct), mod_path).map_err(
                        |e| ResolveError::new(mod_path, e.to_string().as_str())
                    )?;
                };
            }
        }
        Ok(())
    }

    pub fn write_implements(&mut self) -> Result<(), OutputError> {
        let result = self.cell_resolver.get_implements().map_err(
            |e| OutputError::new(e.to_string().as_str())
        )?;

        let result_string = result.to_string();
        self.output_file.write_all(result_string.as_bytes()).map_err(
            |e| OutputError::new(e.to_string().as_str())
        )
    }
}