//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Node<'a> {
    connections: Vec<&'a str>,
}

struct Nodes<'a>(HashMap<&'a str, Node<'a>>);

impl<'a> Nodes<'a> {
    fn gen_nodes(input: &'a str) -> Self {
        // First pass generate list of all nodes.
        let mut nodes: HashMap<&'a str, Node> = HashMap::default();
        for line in input.lines() {
            let (name, connections) = line.split_once(": ").unwrap();

            let connections = connections.split(' ').collect::<Vec<_>>();
            nodes.insert(name, Node { connections });
        }
        Nodes(nodes)
    }

    fn get_mut(&mut self, name: &'a str) -> Option<&mut Node<'a>> {
        self.0.get_mut(name)
    }
}

fn part1(input: &str) -> usize {
    // Fragments has run from "you" to "out".
    let mut champions = vec![];
    let mut nodes = Nodes::gen_nodes(input);

    // "you" must exit in the list of nodes.
    let root_node = &mut nodes.get_mut("you").unwrap().clone();

    // Fragments are distinct paths where not loop cycle has been detected but also have not completed.
    // Initial fragments are one hop form "you".
    // when loop is detected delete the fragment.
    let mut fragments = root_node
        .connections
        .iter()
        .map(|name| vec!["you", name])
        .collect::<Vec<_>>();

    let mut count = 0;
    'candidate_loop: loop {
        // Advance the fragment by one node until
        // it get out whereby it is moved to the list of champions
        // if a loop is detected do not pass onto the next generation
        let mut fragments_next = vec![];
        for fragment in &fragments {
            let name = fragment.last().unwrap();
            // Node Under Test.
            if *name == "out" {
                // path is complete.. move to the list of champions
                let mut champion_new = fragment.clone();
                champion_new.push(name);
                champions.push(champion_new);
            } else {
                let last_node = nodes.get_mut(name).unwrap();

                let next_connections = &last_node.connections;
                // if there are 3 connections here potentially spawn 3 new fragments.
                // only spawn if no loop is detected.
                for nc in next_connections {
                    // path is complete.. move to the list of champions
                    // implicit drop by not moving onto the next generation
                    if !fragment.contains(nc) {
                        // loop detection implicit drop by not moving onto the next generation.
                        let mut fragment_new = fragment.clone();
                        fragment_new.push(nc);
                        fragments_next.push(fragment_new);
                    }
                }
                if fragments_next.is_empty() {
                    // the next generation of fragment is empty
                    // so all path have become champion or are deleeted.
                    break 'candidate_loop;
                }
            }
        }

        fragments = fragments_next;

        if count > 1000 {
            break 'candidate_loop;
        }
        count += 1;
    }
    // for c in &champions {
    //     for path in c {
    //         print!("{path} ");
    //     }
    //     println!();
    // }
    champions.len()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        assert_eq!(part1(input), 5usize);
    }
}
