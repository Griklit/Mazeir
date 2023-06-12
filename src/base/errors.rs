#[derive(Debug)]
pub enum MazeError {
    SizeIsZero,
    InvalidGeneratorType,
    InvalidOutputType,
    OutputError(String),
    Unsupported(String),
}

impl ToString for MazeError {
    fn to_string(&self) -> String {
        match self {
            MazeError::SizeIsZero => "Width or height cannot be zero".to_string(),
            MazeError::InvalidGeneratorType => "Invalid generator type".to_string(),
            MazeError::InvalidOutputType => "Invalid output type".to_string(),
            MazeError::OutputError(s) => format!("Failed to output maze, {}", s),
            MazeError::Unsupported(s) => s.clone(),
        }
    }
}


#[derive(Debug)]
pub enum CommandLineInterfaceError {
    ArgumentParseError(String),
    CreateMazeError(MazeError),
    Unsupported(String),
}


impl ToString for CommandLineInterfaceError {
    fn to_string(&self) -> String {
        match self {
            CommandLineInterfaceError::ArgumentParseError(s) => format!("ArgumentParseError: {s}"),
            CommandLineInterfaceError::CreateMazeError(m) => format!("CreateMazeError: {}", m.to_string()),
            CommandLineInterfaceError::Unsupported(s) => format!("UnsupportedError: {s}"),
        }
    }
}