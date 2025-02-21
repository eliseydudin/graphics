use std::{error, ffi::NulError, fmt};

#[derive(Debug)]
pub enum ShaderError {
    CompilationError(String),
    LinkingError(String),
    CStringConversion(NulError),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompilationError(err) => write!(f, "Cannot compile the shader: {err}"),
            Self::CStringConversion(err) => {
                write!(f, "Cannot convert string to a C pointer: {err}")
            }
            Self::LinkingError(err) => write!(f, "Cannot link the shaders: {err}"),
        }
    }
}

impl error::Error for ShaderError {}
