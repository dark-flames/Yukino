use crate::mapping::error::CompileError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct ResolveError(String);

impl ResolveError {
    #[allow(dead_code)]
    pub fn new<N: Display + ?Sized, D: Display + ?Sized>(name: &N, message: &D) -> Self {
        ResolveError(format!(
            "Resolve Error: Error('{}') occurred while resolving {}.",
            message, name
        ))
    }
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for ResolveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl CompileError for ResolveError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UnresolvedError(String);

impl UnresolvedError {
    #[allow(dead_code)]
    pub fn new<N: Display + ?Sized>(name: &N) -> Self {
        UnresolvedError(format!("Unresolved Error: Unresolved cell {}.", name))
    }
}

impl Display for UnresolvedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for UnresolvedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl CompileError for UnresolvedError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}
