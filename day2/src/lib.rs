pub mod p1 {
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
