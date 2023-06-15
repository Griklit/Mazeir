use std::path::PathBuf;
use std::str::FromStr;

use argh::FromArgs;
use rand::random;

use mazeir::{Maze, OutputType, GeneratorType, MazeError};
use mazeir::CommandLineInterfaceError as CLIErr;

#[derive(FromArgs, Debug)]
/// 123
struct CliArguments {
    /// width of maze
    #[argh(option, short = 'w', default = "127")]
    width: usize,

    /// height of maze
    #[argh(option, short = 'h', default = "127")]
    height: usize,

    /// seed
    #[argh(option, short = 's', default = "random()")]
    seed: u64,

    /// generator(algorithm) type, options: depth-first
    #[argh(option, short = 'g', default = "GeneratorType::DepthFirst", from_str_fn(GeneratorType::from_str))]
    generator: GeneratorType,

    /// output type, options: image, text, stdout
    #[argh(option, short = 'o', default = "\"stdout\".to_string()")]
    output: String,

    #[argh(option, from_str_fn(PathBuf::from_str))]
    /// output path
    output_path: Option<PathBuf>,
}

fn cli() -> Result<(), CLIErr> {
    let args: CliArguments = argh::from_env();
    let output_str = args.output.to_ascii_lowercase();
    let output = match output_str.as_str() {
        "image" | "pic" | "png" => OutputType::Image(args.output_path.unwrap_or(PathBuf::from_str("maze.png").unwrap())),
        "stdout" | "print" => OutputType::Stdout,
        "text" | "txt" => OutputType::Text(args.output_path.unwrap_or(PathBuf::from_str("maze.txt").unwrap())),
        _ => { return Err(CLIErr::BuildMazeError(MazeError::InvalidOutputType(output_str))); }
    };
    let mut maze = Maze::new(args.width, args.height).map_err(|e| CLIErr::BuildMazeError(e))?;
    maze.seed(args.seed)
        .build(args.generator)
        .output(output).map_err(|e| CLIErr::BuildMazeError(e))?;
    Ok(())
}

fn main() {
    match cli() {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}