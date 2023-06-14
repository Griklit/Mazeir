use std::env;
use std::str::FromStr;
use std::path::Path;
use std::collections::HashMap;

use rand::random;

use mazeir::{base::*, generator::*, output::*};
use CommandLineInterfaceError as CLIErr;

struct CliArguments {
    width: usize,
    height: usize,
    seed: u64,
    generator: GeneratorType,
    output: Vec<OutputType>,
}

impl CliArguments {
    fn new(value: env::Args) -> Result<Self, CLIErr> {
        let mut args = value.skip(1).collect::<Vec<String>>().iter();
        let mut args = {
            let mut new_args = Vec::new();
            while let Some(arg) = args.next() {
                if arg.starts_with("--") && arg.contains('=') {
                    let mut arg = arg.splitn(1, '=').collect::<Vec<String>>();
                    while let Some(s) = arg.next() {
                        new_args.push(s.to_string());
                    }
                } else {
                    new_args.push(arg.clone());
                }
            }
            new_args.iter()
        };
        let mut pos_args = Vec::new();
        let mut width = None;
        let mut height = None;
        let mut generator = None;
        let mut output = None;
        let mut output_path = None;
        let mut seed = None;
        while let arg = args.next() {
            if arg.is_none() { break; }
            let arg = arg.unwrap();
            if arg.starts_with("-") {
                match arg.as_str() {
                    "-w" | "--width" => { width = args.next(); }
                    "-h" | "--height" => { height = args.next(); }
                    "-s" | "--seed" => { seed = args.next(); }
                    "-g" | "--generator" => { generator = args.next(); }
                    "-o" | "--output" => { output = args.next(); }
                    "-p" | "--path" | "--output_path" => { output_path = args.next(); }
                    _ => ()
                }
            } else {
                pos_args.push(arg);
            }
        }
        let mut self_ = Self::default();
        Ok(self_)
    }
}

impl Default for CliArguments {
    fn default() -> Self {
        Self {
            width: 127,
            height: 127,
            generator: GeneratorType::DepthFirst,
            output: Vec::new(),
            seed: random(),
        }
    }
}

fn cli() -> Result<(), CLIErr> {
    let mut pos_args = Vec::new();
    let mut width: Option<String> = None;
    let mut height: Option<String> = None;
    let mut generator: Option<String> = None;
    let mut output: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut seed: Option<String> = None;
    let mut args = env::args().skip(1);
    while let arg = args.next() {
        if arg.is_none() { break; }
        let arg = arg.unwrap();
        if arg.starts_with("-") {
            match arg.as_str() {
                "-w" | "--width" => { width = args.next(); }
                "-h" | "--height" => { height = args.next(); }
                "-g" | "--generator" => { generator = args.next(); }
                "-o" | "--output" => { output = args.next(); }
                "-p" | "--path" | "--output_path" => { output_path = args.next(); }
                "-s" | "--seed" => { seed = args.next(); }
                _ => ()
            }
        } else {
            pos_args.push(arg);
        }
    }
    pos_args.reverse();
    if width.is_none() { width = pos_args.pop(); }
    if height.is_none() { height = pos_args.pop(); }
    let mut p_width = 127usize;
    if let Some(w) = width {
        p_width = w.parse().map_err(|e| CLIErr::ParseArgumentError(format!("Width parsing failed, {e}")))?;
    }
    let mut p_height = 127usize;
    if let Some(h) = height {
        p_height = h.parse().map_err(|e| CLIErr::ParseArgumentError(format!("Height parsing failed, {e}")))?;
    }
    let mut p_generator = GeneratorType::DepthFirst;
    if let Some(g) = generator {
        p_generator = GeneratorType::from_str(g.as_str())
            .map_err(|e| CLIErr::ParseArgumentError(format!("Generator parsing failed, {e}")))?;
    }
    let mut p_output = OutputType::Stdout;
    if let Some(o) = output {
        p_output = OutputType::from_str(o.as_str())
            .map_err(|e| CLIErr::ParseArgumentError(format!("Output parsing failed. {e:?}")))?;
    }
    let mut p_output_path = Path::new("");
    if let Some(p) = output_path {
        p_output_path = Path::new(p.as_str());
    } else {
        p_output_path = Path::new(match p_output {
            OutputType::Image => "maze.png",
            OutputType::Stdout => "",
            OutputType::Text => "maze.txt",
        });
    }
    let mut p_seed = u64::default();
    if let Some(s) = seed {
        p_seed = s.parse().map_err(|e| CLIErr::ParseArgumentError(format!("Seed parsing failed. {e:?}")))?
    }
    let mut maze = Maze::new(p_width, p_height).map_err(|e| CLIErr::BuildMazeError(e))?;
    // maze.seed(seed);
    maze.build(p_generator)
        .output(p_output, p_output_path)
        .map_err(|e| CLIErr::BuildMazeError(e))?;
    Ok(())
}

fn test() {
    let mut maze = Maze::new(32, 10).unwrap();
    maze.depth_first();
    maze.print();
}

fn main() {
    // test();
    match cli() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err.to_string())
    }
}