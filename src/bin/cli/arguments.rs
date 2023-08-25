use std::path::PathBuf;
use clap::{Parser, Subcommand};

use super::enumerate::*;

#[derive(Parser, Debug)]
#[command(version)]
pub struct MazeirCli {
    #[command(subcommand)]
    pub map_type: MapType,
}

#[derive(Subcommand, Debug)]
pub enum MapType {
    /// 2D orthogonal maze
    #[command(alias = "o")]
    Orthogonal(OrthogonalCli),
}

#[derive(Parser, Debug)]
pub struct OrthogonalCli {
    /// Width of the maze
    #[arg(default_value = "16")]
    pub width: usize,

    /// Height of the maze
    #[arg(default_value = "16")]
    pub height: usize,

    /// Algorithm to generate the maze. support: DepthFirst only
    #[arg(short, long, default_value = "DepthFirst")]
    pub algorithm: Algorithm,

    /// Seed for the maze
    #[arg(short, long)]
    pub seed: Option<String>,

    /// Draw the maze to a png file
    #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
    pub draw: Option<PathBuf>,

    /// Print the maze to stdout
    #[arg(short, long)]
    pub print: bool,
}