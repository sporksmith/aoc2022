#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn outcome_vs(&self, other: &Self) -> Outcome {
        use Outcome::*;
        use RPS::*;
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    pub fn score_vs(&self, other: &Self) -> u32 {
        self.score() + self.outcome_vs(other).score()
    }

    pub fn to_throw_for_outcome(other: RPS, outcome: &Outcome) -> RPS {
        use Outcome::*;
        use RPS::*;
        match (other, outcome) {
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Rock, Draw) => Rock,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

pub mod p1 {
    use super::*;

    pub fn parse_line(line: &str) -> (RPS, RPS) {
        use RPS::*;
        let mut chars = line.split_whitespace();
        let their_char = chars.next().unwrap().chars().next().unwrap();
        let my_char = chars.next().unwrap().chars().next().unwrap();
        let theirs = match their_char {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Bad char {their_char}"),
        };
        let mine = match my_char {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("Bad char {my_char}"),
        };
        (theirs, mine)
    }

    #[test]
    fn test_parse_line() {
        use RPS::*;
        assert_eq!(parse_line("A Y"), (Rock, Paper));
        assert_eq!(parse_line("B X"), (Paper, Rock));
        assert_eq!(parse_line("C Z"), (Scissors, Scissors));
    }

    pub fn solve(input: &str) -> u32 {
        let lines = input.trim().lines();
        let parsed_lines = lines.map(parse_line);
        let scores = parsed_lines.map(|(theirs, mine)| mine.score_vs(&theirs));
        scores.sum()
    }

    #[test]
    fn test_solve() {
        let input = "A Y
B X
C Z
";
        assert_eq!(solve(input), 15);
    }
}

pub mod p2 {
    use super::*;

    fn parse_line(line: &str) -> (RPS, Outcome) {
        use RPS::*;
        use Outcome::*;
        let mut chars = line.split_whitespace();
        let their_char = chars.next().unwrap().chars().next().unwrap();
        let outcome_char = chars.next().unwrap().chars().next().unwrap();
        let theirs = match their_char {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Bad char {their_char}"),
        };
        let outcome = match outcome_char {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Bad char {outcome_char}"),
        };
        (theirs, outcome)
    }

    #[test]
    fn test_parse_line() {
        use RPS::*;
        use Outcome::*;
        assert_eq!(parse_line("A Y"), (Rock, Draw));
        assert_eq!(parse_line("B X"), (Paper, Lose));
        assert_eq!(parse_line("C Z"), (Scissors, Win));
    }

    fn score_line(theirs: RPS, outcome: Outcome) -> u32 {
        let mine = RPS::to_throw_for_outcome(theirs, &outcome);
        mine.score() + outcome.score()
    }

    #[test]
    fn test_score_line() {
        use RPS::*;
        use Outcome::*;

        assert_eq!(score_line(Rock, Draw), 4);
        assert_eq!(score_line(Paper, Lose), 1);
        assert_eq!(score_line(Scissors, Win), 7);
    }

    pub fn solve(input: &str) -> u32 {
        let lines = input.trim().lines();
        let parsed_lines = lines.map(parse_line);
        let scores = parsed_lines.map(|(theirs, outcome)| {
            score_line(theirs, outcome)
        });
        scores.sum()
    }

    #[test]
    fn test_solve() {
        let input = "A Y
B X
C Z
";
        assert_eq!(solve(input), 12);
    }
}