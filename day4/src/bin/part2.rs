//! Wharehouse full of rolls.
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::fmt::Display;

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

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::Roll => '@',
            Self::Empty => '.',
        };
        write!(f, "{char}")?;
        Ok(())
    }
}

fn display_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
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

fn remove(grid: &[Vec<Cell>]) -> (Vec<Vec<Cell>>, u32) {
    let mut output = vec![vec![]];
    let mut num_free = 0;
    for (row, line) in grid.iter().enumerate() {
        let mut output_row = vec![];
        for (col, cell) in line.iter().enumerate() {
            let update = match cell {
                Cell::Roll => {
                    if neigbours(grid, row, col) < 4 {
                        num_free += 1;
                        Cell::Empty
                    } else {
                        Cell::Roll
                    }
                }
                Cell::Empty => Cell::Empty,
            };
            output_row.push(update);
        }
        output.push(output_row);
    }
    (output, num_free)
}

fn part1(input: &str) -> u32 {
    let mut total = 0;
    let mut grid = form_grid(input);
    'removal_loop: loop {
        let (new, num_free) = remove(&grid);

        if num_free == 0 {
            break 'removal_loop;
        }
        total += num_free;

        // Display map.
        // display_grid(&grid);

        grid = new;
    }
    total
}
