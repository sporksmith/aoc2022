use std::collections::BTreeSet;

fn bytes_read_until_unique_n(input: &str, n: usize) -> usize {
    let mut chars = input.chars();
    let mut idx = 0;
    loop {
        let next_n: BTreeSet<char> = chars.clone().take(n).collect();
        if next_n.len() == n {
            return idx + n;
        }
        idx += 1;
        chars.next();
    }
}

pub mod p1 {
    use crate::bytes_read_until_unique_n;

    pub fn solve(input: &str) -> usize {
        bytes_read_until_unique_n(input, 4)
    }
    #[test]
    pub fn test_solve() {
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}

pub mod p2 {
    use crate::bytes_read_until_unique_n;

    pub fn solve(input: &str) -> usize {
        bytes_read_until_unique_n(input, 14)
    }
    #[test]
    pub fn test_solve() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
}
