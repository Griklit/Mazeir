mod arguments;
mod enumerate;

use arguments::{MazeirCli, MapType, OrthogonalCli};
use enumerate::*;

use std::path::Path;
use clap::Parser;

use mazeir::map::Orthogonal;
use mazeir::algorithm::DepthFirst;
use mazeir::output::{Draw, Print};


fn main() {
    let cli = MazeirCli::parse();
    match cli.map_type {
        MapType::Orthogonal(x) => orthogonal_cli(x),
    }
}

fn orthogonal_cli(cli: OrthogonalCli) {
    let mut maze = Orthogonal::new(cli.width, cli.height);
    if let Some(seed) = cli.seed {
        match cli.algorithm {
            Algorithm::DepthFirst => maze.depth_first_with_str_seed(&seed),
        }
    } else {
        match cli.algorithm {
            Algorithm::DepthFirst => maze.depth_first(),
        }
    }
    if cli.print {
        maze.print();
    }
    if let Some(draw) = cli.draw {
        let file = std::fs::File::create(Path::new(&draw)).unwrap();
        maze.draw(file).unwrap();
    }
}