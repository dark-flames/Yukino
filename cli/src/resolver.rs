use crate::entry::ModePath;
use crate::error::CLIError;
use entity::resolver::entity_resolver_passes::{
    ConverterGetterResolverPass, EntityImplementResolverPass,
};
use entity::resolver::{FieldResolverSeedBox, ImmutableSchemaResolver, SchemaResolver};
use std::collections::HashMap;
use std::fs::{remove_file, File};
use std::io::{Error as IOError, Read, Write};
use std::path::Path;
use syn::{parse_file, DeriveInput, Item};

pub struct Resolver {
    schema_resolver: SchemaResolver,
    model_files: HashMap<ModePath, File>,
    output_file_path: String,
}

impl Resolver {
    pub fn new(
        seeds: Vec<FieldResolverSeedBox>,
        model_file_paths: HashMap<ModePath, String>,
        output_file_path: String,
    ) -> Result<Self, CLIError> {
        let model_files = model_file_paths
            .into_iter()
            .map(|(mod_path, path)| File::open(path).map(|file| (mod_path, file)))
            .collect::<Result<HashMap<ModePath, File>, IOError>>()?;

        Ok(Resolver {
            schema_resolver: SchemaResolver::new(
                seeds,
                vec![
                    Box::new(EntityImplementResolverPass),
                    Box::new(ConverterGetterResolverPass),
                ],
            ),
            model_files,
            output_file_path,
        })
    }

    pub fn resolve(mut self) -> Result<AchievedResolver, CLIError> {
        for (mod_path, file) in self.model_files.iter_mut() {
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            let syntax =
                parse_file(content.as_str()).map_err(|e| CLIError::ParseError(e.to_string()))?;

            for item in syntax.items {
                if let Item::Struct(item_struct) = item {
                    self.schema_resolver
                        .parse(DeriveInput::from(item_struct), mod_path)
                        .map_err(|e| CLIError::ResolveError(e.to_string()))?;
                };
            }
        }

        let path = Path::new(&self.output_file_path);
        if path.exists() {
            remove_file(path)?
        };

        let output_file = File::create(&self.output_file_path)?;

        Ok(AchievedResolver {
            output_file,
            schema_resolver: self
                .schema_resolver
                .achieve()
                .map_err(|e| CLIError::ResolveError(e.to_string()))?,
        })
    }
}

pub struct AchievedResolver {
    output_file: File,
    schema_resolver: ImmutableSchemaResolver,
}

impl AchievedResolver {
    pub fn write_result(&mut self) -> Result<(), CLIError> {
        let result = self.schema_resolver.get_implements().to_string();

        self.output_file.write_all(result.as_bytes())?;

        Ok(())
    }
}
