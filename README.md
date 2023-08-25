Mazeir
---

## Library

```rust
use mazeir::map::Orthogonal;
use mazeir::arithmetic::DepthFirst;
use mazeir::output::Print;

fn main() {
    let mut maze = Orthogonal::new(7, 16);
    maze.depth_first();
    maze.print();
}
```

### Maze Map Type

- `Orthogonal`

### Arithmetic Traits

- `DepthFirst`

### Output Traits

- `Print`
- `Draw`

## Command Line Interface

### Base

```bash
>>> mazeir-cli --help

Usage: mazeir-cli <COMMAND>

Commands:
  orthogonal  2D orthogonal maze
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
>>> mazeir-cli orthogonal --help

2D orthogonal maze

Usage: mazeir-cli orthogonal [OPTIONS] [WIDTH] [HEIGHT]

Arguments:
  [WIDTH]   Width of the maze [default: 16]
  [HEIGHT]  Height of the maze [default: 16]

Options:
  -a, --arithmetic <ARITHMETIC>  Arithmetic to generate the maze. support: DepthFirst only [default: DepthFirst]
  -s, --seed <SEED>              Seed for the maze
  -d, --draw <OUTPUT_FILE_PATH>  Draw the maze to a png file
  -p, --print                    Print the maze to stdout
  -h, --help                     Print help
```