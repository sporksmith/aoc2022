use std::str::FromStr;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rev_lines = s.lines().rev();
        let numbers_line = rev_lines.next().unwrap();
        let num_stacks = numbers_line.trim().split_whitespace().count();

        let mut stacks = Vec::new();
        stacks.resize_with(num_stacks, Vec::new);

        while let Some(line) = rev_lines.next() {
            let mut chars = line.chars();
            for stack_idx in 0..num_stacks {
                let c = chars.next().unwrap();
                if c == ' ' {
                    assert_eq!(chars.next().unwrap(), ' ');
                    assert_eq!(chars.next().unwrap(), ' ');
                    chars.next();
                    continue;
                }
                assert_eq!(c, '[');
                let item = chars.next().unwrap();
                assert!(item.is_alphabetic());
                stacks[stack_idx].push(item);
                assert_eq!(chars.next().unwrap(), ']');
                // Eat whitespace if we're not at end
                chars.next();
            }
        }
        Ok(Stacks { stacks })
    }
}
#[test]
fn test_parse_stacks() {
    let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;
    let stacks: Stacks = input.parse().unwrap();
    assert_eq!(stacks.stacks[0], vec!['Z', 'N']);
    assert_eq!(stacks.stacks[1], vec!['M', 'C', 'D']);
    assert_eq!(stacks.stacks[2], vec!['P']);
}

impl Stacks {
    pub fn execute(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.count {
            let item = self.stacks[instruction.src - 1].pop().unwrap();
            self.stacks[instruction.dst - 1].push(item);
        }
    }

    pub fn execute9001(&mut self, i: &Instruction) {
        let start_idx = self.stacks[i.src - 1].len() - i.count;
        let items = self.stacks[i.src - 1].split_off(start_idx);
        self.stacks[i.dst - 1].extend(items);
    }

    pub fn message(&self) -> String {
        let mut chars = String::new();
        for stack in &self.stacks {
            chars.push(stack[stack.len() - 1]);
        }
        chars
    }
}

struct Instruction {
    pub count: usize,
    pub src: usize,
    pub dst: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        assert_eq!(tokens.next(), Some("move"));
        let count = tokens.next().unwrap().parse().unwrap();
        assert_eq!(tokens.next(), Some("from"));
        let src = tokens.next().unwrap().parse().unwrap();
        assert_eq!(tokens.next(), Some("to"));
        let dst = tokens.next().unwrap().parse().unwrap();
        Ok(Instruction { count, src, dst })
    }
}

fn parse_input(input: &str) -> (Stacks, Vec<Instruction>) {
    let mut parts = input.split("\n\n");
    let diagram = parts.next().unwrap();
    let instructions = parts.next().unwrap();
    assert!(parts.next().is_none());

    let stacks = diagram.parse().unwrap();
    let instructions = instructions.lines().map(|l| l.parse().unwrap()).collect();
    (stacks, instructions)
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> String {
        let (mut stacks, instructions) = parse_input(input);
        for i in &instructions {
            stacks.execute(i);
        }
        stacks.message()
    }
    #[test]
    fn test_solve() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve(input), "CMZ");
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> String {
        let (mut stacks, instructions) = parse_input(input);
        for i in &instructions {
            stacks.execute9001(i);
        }
        stacks.message()
    }
    #[test]
    fn test_solve() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve(input), "MCD");
    }
}
