Mazeir
---
![123](/tests/orthogonal/snapshot/depth-first_1076x593.png)

**For Giant Maze**

## Library

```rust
use mazeir::map::Orthogonal;
use mazeir::algorithm::DepthFirst;
use mazeir::output::Print;

fn main() {
    let mut maze = Orthogonal::new(7, 16);
    maze.depth_first();
    maze.print();
}
```

### Maze Map Type

- `Orthogonal` one byte one cell

### Algorithm Traits

- `DepthFirst`

### Output Traits

- `Print`
- `Draw` stream write to 1bit png file

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

Orthogonal Maze

```bash
>>> mazeir-cli orthogonal --help

2D orthogonal maze

Usage: mazeir-cli orthogonal [OPTIONS] [WIDTH] [HEIGHT]

Arguments:
  [WIDTH]   Width of the maze [default: 16]
  [HEIGHT]  Height of the maze [default: 16]

Options:
  -a, --algorithm <ALGORITHM>  Algorithm of generate the maze. support DepthFirst only [default: DepthFirst]
  -s, --seed <SEED>              Seed for the maze
  -d, --draw <OUTPUT_FILE_PATH>  Draw the maze to a png file
  -p, --print                    Print the maze to stdout
  -h, --help                     Print help
```

## Features

### Map

- [x] 2D orthogonal maze
- [ ] 2D hexagonal maze
- [ ] 3D orthogonal maze

### Algorithm

- [x] Depth First
- [ ] Some kind of borderless algorithm
- [ ] Etc..

### Output

- [x] Print to stdout
- [x] Draw to png file
- [ ] Output to txt file
- [ ] Custom color index 1bit PNG file
