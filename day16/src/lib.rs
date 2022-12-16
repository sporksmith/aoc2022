use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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

pub mod p1 {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct StateKey<'a> {
        pos: Label<'a>,
        valves_enabled: BTreeSet<Label<'a>>,
    }

    fn next_states<'a>(
        nodes: &'a HashMap<Label, Node>,
        prev_states: BTreeMap<StateKey<'a>, Pressure>,
        rem: Minute,
    ) -> BTreeMap<StateKey<'a>, Pressure> {
        let mut next = BTreeMap::<StateKey, Pressure>::new();
        for (key, pressure) in prev_states {
            let node = &nodes[key.pos];
            let valves_enabled = &key.valves_enabled;
            if node.rate > 0 && !valves_enabled.contains(node.id) {
                // Consider turning on the current valve.
                let mut valves_enabled = valves_enabled.clone();
                valves_enabled.insert(node.id);
                let gain = node.rate * (rem - 1);
                let pressure = pressure + gain;
                let key = StateKey {
                    pos: node.id,
                    valves_enabled,
                };
                let prev_best = next.get(&key).copied().unwrap_or_default();
                if pressure > prev_best {
                    next.insert(key, pressure);
                }
            }
            for edge in &node.edges {
                // Explore path where we travel this edge
                let key = StateKey {
                    pos: *edge,
                    valves_enabled: valves_enabled.clone(),
                };
                let prev_best = next.get(&key).copied().unwrap_or_default();
                if pressure >= prev_best {
                    next.insert(key, pressure);
                }
            }
        }
        next
    }

    pub fn solve(input: &str) -> u32 {
        let nodes: Vec<Node> = input.lines().map(|s| Node::parse(s.trim())).collect();
        let nodes: HashMap<Label, Node> = nodes.into_iter().map(|n| (n.id, n)).collect();
        let mut states = BTreeMap::<StateKey, Pressure>::from([(
            StateKey {
                pos: "AA",
                valves_enabled: BTreeSet::new(),
            },
            0,
        )]);
        for rem in (1..=30).rev() {
            println!("{rem} minutes rem: {} states", states.len());
            states = next_states(&nodes, states, rem);
        }
        *states.values().max().unwrap()
    }
}

pub mod p2 {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct StateKey<'a> {
        pos: (Label<'a>, Label<'a>),
        valves_enabled: BTreeSet<Label<'a>>,
    }

    fn next_states<'a>(
        nodes: &'a HashMap<Label, Node>,
        prev_states: BTreeMap<StateKey<'a>, Pressure>,
        rem: Minute,
    ) -> BTreeMap<StateKey<'a>, Pressure> {
        let mut next = BTreeMap::<StateKey, Pressure>::new();
        let best_so_far = *prev_states.values().max().unwrap();
        for (key, pressure) in prev_states {
            let valves_enabled = &key.valves_enabled;
            let (pos1, pos2) = key.pos;
            let node1 = &nodes[pos1];
            let node2 = &nodes[pos2];
            for next1 in node1.edges.iter().chain(&["on"]).copied() {
                for next2 in node2.edges.iter().chain(&["on"]).copied() {
                    let mut next_pressure = pressure;
                    // Preemptively clone here for simplicity. If you remove this,
                    // be careful that both don't turn on the same valve below.
                    let mut valves_enabled = valves_enabled.clone();
                    let mut pos1 = pos1;
                    let mut pos2 = pos2;
                    if next1 == "on" && node1.rate > 0 && !valves_enabled.contains(pos1) {
                        next_pressure += node1.rate * (rem - 1);
                        valves_enabled.insert(pos1);
                    } else if next1 != "on" {
                        pos1 = next1;
                    }
                    if next2 == "on" && node2.rate > 0 && !valves_enabled.contains(pos2) {
                        next_pressure += node2.rate * (rem - 1);
                        valves_enabled.insert(pos2);
                    } else if next2 != "on" {
                        pos2 = next2;
                    }

                    let potential_future_gain: Pressure = {
                        let remaining_valve_values = nodes.values().filter_map(|node|
                            if valves_enabled.contains(node.id) {
                                None
                            } else {
                                Some(node.rate)
                            }
                        );

                        /*
                        .collect();
                        remaining_valve_values.sort();
                        let mut iter = remaining_valve_values.iter().rev();
                        let mut sum = 0;
                        if rem > 2 {
                            for rem in (1..(rem-2)).rev() {
                                if let Some(p) = iter.next() {
                                    sum += (rem - 1) * p;
                                } else {
                                    break;
                                }
                                if let Some(p) = iter.next() {
                                    sum += (rem - 1) * p;
                                } else {
                                    break;
                                }
                            }
                        }
                        sum
                        */
                        // First try bounding by if we could turn them all on at once
                        if rem <= 2 {
                            0
                        } else {
                            remaining_valve_values.sum::<Pressure>() * (rem - 2)
                        }
                    };
                    if next_pressure + potential_future_gain < best_so_far {
                        // Don't bother exploring
                        continue
                    }

                    // Sort positions
                    if pos1 > pos2 {
                        std::mem::swap(&mut pos1, &mut pos2);
                    }
                    let key = StateKey {
                        pos: (pos1, pos2),
                        valves_enabled,
                    };
                    let best_for_key = next.entry(key).or_default();
                    *best_for_key = std::cmp::max(*best_for_key, next_pressure);
                }
            }
        }
        next
    }

    pub fn solve(input: &str) -> u32 {
        let nodes: Vec<Node> = input.lines().map(|s| Node::parse(s.trim())).collect();
        let nodes: HashMap<Label, Node> = nodes.into_iter().map(|n| (n.id, n)).collect();
        let mut states = BTreeMap::<StateKey, Pressure>::from([(
            StateKey {
                pos: ("AA", "AA"),
                valves_enabled: BTreeSet::new(),
            },
            0,
        )]);
        for rem in (1..=26).rev() {
            println!("{rem} minutes rem: {} states", states.len());
            states = next_states(&nodes, states, rem);
        }
        *states.values().max().unwrap()
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

    #[test]
    fn test_p2() {
        assert_eq!(p2::solve(INPUT), 1707);
    }
}
