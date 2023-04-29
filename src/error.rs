use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum EcsErrorKind {
    NodeExists,
    Other,
}

impl Display for EcsErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EcsErrorKind::Other => write!(f, "Other"),
            EcsErrorKind::NodeExists => write!(f, "NodeExists"),
        }
    }
}

#[derive(Debug)]
pub struct EcsError {
    description: String,
    kind: EcsErrorKind,
}

impl EcsError {
    pub fn node_exists() -> Self {
        Self {
            kind: EcsErrorKind::NodeExists,
            description: String::from("Node already exists"),
        }
    }

    pub fn other<T>(description: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            kind: EcsErrorKind::Other,
            description: description.into(),
        }
    }
}

impl Error for EcsError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Display for EcsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.description)
    }
}

impl From<String> for EcsError {
    fn from(value: String) -> Self {
        EcsError {
            description: value,
            kind: EcsErrorKind::Other,
        }
    }
}

impl From<&str> for EcsError {
    fn from(value: &str) -> Self {
        EcsError {
            description: value.to_string(),
            kind: EcsErrorKind::Other,
        }
    }
}
