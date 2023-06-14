use std::str::FromStr;

use argh::FromArgs;
use rand::random;

use mazeir::{OutputType, GeneratorType};
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

    /// generator(algorithm) type
    #[argh(option, short = 'g', default = "GeneratorType::DepthFirst", from_str_fn(GeneratorType::from_str))]
    generator: GeneratorType,

    // #[argh(option, short = 'o', default = "vec![OutputType::Stdout]")]
    // /// output type
    // output: Vec<OutputType>,
}

fn main() {
    let args: CliArguments = argh::from_env();
    println!("{:?}", args);
}