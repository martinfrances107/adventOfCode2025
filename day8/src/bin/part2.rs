//! Extension Coord
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::collections::{HashSet, LinkedList};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input, 1000000));
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    // length squared
    const fn len2(&self, other: &Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn part2(input: &str, loop_count_max: usize) -> i64 {
    let mut boxes: HashSet<Box> = HashSet::default();
    let mut circuits: LinkedList<Vec<Box>> = LinkedList::default();
    for line in input.lines() {
        let mut splitter = line.split(',');

        let x = splitter.next().unwrap().parse().unwrap();
        let y = splitter.next().unwrap().parse().unwrap();
        let z = splitter.next().unwrap().parse().unwrap();
        boxes.insert(Box { x, y, z });
    }

    let mut last_pair = None;

    // closet first
    for (loop_count, (a, b, _dist_)) in boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.len2(b)))
        .sorted_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
        .enumerate()
    {
        // look for a in existing circuits.
        // look for b in existing circuits.
        let mut a_in_circuit = None;
        let mut b_in_circuit = None;
        for (circuit_id, circuit) in circuits.iter().enumerate() {
            if circuit.contains(a) {
                a_in_circuit = Some(circuit_id);
            }
            if circuit.contains(b) {
                b_in_circuit = Some(circuit_id);
            }
        }
        // println!("founds circuits {a_in_circuit:?} {b_in_circuit:?}");

        // (1) if a and b are aleady in the the same circuit, no action.
        // (2) if a and b are in distinct circuits then merge circuits.
        // (3) is a already connected and b is unconnected add b to a's circuit
        // (4) is b already connected and a is unconnected add a to b's circuit
        // (5) if a and b are unconnected make a new circuit.
        match (a_in_circuit, b_in_circuit) {
            (None, None) => {
                // println!("Making a new circuit from two unconnected");

                circuits.push_back(vec![a.clone(), b.clone()]);
            }
            (None, Some(cir_b)) => {
                // println!("Adding a to b's exiting circuit");
                // Walk the list of circuits dropping off a
                circuits
                    .iter_mut()
                    .enumerate()
                    .position(|(pos, circuit)| {
                        if pos == cir_b {
                            circuit.push(a.clone());
                            true
                        } else {
                            false
                        }
                    })
                    // Panic The require circuit was not found
                    .unwrap();
                last_pair = Some((a.clone(), b.clone()));
                // connection_count += 1;
            }
            (Some(cir_a), None) => {
                // println!("Adding b to a's exiting circuit");
                // Walk the list dof circuits dropping off b
                circuits
                    .iter_mut()
                    .enumerate()
                    .position(|(pos, circuit)| {
                        if pos == cir_a {
                            circuit.push(b.clone());
                            true
                        } else {
                            false
                        }
                    })
                    // Panic The require circuit was not found
                    .unwrap();
                last_pair = Some((a.clone(), b.clone()));
            }
            (Some(cir_a), Some(cir_b)) => {
                // TODO rethink this looking for elements in circuits is expensive, I am doing this a total of 4 times!!
                if cir_a == cir_b {
                    // println!("Doing nothing");
                } else {
                    // println!("Merging circuits");
                    let mut frag_a_iter = circuits.extract_if(|c| c.contains(a));
                    let mut frag_a = frag_a_iter.next().unwrap();
                    debug_assert_eq!(frag_a_iter.next(), None);

                    // Extract circuit containing b
                    let mut frag_b_iter = circuits.extract_if(|c| c.contains(b));
                    let mut frag_b = frag_b_iter.next().unwrap();
                    debug_assert_eq!(frag_b_iter.next(), None);

                    frag_a.append(&mut frag_b);

                    circuits.push_back(frag_a);
                    // Joining two circuits, is still a new connection.
                    last_pair = Some((a.clone(), b.clone()));
                }
            }
        }

        if loop_count == loop_count_max {
            println!("{loop_count}");
            panic!("too much counting");
        }
    }

    let Some((a, b)) = last_pair else {
        panic!("did not find a pair");
    };
    println!("last {a:#?}, {b:#?}");

    a.x * b.x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Block of numbers
    fn connect2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

        assert_eq!(part2(input, 10_000), 25272);
    }
}
