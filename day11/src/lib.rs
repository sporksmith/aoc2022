use std::cell::RefCell;
use std::collections::BTreeMap;
use std::str::FromStr;

type MonkeyIdx = usize;
type Worry = u64;

#[derive(Eq, PartialEq, Debug)]
struct Monkey {
    items: Vec<Worry>,
    operation: Operation,
    test_divisor: Worry,
    true_dst: MonkeyIdx,
    false_dst: MonkeyIdx,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        assert!(lines.next().unwrap().trim().starts_with("Monkey "));

        let items: Vec<Worry> = {
            let mut items = Vec::new();
            let line = lines.next().unwrap();
            let items_str = line.trim().strip_prefix("Starting items: ").unwrap();
            let items_tokens = items_str.split(", ");
            for t in items_tokens {
                items.push(t.parse::<Worry>().unwrap());
            }
            items
        };

        let operation: Operation = {
            let line = lines.next().unwrap();
            let suffix = line.trim().strip_prefix("Operation: new = old ").unwrap();
            let (operator_str, operand_str) = suffix.split_once(' ').unwrap();
            match operator_str {
                "+" => {
                    let operand: Worry = operand_str.parse().unwrap();
                    Operation::Add(operand)
                }
                "*" => {
                    if operand_str == "old" {
                        Operation::Square
                    } else {
                        let operand: Worry = operand_str.parse().unwrap();
                        Operation::Mul(operand)
                    }
                }
                _ => {
                    panic!("Unexpected operator {operator_str}");
                }
            }
        };

        let test_divisor: Worry = {
            let line = lines.next().unwrap();
            let suffix = line.trim().strip_prefix("Test: divisible by ").unwrap();
            suffix.parse().unwrap()
        };

        let true_dst: MonkeyIdx = {
            let line = lines.next().unwrap();
            let suffix = line
                .trim()
                .strip_prefix("If true: throw to monkey ")
                .unwrap();
            suffix.parse().unwrap()
        };

        let false_dst: MonkeyIdx = {
            let line = lines.next().unwrap();
            let suffix = line
                .trim()
                .strip_prefix("If false: throw to monkey ")
                .unwrap();
            suffix.parse().unwrap()
        };

        Ok(Monkey {
            items,
            operation,
            test_divisor,
            true_dst,
            false_dst,
        })
    }
}

#[test]
fn test_parse_monkey() {
    let input = "Monkey 0:
    Starting items: 59, 65
    Operation: new = old * 17
    Test: divisible by 3
      If true: throw to monkey 3
      If false: throw to monkey 6";
    assert_eq!(
        input.parse::<Monkey>().unwrap(),
        Monkey {
            items: vec![59, 65],
            operation: Operation::Mul(17),
            test_divisor: 3,
            true_dst: 3,
            false_dst: 6
        }
    )
}

#[derive(Eq, PartialEq, Debug)]
enum Operation {
    Add(Worry),
    Mul(Worry),
    Square,
}

impl Operation {
    fn operate(&self, w: Worry) -> Worry {
        match self {
            Operation::Add(i) => w.checked_add(*i).unwrap(),
            Operation::Mul(i) => w.checked_mul(*i).unwrap(),
            Operation::Square => w.checked_mul(w).unwrap(),
        }
    }
}

fn monkey_business(monkey_string: &str, nrounds: usize, do_divide: bool) -> usize {
    let monkeys: Vec<RefCell<Monkey>> = monkey_string
        .split("\n\n")
        .map(|l| RefCell::new(l.parse::<Monkey>().unwrap()))
        .collect();
    // We only need to keep track of whether the worry-level is divisible by all the monkey divisors.
    // We can operate in a group that's the product of those divisors.
    let divisor_product: Worry = monkeys.iter().map(|m| m.borrow().test_divisor).product();
    let mut item_inspection_counts: BTreeMap<MonkeyIdx, usize> = BTreeMap::new();

    for _round in 0..nrounds {
        for midx in 0..monkeys.len() {
            let mut monkey = monkeys[midx].borrow_mut();
            *item_inspection_counts.entry(midx).or_default() += monkey.items.len();
            for worry in std::mem::replace(&mut monkey.items, Vec::new()).drain(..) {
                let mut new_worry = monkey.operation.operate(worry);
                if do_divide {
                    new_worry /= 3;
                }
                new_worry %= divisor_product;
                let dst = if new_worry % monkey.test_divisor == 0 {
                    monkey.true_dst
                } else {
                    monkey.false_dst
                };
                // println!("Monkey {midx} worry {worry}->{new_worry} throws to {dst}");
                monkeys[dst].borrow_mut().items.push(new_worry);
            }
        }
    }
    let mut counts: Vec<usize> = item_inspection_counts.into_values().collect();
    counts.sort_by_key(|i| std::cmp::Reverse(*i));
    counts[0].checked_mul(counts[1]).unwrap()
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        monkey_business(input, 20, true)
    }
    #[test]
    fn test_solve() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3

      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0

      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3

      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";
        assert_eq!(solve(input), 10605);
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        monkey_business(input, 10_000, false)
    }
    #[test]
    fn test_solve() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3

      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0

      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3

      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";
        assert_eq!(solve(input), 2713310158);
    }
}
