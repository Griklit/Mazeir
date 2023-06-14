use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MazeError {
    SizeIsZero,
    InvalidGeneratorType,
    InvalidOutputType,
    OutputError(String),
    Unsupported(String),
}

impl Display for MazeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeError::SizeIsZero => write!(f, "width or height cannot be zero"),
            MazeError::InvalidGeneratorType => write!(f, "invalid generator type"),
            MazeError::InvalidOutputType => write!(f, "invalid output type"),
            MazeError::OutputError(s) => write!(f, "failed to output maze, {s}"),
            MazeError::Unsupported(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug)]
pub enum CommandLineInterfaceError {
    ParseArgumentError(String),
    BuildMazeError(MazeError),
    Unsupported(String),
}

impl Display for CommandLineInterfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandLineInterfaceError::ParseArgumentError(s) => write!(f, "parsing argument failed, {s}"),
            CommandLineInterfaceError::BuildMazeError(m) => write!(f, "build maze error, {m}"),
            CommandLineInterfaceError::Unsupported(s) => write!(f, "{s}"),
        }
    }
}
