use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ProblemError {
    Unsatisfiable,
    Dimensions,
}

impl Error for ProblemError {}

impl Display for ProblemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemError::Unsatisfiable => f.write_str("Cannot satisfy problem description"),
            ProblemError::Dimensions => f.write_str("Invalid dimensions occured"),
        }
    }
}
