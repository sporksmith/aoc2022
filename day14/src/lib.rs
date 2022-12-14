use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

// Signed here makes arithmetic simpler
type Point = (isize, isize);

const GENERATOR: Point = (500, 0);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Rock,
    SandSource,
    DeadSand,
}

struct Cave<const FLOORED: bool> {
    max_wall_y: isize,
    cells: HashMap<Point, Cell>,
}

impl<const FLOORED: bool> Cave<FLOORED> {
    fn get(&self, point: Point) -> Cell {
        if FLOORED && point.1 == (self.max_wall_y + 2) {
            Cell::Rock
        } else {
            *self.cells.get(&point).unwrap_or(&Cell::Empty)
        }
    }

    fn in_abyss(&self, point: Point) -> bool {
        !FLOORED && point.1 > self.max_wall_y
    }

    fn set(&mut self, point: Point, cell: Cell) {
        self.cells.insert(point, cell);
    }

    // Returns final position of the new sand.
    fn process_one_sand(&mut self) -> Option<Point> {
        let mut pos = GENERATOR;
        loop {
            if self.in_abyss(pos) {
                return None
            }
            let next = [
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ]
            .iter()
            .copied()
            .find(|p| 
                self.get(*p) == Cell::Empty
            );
            match next {
                Some(p) => {
                    pos = p;
                }
                None => {
                    self.set(pos, Cell::DeadSand);
                    return Some(pos);
                }
            }
        }
    }
}

impl<const FLOORED: bool> FromStr for Cave<FLOORED> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Vec<Path> = s.lines().map(|l| l.parse().unwrap()).collect();
        let mut cave = {
            let points: Vec<Point> = paths.clone().into_iter().flatten().collect();
            let max_wall_y: isize = points
                .iter()
                .map(|p| p.1)
                .max()
                .unwrap();
            let cells: HashMap<Point, Cell> = HashMap::new();
            Cave {
                max_wall_y,
                cells,
            }
        };
        for path in paths {
            let mut points = path.into_iter();
            let mut current = points.next().unwrap();
            cave.set(current, Cell::Rock);
            for dst in points {
                while dst != current {
                    let dx = (dst.0 - current.0).signum();
                    let dy = (dst.1 - current.1).signum();
                    current = (current.0 + dx, current.1 + dy);
                    cave.set(current, Cell::Rock);
                }
            }
        }
        cave.set(GENERATOR, Cell::SandSource);
        Ok(cave)
    }
}

impl<const FLOORED: bool> Display for Cave<FLOORED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maxy = if FLOORED {
            self.max_wall_y + 2
        } else {
            self.max_wall_y
        };
        let minx: isize = self.cells.keys()
            .map(|p| p.0)
            .min()
            .unwrap();
        let maxx: isize = self.cells.keys()
            .map(|p| p.0)
            .max()
            .unwrap();
        for y in 0..=maxy {
            let y: isize = y.try_into().unwrap();
            if y != 0 {
                write!(f, "\n")?;
            }
            for x in minx..=maxx {
                let c = match self.get((x,y)) {
                    Cell::Empty => '.',
                    Cell::Rock => '#',
                    Cell::SandSource => '+',
                    Cell::DeadSand => 'o',
                };
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Path {
    points: Vec<Point>,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.trim().split(" -> ").map(|pt| {
            let (x, y) = pt.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        });
        Ok(Path {
            points: points.collect(),
        })
    }
}

impl IntoIterator for Path {
    type Item = Point;

    type IntoIter = <Vec<Point> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> u32 {
        let mut cave: Cave<false> = input.parse().unwrap();
        let mut count = 0;
        while let Some(_pos) = cave.process_one_sand() {
            count += 1;
            //println!("{}\n", cave);
        }
        count
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> u32 {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cave() {
        let s = "498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let cave: Cave<false> = s.parse().unwrap();
        assert_eq!(
            format!("{cave}"),
            "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########."
        );
    }

    #[test]
    fn test_process_sand() {
        let s = "498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut cave: Cave<false> = s.parse().unwrap();
        assert_eq!(cave.process_one_sand(), Some((500, 8)));
        assert_eq!(cave.process_one_sand(), Some((499, 8)));
        assert_eq!(cave.process_one_sand(), Some((501, 8)));
        assert_eq!(cave.process_one_sand(), Some((500, 7)));
        assert_eq!(cave.process_one_sand(), Some((498, 8)));
    }

    #[test]
    fn test_solvep1() {
        let s = "498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(p1::solve(s), 24);
    }
}
