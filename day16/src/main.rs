use bit_set::BitSet;
use petgraph::dot::{Config, Dot};
use petgraph::visit::Bfs;
use petgraph::Graph;
use regex::Regex;
use std::fmt;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
struct Valve {
    label: String,
    flow: u32,
    opened: bool,
}

impl fmt::Debug for Valve {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{} w={},O={}}}", self.label, self.flow, self.opened)
    }
}

fn parse_input(contents: &str) -> Graph<Valve, ()> {
    let mut valves = Graph::<Valve, ()>::new();

    let re = Regex::new(r"^Valve ([A-Z]{2}) [a-z =]+([0-9]+);[^A-Z]+(.*)").unwrap();
    for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let valve = Valve {
            label: label.to_string(),
            flow: caps.get(2).unwrap().as_str().parse().unwrap(),
            opened: false,
        };
        valves.add_node(valve);
    }

    for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let neighbors = caps.get(3).unwrap().as_str().replace(" ", "");

        for neighbor in neighbors.split(",") {
            let from = valves
                .node_indices()
                .find(|n| valves[*n].label == label)
                .unwrap();
            let to = valves
                .node_indices()
                .find(|n| valves[*n].label == neighbor)
                .unwrap();

            valves.add_edge(from, to, ());
        }
    }

    valves
}

fn main() {
    let contents = include_str!("test.txt");

    let mut valves = parse_input(contents);

    let start = valves
        .node_indices()
        .find(|idx| valves[*idx].label == "AA")
        .unwrap();

    let mut minutes = 30;

    let opened = BitSet::new();

    println!("start={:?}", start);

    // TODO: This is where we died on AOC, though we did learn how to use `petgraph` a bit, and it is a very nice library.
    while minutes > 0 {
        minutes -= 1;
    }
}
