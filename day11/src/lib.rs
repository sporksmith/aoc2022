use std::str::FromStr;

type MonkeyIdx = usize;
type Worry = u32;

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
        assert!(lines.next().unwrap().starts_with("Monkey "));

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
