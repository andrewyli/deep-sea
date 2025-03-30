use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum DeepSeaError {
    Internal(String),
    AgentError(String),
}

impl Display for DeepSeaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeepSeaError::Internal(msg) => write!(f, "Internal error: {msg}"),
            DeepSeaError::AgentError(msg) => write!(f, "Agent error: {msg}"),
        }
    }
}

impl Error for DeepSeaError {}

pub type DeepSeaResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;
