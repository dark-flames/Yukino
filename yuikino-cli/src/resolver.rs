use yukino::mapping::{CellResolver, FieldResolveCell};
use std::fs::File;
use crate::error::FileError;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Resolver {
    cell_resolvers: CellResolver,
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
            cell_resolvers: CellResolver::new(seeds),
            model_files,
            output_file
        })
    }
}