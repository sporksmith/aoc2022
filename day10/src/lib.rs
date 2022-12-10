use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Nop,
    Add(i64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let t1 = s.next().unwrap();
        match t1 {
            "addx" => {
                let i: i64 = s.next().unwrap().parse().unwrap();
                Ok(Instruction::Add(i))
            }
            "noop" => Ok(Instruction::Nop),
            _ => panic!("Bad token {t1}"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    x: i64,
    cycles_executed: usize,
}

fn execute(state: State, inst: Instruction) -> State {
    match inst {
        Instruction::Nop => State {
            x: state.x,
            cycles_executed: state.cycles_executed + 1,
        },
        Instruction::Add(i) => State {
            x: state.x + i,
            cycles_executed: state.cycles_executed + 2,
        },
    }
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> i64 {
        let instructions = input
            .lines()
            .map(|l| l.trim().parse::<Instruction>().unwrap());
        let mut pending_cycle_counts = vec![220, 180, 140, 100, 60, 20];
        let mut sum = 0;
        let mut state = State {
            x: 1,
            cycles_executed: 0,
        };
        for inst in instructions {
            let next_state = execute(state, inst);
            let next_cycle_count = pending_cycle_counts[pending_cycle_counts.len() - 1];
            if next_state.cycles_executed >= next_cycle_count {
                sum += i64::try_from(next_cycle_count).unwrap() * state.x;
                pending_cycle_counts.pop();
                if pending_cycle_counts.is_empty() {
                    break;
                }
            }
            state = next_state;
        }
        sum
    }

    #[test]
    fn test_solve() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(solve(input), 13140);
    }
}
