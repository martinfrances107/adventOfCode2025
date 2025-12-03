//! day 3
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

fn calculate_joltage(bank: &[u64]) -> u64 {
    let mut digits: Vec<u64> = vec![];

    let mut pos = 0;
    for num_remaining_digits in (0..12).rev() {
        let window_size = bank.len() - pos - num_remaining_digits;
        let active_slice = &bank[pos..pos + window_size];
        let (pos_within_window, digit) = largest_in_range(active_slice);

        // Advance the head by the found position within the window,  the next start pos is
        // one beyound point.
        pos += pos_within_window + 1;
        digits.push(digit);
    }

    let mut joltage = 0;
    let mut multiplier = 1u64;
    for d in digits.iter().rev() {
        joltage += d * multiplier;
        multiplier *= 10;
    }

    joltage
}

fn largest_in_range(bank: &[u64]) -> (usize, u64) {
    let mut max = u64::MIN;
    let mut pos = 0;
    for (index, d) in bank.iter().enumerate() {
        if *d > max {
            max = *d;
            pos = index;
        }
    }
    (pos, max)
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut bank = vec![];
        for c in line.chars() {
            let digit: u64 = u64::from(c.to_digit(10).unwrap());
            bank.push(digit);
        }
        let joltage = calculate_joltage(&bank);
        sum += joltage;
    }

    sum
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn max_joltage() {
        let cases = vec![
            (
                [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                987654321111u64,
            ),
            ([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 811111111119),
            ([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 434234234278),
            ([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 888911112111),
        ];

        for (bank, joltage_expected) in cases {
            assert_eq!(calculate_joltage(&bank), joltage_expected);
        }
    }

    #[test]
    fn sum() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(part2(&input), 3121910778619);
    }
}
