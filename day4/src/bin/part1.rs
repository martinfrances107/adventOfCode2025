//! Wharehouse full of rolls.
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug)]
struct Offset {
    row: i64,
    col: i64,
}

static DIRECTIONS: [Offset; 8] = [
    Offset { row: -1, col: 0 },  // N
    Offset { row: -1, col: 1 },  // NE
    Offset { row: 0, col: 1 },   // E
    Offset { row: 1, col: 1 },   // SE
    Offset { row: 1, col: 0 },   // S
    Offset { row: 1, col: -1 },  // SW
    Offset { row: 0, col: -1 },  // W
    Offset { row: -1, col: -1 }, // NW
];

#[derive(PartialEq)]
enum Cell {
    Roll,
    Empty,
}

fn neigbours(grid: &[Vec<Cell>], origin_row: usize, origin_col: usize) -> u32 {
    let mut num_neighbours = 0;

    for Offset {
        row: row_offset,
        col: col_offset,
    } in &DIRECTIONS
    {
        let row_index = origin_row as i64 + row_offset;
        let col_index = origin_col as i64 + col_offset;
        if let Some(target_row) = grid.get(row_index as usize)
            && let Some(target_cell) = target_row.get(col_index as usize)
        {
            match target_cell {
                Cell::Roll => {
                    num_neighbours += 1;
                }
                Cell::Empty => {}
            }
        }
    }

    num_neighbours
}

fn form_grid(input: &str) -> Vec<Vec<Cell>> {
    let mut grid: Vec<Vec<Cell>> = vec![vec![]];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            if c == '@' {
                row.push(Cell::Roll);
            } else if c == '.' {
                row.push(Cell::Empty);
            } else if c == '\n' {
                // ignore newline
            } else {
                panic!("invalid grid char - {c}");
            }
        }
        grid.push(row);
    }

    grid
}

fn compute_access(grid: &[Vec<Cell>]) -> (String, u32) {
    let mut output = String::new();
    let mut num_free = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, cell) in line.iter().enumerate() {
            let update = match cell {
                Cell::Roll => {
                    if neigbours(grid, row, col) < 4 {
                        num_free += 1;
                        'x'
                    } else {
                        '@'
                    }
                }
                Cell::Empty => '.',
            };
            output.push(update);
        }
        output.push('\n');
    }
    (output, num_free)
}

fn part1(input: &str) -> u32 {
    let grid = form_grid(input);
    let (output, num_free) = compute_access(&grid);
    println!("{output}");
    num_free
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn compute() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let expected_output = "\n..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
";

        let grid = form_grid(input);
        let (output, num_free) = compute_access(&grid);

        assert_eq!(output, expected_output);
        assert_eq!(num_free, 13);
    }
}
