use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug, Clone)]
struct Assignment {
    pub range: RangeInclusive<u32>,
}
impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let begin = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        assert!(parts.next().is_none());
        Ok(Assignment { range: begin..=end })
    }
}

#[derive(Debug, Clone)]
struct AssignmentPair {
    pub a1: Assignment,
    pub a2: Assignment,
}
impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut assignments = s.split(',');
        let a1: Assignment = assignments.next().unwrap().parse().unwrap();
        let a2: Assignment = assignments.next().unwrap().parse().unwrap();
        Ok(AssignmentPair { a1, a2 })
    }
}

impl AssignmentPair {
    pub fn is_completely_redundant(&self) -> bool {
        (self.a1.range.contains(self.a2.range.start())
            && self.a1.range.contains(self.a2.range.end()))
            || (self.a2.range.contains(self.a1.range.start())
                && self.a2.range.contains(self.a1.range.end()))
    }

    pub fn is_partly_redundant(&self) -> bool {
        self.a1.range.contains(self.a2.range.start())
        || self.a1.range.contains(self.a2.range.end())
        || self.a2.range.contains(self.a1.range.start())
        || self.a2.range.contains(self.a1.range.end())
    }
}

pub mod p1 {
    use super::*;

    pub fn solve(s: &str) -> usize {
        let pairs = s.lines().map(|l| l.parse::<AssignmentPair>().unwrap());
        let filtered_pairs = pairs.filter(AssignmentPair::is_completely_redundant);
        filtered_pairs.count()
    }

    #[test]
    fn test_solve() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve(input), 2);
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(s: &str) -> usize {
        let pairs = s.lines().map(|l| l.parse::<AssignmentPair>().unwrap());
        let filtered_pairs = pairs.filter(AssignmentPair::is_partly_redundant);
        filtered_pairs.count()
    }

    #[test]
    fn test_solve() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve(input), 4);
    }
}
