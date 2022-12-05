use std::collections::BTreeSet;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        u32::from(c) - u32::from('A') + 27
    }
}
#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('b'), 2);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('B'), 28);
}

type Knapsack = Vec<char>;
fn parse_line(line: &str) -> Knapsack {
    line.chars().collect()
}
#[test]
fn test_parse_line() {
    assert_eq!(parse_line("abcd"), vec!['a', 'b', 'c', 'd']);
}

pub mod p1 {
    use super::*;

    type Compartments = (BTreeSet<char>, BTreeSet<char>);
    fn knapsack_compartments(knapsack: &Knapsack) -> Compartments {
        let (front, back) = knapsack.split_at(knapsack.len() / 2);
        assert_eq!(front.len(), back.len());
        (
            front.iter().copied().collect(),
            back.iter().copied().collect(),
        )
    }

    fn dupe_item(c: &Compartments) -> char {
        c.0.intersection(&c.1).copied().next().unwrap()
    }
    #[test]
    fn test_dupe_item() {
        assert_eq!(
            dupe_item(&(
                BTreeSet::from(['a', 'b', 'c']),
                BTreeSet::from(['c', 'd', 'e'])
            )),
            'c'
        );
    }

    pub fn solve(input: &str) -> u32 {
        input
            .trim()
            .lines()
            .map(parse_line)
            .map(|k| knapsack_compartments(&k))
            .map(|c| dupe_item(&c))
            .map(priority)
            .sum()
    }
    #[test]
    fn test_solve() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(solve(input), 157);
    }
}

pub mod p2 {
    use super::*;

    type KnapsackSet = BTreeSet<char>;
    fn knapsack_set(knapsack: &Knapsack) -> KnapsackSet {
        knapsack.iter().copied().collect()
    }

    fn groups(mut knapsacks: impl Iterator<Item = KnapsackSet>) -> Vec<[KnapsackSet; 3]> {
        let mut res = Vec::new();
        loop {
            let Some(first) = knapsacks.next() else {
                break;
            };
            let second = knapsacks.next().unwrap();
            let third = knapsacks.next().unwrap();
            res.push([first, second, third]);
        }
        res
    }

    fn badge_for_group(group: &[KnapsackSet; 3]) -> char {
        group[0]
            .intersection(&group[1])
            .copied()
            .collect::<KnapsackSet>()
            .intersection(&group[2])
            .copied()
            .next()
            .unwrap()
    }

    pub fn solve(input: &str) -> u32 {
        let knapsacks = input.trim().lines().map(parse_line);
        let knapsack_sets = knapsacks.map(|k| knapsack_set(&k));
        let groups = groups(knapsack_sets);
        let badges = groups.iter().map(|g| badge_for_group(&g));
        let priorities = badges.map(priority);
        priorities.sum()
    }
    #[test]
    fn test_solve() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(solve(input), 70);
    }
}
