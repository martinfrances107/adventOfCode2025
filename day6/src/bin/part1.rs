//! Kitchen
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use nom::{
    IResult,
    branch::alt,
    character::complete::{char, digit1, newline, space0, space1},
    combinator::{map_res, recognize},
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated, tuple},
};

use std::ops::{Index, IndexMut};

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), |s: &str| s.parse::<u32>())(input)
}

fn parse_list_of_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, parse_number)(input)
}
// Parse a line of natural numbers separated by one or more spaces, terminated by a newline
fn parse_line_of_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(space0, parse_list_of_numbers, newline)(input)
}

fn parse_block_of_numbers(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(parse_line_of_numbers)(input)
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mul,
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    map_res(alt((char('+'), char('*'))), |c| {
        match c {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Mul),
            _ => {
                // a

                Err(Error::new("Failed to parse operator", ErrorKind::Alt))
                // Err(bool)
            }
        }
    })(input)
}

fn parse_list_of_operators(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(space1, parse_op)(input)
}

fn consume_input(input: &str) -> IResult<&str, (Vec<Vec<u32>>, Vec<Op>)> {
    tuple((parse_block_of_numbers, parse_list_of_operators))(input)
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u32 {
    let (_remain, (numbers, ops)) = consume_input(input).expect("Failed to parse input");

    let num_cols = &numbers[0].len();
    let num_operators = ops.len();
    debug_assert_eq!(*num_cols, num_operators);
    let mut results = vec![];
    for col_idx in 0usize..*num_cols {
        let result = match ops[col_idx] {
            Op::Add => {
                let mut sum = 0;
                for row in &numbers {
                    sum += row[col_idx];
                }
                sum
            }
            Op::Mul => {
                let mut total = numbers[0][col_idx];
                for row in numbers.iter().skip(1) {
                    total *= row[col_idx];
                }
                total
            }
        };
        results.push(result);
    }
    dbg!(&results);
    results.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // line of numbers
    fn lon() {
        let input = "123 328  51 64
";
        assert_eq!(
            parse_line_of_numbers(input),
            Ok(("", vec![123, 328, 51, 64]))
        );
    }

    #[test]
    // Block of numbers
    fn bon() {
        let input = "123 328  51 64
  45 64  387 23
   6 98  215 314
*   +   *   +";

        let block_expected = vec![
            vec![123, 328, 51, 64],
            vec![45, 64, 387, 23],
            vec![6, 98, 215, 314],
        ];

        assert_eq!(
            parse_block_of_numbers(input),
            Ok(("*   +   *   +", block_expected))
        );
    }

    #[test]
    // Line of operators
    fn loo() {
        let input = "*   +   *   +";
        assert_eq!(
            parse_list_of_operators(input),
            Ok(("", vec![Op::Mul, Op::Add, Op::Mul, Op::Add]))
        );
    }

    #[test]
    fn p1() {
        let input = "123 328  51 64
    45 64  387 23
     6 98  215 314
*   +   *   +";

        assert_eq!(part1(input), 4277556);
    }
}
