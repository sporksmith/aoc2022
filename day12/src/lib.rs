use std::collections::VecDeque;
use std::str::FromStr;

struct Map {
    heights: Vec<u8>,
    width: usize,
    height: usize,
    start_idx: usize,
    end_idx: usize,
}

impl Map {
    fn row(&self, idx: usize) -> usize {
        idx / self.width
    }

    fn col(&self, idx: usize) -> usize {
        idx % self.width
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }

    fn shortest_path_from(&self, start: usize) -> Option<usize> {
        let mut steps_from_start = Vec::<Option<usize>>::new();
        steps_from_start.resize(self.heights.len(), None);

        steps_from_start[start] = Some(0);
        let mut to_visit = VecDeque::<usize>::new();
        to_visit.push_back(start);

        while let Some(idx) = to_visit.pop_front() {
            let curr_steps_from_start = steps_from_start[idx].unwrap();
            if idx == self.end_idx {
                return Some(curr_steps_from_start);
            }
            let col = self.col(idx);
            let row = self.row(idx);

            let mut candidates = Vec::new();

            // left
            if col > 0 {
                candidates.push(idx - 1)
            }
            // right
            if col < (self.width - 1) {
                candidates.push(idx + 1)
            }
            // up
            if row > 0 {
                candidates.push(self.idx(row - 1, col))
            }
            // down
            if row < (self.height - 1) {
                candidates.push(self.idx(row + 1, col))
            }

            for c in candidates {
                if steps_from_start[c].is_some() {
                    // already covered
                    continue;
                }
                if self.heights[c] > (self.heights[idx] + 1) {
                    // too high
                    continue;
                }
                steps_from_start[c] = Some(curr_steps_from_start + 1);
                to_visit.push_back(c);
            }
        }
        // unreachable
        None
    }
}

fn ascii_ord(c: char) -> u8 {
    let mut buf = [0; 1];
    c.encode_utf8(&mut buf);
    buf[0]
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut heights = Vec::new();
        let mut start_idx = 0;
        let mut end_idx = 0;
        for c in s.chars() {
            match c {
                '\n' | ' ' => continue,
                'S' => {
                    start_idx = heights.len();
                    heights.push(ascii_ord('a'));
                }
                'E' => {
                    end_idx = heights.len();
                    heights.push(ascii_ord('z'));
                }
                'a'..='z' => {
                    heights.push(ascii_ord(c));
                }
                _ => {
                    panic!("unexpected char {c}");
                }
            }
        }
        Ok(Map {
            heights,
            width,
            height,
            start_idx,
            end_idx,
        })
    }
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let map: Map = input.parse().unwrap();
        map.shortest_path_from(map.start_idx).unwrap()
    }
    #[test]
    fn test_solve() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(solve(input), 31);
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let map: Map = input.parse().unwrap();
        let starts = map.heights.iter().enumerate().filter_map(|(idx, height)| {
            if *height == ascii_ord('a') {
                Some(idx)
            } else {
                None
            }
        });
        let lengths = starts.filter_map(|idx| map.shortest_path_from(idx));
        lengths.min().unwrap()
    }
    #[test]
    fn test_solve() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(solve(input), 29);
    }
}
