//! Button Smasher
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]

use std::collections::HashMap;

use pathfinding::prelude::count_paths;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part2(input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Node<'a> {
    name: &'a str,
    seen_fft: bool,
    seen_dac: bool,
}

fn gen_nodes(input: &str) -> HashMap<&str, Vec<&str>> {
    // First pass generate list of all nodes.
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::default();
    for line in input.lines() {
        let (name, connections) = line.split_once(": ").unwrap();

        let connections = connections.split(' ').collect::<Vec<_>>();
        nodes.insert(name, connections);
    }
    nodes.insert("out", vec![]);
    nodes
}

fn part2(input: &str) -> usize {
    let nodes = gen_nodes(input);

    let svr = Node {
        name: "svr",
        seen_fft: false,
        seen_dac: false,
    };

    count_paths(
        svr,
        |&node| {
            nodes[node.name].iter().map(move |c_name| Node {
                name: c_name,
                seen_fft: node.seen_fft || *c_name == "fft",
                seen_dac: node.seen_dac || *c_name == "dac",
            })
        },
        |&n| n.name == "out" && n.seen_dac & n.seen_fft,
    )
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_fp() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        assert_eq!(part2(input), 2usize);
    }
}
