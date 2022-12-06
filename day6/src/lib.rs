pub mod p1 {
    use std::collections::BTreeSet;

    pub fn solve(input: &str) -> usize {
        let mut chars = input.chars();
        let mut idx = 0;
        loop {
            let next4 : BTreeSet<char> = chars.clone().take(4).collect();
            if next4.len() == 4 {
                return idx + 4;
            }
            idx += 1;
            chars.next();
        }
    }
    #[test]
    pub fn test_solve() {
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}