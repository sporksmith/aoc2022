use std::collections::HashSet;
use std::str::FromStr;

type Minute = u32;
type Label<'a> = &'a str;
type Pressure = u32;

#[derive(Debug, Eq, PartialEq)]
struct Node<'a> {
    id: Label<'a>,
    rate: u32,
    edges: Vec<Label<'a>>,
}

impl<'a> Node<'a> {
    fn parse(s: &'a str) -> Self {
        let s = s.strip_prefix("Valve ").unwrap();
        let (id, s) = s.split_once(' ').unwrap();
        let s = s.strip_prefix("has flow rate=").unwrap();
        let (rate, s) = s.split_once("; ").unwrap();
        let rate = rate.parse().unwrap();
        let s = s.trim_start_matches("tunnels lead to valves ");
        let s = s.trim_start_matches("tunnel leads to valve ");
        let s = s.trim_start();
        let edges = s.split(", ").collect();
        Node { id, rate, edges }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct StateKey<'a> {
    time_remaining: Minute,
    pos: Label<'a>,
    valves_enabled: HashSet<Label<'a>>,
    pressure_released: Pressure,
}

/*
impl<'a> Hash for StateKey<'a> {

}
*/

pub mod p1 {
    use std::collections::{HashMap, HashSet};

    use super::*;

    fn max_addtl_possible(
        nodes: &HashMap<Label, Node>,
        already_enabled: &HashSet<Label>,
        pos: Label,
        time_remaining: Minute,
    ) -> u32 {
        if time_remaining == 0 {
            return 0;
        }
        let mut best = 0;
        let node = &nodes[pos];
        if node.rate > 0 && !already_enabled.contains(pos) {
            // Explore path where we turn it on
            let mut already_enabled = already_enabled.clone();
            already_enabled.insert(pos);
            let score = node.rate * (time_remaining - 1)
                + max_addtl_possible(nodes, &already_enabled, pos, time_remaining - 1);
            best = std::cmp::max(score, best);
        }
        for edge in &node.edges {
            // Explore path where we travel this edge
            let score = max_addtl_possible(nodes, already_enabled, *edge, time_remaining - 1);
            best = std::cmp::max(score, best);
        }
        best
    }

    pub fn solve(input: &str) -> u32 {
        let nodes: Vec<Node> = input.lines().map(|s| Node::parse(s.trim())).collect();
        let nodes: HashMap<Label, Node> = nodes.into_iter().map(|n| (n.id, n)).collect();
        max_addtl_possible(&nodes, &HashSet::new(), "AA", 30)
    }
}

pub mod p2 {
    use std::collections::{HashMap, HashSet};

    use super::*;

    pub fn solve(input: &str) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Node::parse("Valve AW has flow rate=0; tunnels lead to valves LG, TL"),
            Node {
                id: "AW",
                rate: 0,
                edges: vec!["LG", "TL"]
            }
        );
        assert_eq!(
            Node::parse("Valve HH has flow rate=22; tunnel leads to valve GG"),
            Node {
                id: "HH",
                rate: 22,
                edges: vec!["GG"]
            }
        );
    }

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_p1() {
        assert_eq!(p1::solve(INPUT), 1651);
    }
}
