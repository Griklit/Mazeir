#[derive(Debug)]
pub enum MazeError {
    WidthIsZero,
    HeightIsZero,
    Unsupported(String),
}

impl ToString for MazeError {
    fn to_string(&self) -> String {
        match self {
            MazeError::WidthIsZero => "width cannot be zero".to_string(),
            MazeError::HeightIsZero => "height cannot be zero".to_string(),
            MazeError::Unsupported(s) => s
        }
    }
}


#[derive(Debug)]
pub enum CommandLineInterfaceError {
    SizeError(String),
    OutputTypeError(String),
    CreateMazeError(String),
    Unsupported(String),
}


impl ToString for CommandLineInterfaceError {
    fn to_string(&self) -> String {
        match self {
            CommandLineInterfaceError::SizeError(s) => format!("SizeError: {s}"),
            CommandLineInterfaceError::OutputTypeError(s) => format!("OutputTypeError: {s}"),
            CommandLineInterfaceError::CreateMazeError(s) => format!("CreateMazeError: {s}"),
            CommandLineInterfaceError::Unsupported(s) => format!("UnsupportedError: {s}"),
        }
    }
}