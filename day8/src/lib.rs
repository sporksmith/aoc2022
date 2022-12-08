use std::{fmt::Debug, str::FromStr};

#[derive(Eq, PartialEq, Debug)]
struct TreeValues<T>
where
    T: Eq + PartialEq + Debug,
{
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T> TreeValues<T>
where
    T: Eq + PartialEq + Debug,
{
    fn idx(&self, x: usize, y: usize) -> usize {
        assert!(x <= self.width);
        assert!(y <= self.height);
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.values[self.idx(x, y)]
    }
}

#[derive(Eq, PartialEq, Debug)]
struct TreeHeights(TreeValues<u32>);
impl TreeHeights {
    pub fn height_at(&self, x: usize, y: usize) -> u32 {
        *self.0.get(x, y)
    }

    pub fn width(&self) -> usize {
        self.0.width
    }

    pub fn height(&self) -> usize {
        self.0.height
    }

    pub fn visibilities(&self) -> TreeVisibilities {
        let mut visibilities = Vec::new();
        visibilities.resize(self.width() * self.height(), false);

        let mut check = |x: usize, y: usize, tallest: &mut Option<u32>| {
            let height = self.height_at(x, y);
            if let Some(tallest) = tallest {
                if *tallest >= height {
                    return;
                }
            }
            visibilities[self.0.idx(x, y)] = true;
            tallest.replace(height);
        };

        // From left
        for y in 0..self.height() {
            let mut tallest = None;
            for x in 0..self.width() {
                check(x, y, &mut tallest);
            }
        }

        // From right
        for y in 0..self.height() {
            let mut tallest = None;
            for x in (0..self.width()).rev() {
                check(x, y, &mut tallest);
            }
        }

        // From top
        for x in 0..self.width() {
            let mut tallest = None;
            for y in 0..self.height() {
                check(x, y, &mut tallest);
            }
        }

        // From bottom
        for x in 0..self.width() {
            let mut tallest = None;
            for y in (0..self.height()).rev() {
                check(x, y, &mut tallest);
            }
        }

        TreeVisibilities(TreeValues {
            width: self.width(),
            height: self.height(),
            values: visibilities,
        })
    }

    pub fn scenic_score_at(&self, x: usize, y: usize) -> usize {
        let max_height = self.height_at(x, y);

        let mut left = 0;
        for x in (0..x).rev() {
            left += 1;
            if self.height_at(x, y) >= max_height {
                break;
            }
        }

        let mut right = 0;
        for x in (x + 1)..self.width() {
            right += 1;
            if self.height_at(x, y) >= max_height {
                break;
            }
        }

        let mut up = 0;
        for y in (y + 1)..self.height() {
            up += 1;
            if self.height_at(x, y) >= max_height {
                break;
            }
        }

        let mut down = 0;
        for y in (0..y).rev() {
            down += 1;
            if self.height_at(x, y) >= max_height {
                break;
            }
        }

        left * right * up * down
    }
}

impl FromStr for TreeHeights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut heights = Vec::new();
        heights.reserve(s.len());
        for (row_idx, row) in s.lines().enumerate() {
            height = row_idx + 1;
            for (col_idx, tree_height) in row.trim().chars().enumerate() {
                width = std::cmp::max(width, col_idx + 1);
                let tree_height = tree_height.to_digit(10).unwrap();
                heights.push(tree_height);
            }
        }
        Ok(TreeHeights(TreeValues {
            width,
            height,
            values: heights,
        }))
    }
}

#[derive(Eq, PartialEq, Debug)]
struct TreeVisibilities(TreeValues<bool>);
impl TreeVisibilities {
    pub fn count(&self) -> usize {
        self.0.values.iter().copied().filter(|x| *x).count()
    }
}

#[test]
fn test_trees_fromstr() {
    let s = r#"12
                     34"#;
    let trees: TreeHeights = s.parse().unwrap();
    assert_eq!(
        trees,
        TreeHeights(TreeValues {
            width: 2,
            height: 2,
            values: vec![1, 2, 3, 4]
        })
    );
    assert_eq!(trees.height_at(0, 0), 1);
    assert_eq!(trees.height_at(1, 0), 2);
    assert_eq!(trees.height_at(0, 1), 3);
    assert_eq!(trees.height_at(1, 1), 4);
}

#[test]
fn test_get_visibilities() {
    let s: &str = r#"00
                     00"#;
    let heights: TreeHeights = s.parse().unwrap();
    assert_eq!(
        heights.visibilities(),
        TreeVisibilities(TreeValues {
            width: 2,
            height: 2,
            values: vec![true, true, true, true]
        })
    );

    let s: &str = r#"11
                     11"#;
    let heights: TreeHeights = s.parse().unwrap();
    assert_eq!(
        heights.visibilities(),
        TreeVisibilities(TreeValues {
            width: 2,
            height: 2,
            values: vec![true, true, true, true]
        })
    );

    let s: &str = r#"111
                     101
                     111"#;
    let heights: TreeHeights = s.parse().unwrap();
    assert_eq!(heights.visibilities().count(), 8);

    let s: &str = r#"111
                     121
                     111"#;
    let heights: TreeHeights = s.parse().unwrap();
    assert_eq!(heights.visibilities().count(), 9);

    let s: &str = r#"30373
                     25512
                     65332
                     33549
                     35390"#;
    let heights: TreeHeights = s.parse().unwrap();
    assert_eq!(heights.visibilities().count(), 21);
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let heights: TreeHeights = input.parse().unwrap();
        let visibilities = heights.visibilities();
        visibilities.count()
    }
}

pub mod p2 {
    use itertools::Itertools;

    use super::*;

    pub fn solve(input: &str) -> usize {
        let heights: TreeHeights = input.parse().unwrap();
        (0..heights.width())
            .cartesian_product(0..heights.height())
            .map(|(x, y)| heights.scenic_score_at(x, y))
            .max()
            .unwrap()
    }

    #[test]
    fn test_solve() {
        let s: &str = r#"30373
                         25512
                         65332
                         33549
                         35390"#;
        assert_eq!(solve(s), 8);
    }
}
