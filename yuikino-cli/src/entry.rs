use crate::resolver::Resolver;
use yukino::mapping::FieldResolveCell;
use std::collections::HashMap;
use crate::error::{YukinoCLIError};
use clap::{App, crate_version, crate_authors, crate_description, SubCommand};
use std::process::exit;

#[allow(dead_code)]
pub struct CommandLineEntry {
    resolver: Resolver
    // db connection
}

#[allow(dead_code)]
impl CommandLineEntry {
    pub fn new(
        seeds: Vec<Box<dyn FieldResolveCell>>,
        model_files_path: HashMap<&'static str, &'static str>,
        output_file_path: &'static str
    ) -> Self {
        let resolver = Self::handle_result(Resolver::new(
            seeds,
            model_files_path,
            output_file_path
        ).map_err(|e| YukinoCLIError::from(e)));

        CommandLineEntry {
            resolver
        }
    }
    pub fn process(&mut self) {
        let application = App::new("Yukino CommandLine Tool")
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .subcommand(
                SubCommand::with_name("setup")
                    .about("Setup entities")
                    .version(crate_version!())
                    .author(crate_authors!())
            );
        let matches = application.get_matches();
        if let Some(_) = matches.subcommand_matches("setup") {
            let result = self.setup();
            Self::handle_result(result)
        }
    }

    fn handle_result<R>(result: Result<R, YukinoCLIError>) -> R {
        match result {
            Ok(result) => result,
            Err(err) => {
                eprintln!("error: {:?}", err);
                exit(1)
            }
        }
    }

    fn setup(&mut self) -> Result<(), YukinoCLIError> {
        self.resolver.resolve()?;

        self.resolver.write_implements()?;

        Ok(())
    }
}
