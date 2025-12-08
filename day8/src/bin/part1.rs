//! Beam Splitter
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::collections::{HashSet, LinkedList};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input, 1000));
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

fn part1(input: &str, num_connections: usize) -> usize {
    let mut boxes: HashSet<Box> = HashSet::default();
    let mut circuits: LinkedList<Vec<Box>> = LinkedList::default();
    for line in input.lines() {
        let mut splitter = line.split(',');

        let x = splitter.next().unwrap().parse().unwrap();
        let y = splitter.next().unwrap().parse().unwrap();
        let z = splitter.next().unwrap().parse().unwrap();
        boxes.insert(Box { x, y, z });
    }

    let mut loop_count = 0;
    let mut connection_count = 0;
    let mut prev_min2 = 0;
    'connection_loop: loop {
        let mut pair_candidate: Option<(Box, Box)> = None;
        let mut dist2_min = i64::MAX;
        for a in boxes.iter() {
            for b in &boxes {
                let d2 = a.len2(b);
                if *a != *b && d2 > prev_min2 && d2 < dist2_min {
                    dist2_min = d2;
                    pair_candidate = Some((a.clone(), b.clone()));
                }
            }
        }

        let Some((a, b)) = pair_candidate else {
            println!("no more pairs to be found");
            break 'connection_loop;
        };

        // Now have a pairing (a, b)
        prev_min2 = dist2_min;

        // println!("found next pair {:?} {:?}", a, b);

        // look for a in existing circuits.
        // look for b in existing circuits.
        let mut a_in_circuit = None;
        let mut b_in_circuit = None;
        for (circuit_id, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&a) {
                a_in_circuit = Some(circuit_id);
            }
            if circuit.contains(&b) {
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

                circuits.push_back(vec![a, b]);
                connection_count += 1;
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
                connection_count += 1;
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
                connection_count += 1;
            }
            (Some(cir_a), Some(cir_b)) => {
                // TODO rethink this looking for elements in circuits is expensive, I am doing this a total of 4 times!!
                if cir_a == cir_b {
                    // println!("Doing nothing");
                    connection_count += 1;
                } else {
                    // println!("Merging circuits");
                    let mut frag_a_iter = circuits.extract_if(|c| c.contains(&a));
                    let mut frag_a = frag_a_iter.next().unwrap();
                    debug_assert_eq!(frag_a_iter.next(), None);

                    // Extract circuit containing b
                    let mut frag_b_iter = circuits.extract_if(|c| c.contains(&b));
                    let mut frag_b = frag_b_iter.next().unwrap();
                    debug_assert_eq!(frag_b_iter.next(), None);

                    frag_a.append(&mut frag_b);

                    circuits.push_back(frag_a);
                    // Joining two circuits, is still a new connection.
                    connection_count += 1;
                }
            }
        }

        // print!("circuits ");
        // for c in &circuits {
        //     print!("{} ", c.len());
        // }
        // println!();
        // println!("at end of loop num connections {connection_count}");
        // println!();
        if connection_count == num_connections {
            break 'connection_loop;
        }

        if loop_count == num_connections {
            panic!("too much counting");
        }
        loop_count += 1;
    }

    // println!("Final circuits {circuits:#?}");

    let total_nodes = boxes.len();
    let mut connected_nodes = 0;
    let mut circuit_lengths = vec![];
    for cir in &circuits {
        connected_nodes += cir.len();
        circuit_lengths.push(cir.len());
    }
    let isolated_boxes = total_nodes - connected_nodes;

    circuit_lengths.sort();
    circuit_lengths.reverse();

    let top_product: usize = circuit_lengths.iter().take(3).product();
    // println!("top_sum {top_product}");
    // println!("circuit lengths {circuit_lengths:#?}");
    // println!("isolated boxes {connected_nodes}");
    // println!("num circuit {isolated_boxes}");
    top_product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Block of numbers
    fn connect() {
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

        assert_eq!(part1(input, 10), 40);
    }
}
