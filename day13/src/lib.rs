use std::fmt::Debug;
use std::str::FromStr;

type Int = u32;
type List = Vec<Item>;

#[derive(Eq, PartialEq)]
enum Item {
    Int(Int),
    List(List),
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(arg0) => arg0.fmt(f),
            Self::List(arg0) => arg0.fmt(f),
        }
    }
}

fn parse_int(s: &str) -> (Int, &str) {
    let first_non_dig = s.find(|c: char| !c.is_digit(10));
    let (prefix, suffix) = match first_non_dig {
        Some(idx) => s.split_at(idx),
        None => (s, "")
    };
    (prefix.parse().unwrap(), suffix)
}

fn parse_list(s: &str) -> (List, &str) {
    let mut items = List::new();
    let mut s = s.strip_prefix('[').unwrap();
    loop {
        if let Some(suffix) = s.strip_prefix(']') {
            return (items, suffix)
        }
        let (item, suffix) = parse_item(s);
        items.push(item);
        s = suffix.strip_prefix(',').unwrap_or(suffix);
    }
}

fn parse_item(s: &str) -> (Item, &str) {
    if s.starts_with('[') {
        let (list, suffix) = parse_list(s);
        (Item::List(list), suffix)
    } else {
        let (i, suffix) = parse_int(s);
        (Item::Int(i), suffix)
    }
}

#[test]
fn test_parse_item() {
    assert_eq!(parse_item("42"), (Item::Int(42), ""));
    assert_eq!(parse_item("42,"), (Item::Int(42), ","));
    assert_eq!(parse_item("42]"), (Item::Int(42), "]"));
    assert_eq!(parse_item("[]"), (Item::List(vec![]), ""));
    assert_eq!(parse_item("[42]"), (Item::List(vec![Item::Int(42)]), ""));
    assert_eq!(parse_item("[42,43]"), (Item::List(vec![Item::Int(42), Item::Int(43)]), ""));
    assert_eq!(parse_item("[42,43,[44]]"), (Item::List(vec![Item::Int(42), Item::Int(43), Item::List(vec![Item::Int(44)])]), ""));

    assert_eq!(parse_item("[[1],[2,4]]"), (Item::List(vec![
        Item::List(vec![Item::Int(1)]),
        Item::List(vec![Item::Int(2),Item::Int(4)]),
    ]), ""));
    assert_eq!(parse_item("[[]]"), (Item::List(vec![Item::List(vec![])]), ""));
}

impl FromStr for Item {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (item, suffix) = parse_item(s);
        if !suffix.is_empty() {
            panic!("Unexpected suffix '{suffix}' parsing '{s}'");
        }
        Ok(item)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Int(i), Item::Int(j)) => i.partial_cmp(j),
            (Item::Int(i), Item::List(_)) => Item::List(vec![Item::Int(*i)]).partial_cmp(other),
            (Item::List(_), Item::Int(i)) => self.partial_cmp(&Item::List(vec![Item::Int(*i)])),
            (Item::List(l), Item::List(r)) => l.partial_cmp(r),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn test_compare_items() {
    assert!("42".parse::<Item>().unwrap() < "43".parse::<Item>().unwrap());
    assert!("[42]".parse::<Item>().unwrap() < "[43]".parse::<Item>().unwrap());
    assert!("[]".parse::<Item>().unwrap() < "[43]".parse::<Item>().unwrap());

    assert!("42".parse::<Item>().unwrap() < "[43]".parse::<Item>().unwrap());
    assert!("44".parse::<Item>().unwrap() > "[43]".parse::<Item>().unwrap());
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let pairs = input.split("\n\n");
        let pairs = pairs.map(|s| -> (Item, Item) {
            let (first, second) = s.split_once('\n').unwrap();
            (first.trim().parse().unwrap(), second.trim().parse().unwrap())
        });
        pairs.enumerate().filter_map(|(idx, pair)| if pair.0 <= pair.1 {Some(idx + 1)} else {None}).sum()
    }
    #[test]
    fn test_solve() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(solve(input), 13);
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        todo!()
    }
}