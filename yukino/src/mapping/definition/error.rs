use std::fmt::{Display, Formatter, Result};
use std::error::Error;
use crate::mapping::error::CompileError;

#[derive(Debug, Clone)]
pub struct DefinitionError(String);

impl DefinitionError {
    #[allow(dead_code)]
    pub fn new<D: Display + ?Sized>(
        message: &D,
    ) -> Self {
        DefinitionError(format!(
            "Definition Error: Error('{}') occurred while resolving Definition.",
            message
        ))
    }
}

impl Display for DefinitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for DefinitionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl CompileError for DefinitionError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}