use std::env;
use std::str::FromStr;
use Into;
use std::path::Path;

use mazeir::*;
use mazeir::CommandLineInterfaceError as CLIErr;


enum OutPut {
    Image,
    StdOut,
}

impl FromStr for OutPut {
    type Err = CLIErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "image" | "png" | "jpg" => Ok(Self::Image),
            "stdout" | "print" => Ok(Self::StdOut),
            _ => Err(CLIErr::OutputTypeError(format!("{s} is not a legal value. The output argument must be ont of image,png or 000").to_string())),
        }
    }
}


fn cli() -> Result<(), CLIErr> {
    let mut width = 511;
    let mut height = 511;
    let mut output: Option<OutPut> = None;
    let mut output_path: Option<&Path> = None;
    let mut args = env::args().skip(1);
    while let arg = args.next() {
        if arg.is_none() { continue; }
        match arg.unwrap().as_str() {
            "-w" | "--width" | "--width=" => {
                if let Some(w) = args.next() {
                    width = w.parse().map_err(|e| CLIErr::SizeError(format!("Width parsing failed. {e:?}")))?;
                }
            }
            "-h" | "--height" | "--height=" => {
                if let Some(h) = args.next() {
                    height = h.parse().map_err(|e| CLIErr::SizeError(format!("Height parsing failed. {e:?}")))?;
                }
            }
            "-o" | "--output" | "--output=" => {
                if let Some(o) = args.next() {
                    output = Some(OutPut::from_str(o.as_str())?)
                }
            }
            "--output_path" | "--output_path=" => {
                if let Some(p) = args.next() {
                    output_path = Some(Path::new(p.as_str()))
                }
            }
            _ => ()
        }
    }
    let maze = Maze::new(width, height).map_err(|e| CLIErr::CreateMazeError(e.to_string()))?;

    Ok(())
}

fn main() {
    match cli() {
        Ok(_) => eprintln!("Maze build success!"),
        Err(err) => eprintln!("{}", err.to_string())
    }
}