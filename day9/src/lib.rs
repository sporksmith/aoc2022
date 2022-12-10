use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        Ok(match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Unrecognized dir {s}"),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    dir: Dir,
    count: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s.split_once(' ').unwrap();
        Ok(Instruction {
            dir: dir.parse().unwrap(),
            count: count.parse().unwrap(),
        })
    }
}

fn head_pos(prev: (i32, i32), dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Up => (prev.0, prev.1 + 1),
        Dir::Down => (prev.0, prev.1 - 1),
        Dir::Left => (prev.0 - 1, prev.1),
        Dir::Right => (prev.0 + 1, prev.1),
    }
}
#[test]
fn test_head_pos() {
    assert_eq!(head_pos((0, 0), Dir::Left), (-1, 0));
    assert_eq!(head_pos((0, 0), Dir::Right), (1, 0));
    assert_eq!(head_pos((0, 0), Dir::Up), (0, 1));
    assert_eq!(head_pos((0, 0), Dir::Down), (0, -1));
}

fn tail_pos(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let diff = (head.0 - tail.0, head.1 - tail.1);
    let diff_sign = (diff.0.signum(), diff.1.signum());
    let diff_mag = (diff.0.abs(), diff.1.abs());
    assert!(diff_mag.0 <= 2);
    assert!(diff_mag.1 <= 2);
    if diff_mag.0 < 2 && diff_mag.1 < 2 {
        (tail.0, tail.1)
    } else {
        (tail.0 + diff_sign.0, tail.1 + diff_sign.1)
    }
}
#[test]
fn test_tail_pos() {
    assert_eq!(tail_pos((0, 0), (0, 0)), (0, 0));
    assert_eq!(tail_pos((0, 0), (0, 2)), (0, 1));
    assert_eq!(tail_pos((0, 0), (0, -2)), (0, -1));
    assert_eq!(tail_pos((0, 0), (1, 1)), (0, 0));
    assert_eq!(tail_pos((0, 0), (2, 2)), (1, 1));
    assert_eq!(tail_pos((0, 0), (1, 2)), (1, 1));
}

pub mod p1 {
    use std::collections::BTreeSet;

    use super::*;
    pub fn solve(input: &str) -> usize {
        let instructions = input
            .lines()
            .map(|l| -> Instruction { l.trim().parse().unwrap() });
        let mut head = (0i32, 0i32);
        let mut tail = (0i32, 0i32);
        let mut positions = BTreeSet::<(i32, i32)>::new();
        positions.insert(tail);
        for i in instructions {
            for _ in 0..i.count {
                head = head_pos(head, i.dir);
                tail = tail_pos(tail, head);
                positions.insert(tail);
            }
        }
        positions.len()
    }

    #[test]
    fn test_solve() {
        let input = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";
        assert_eq!(solve(input), 13);
    }
}

pub mod p2 {
    use std::collections::BTreeSet;

    use super::*;
    pub fn solve(input: &str) -> usize {
        let instructions = input
            .lines()
            .map(|l| -> Instruction { l.trim().parse().unwrap() });
        let mut knots = [(0i32, 0i32); 10];
        let mut positions = BTreeSet::<(i32, i32)>::new();
        positions.insert(knots[9]);
        for i in instructions {
            for _ in 0..i.count {
                knots[0] = head_pos(knots[0], i.dir);
                for i in 1..knots.len() {
                    knots[i] = tail_pos(knots[i], knots[i-1]);
                }
                positions.insert(knots[9]);
            }
        }
        positions.len()
    }

    #[test]
    fn test_solve() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(solve(input), 36);
    }
}
