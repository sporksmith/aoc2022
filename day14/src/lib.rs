use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

// Signed here makes arithmetic simpler
type Point = (isize, isize);

const GENERATOR : Point = (500, 0);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Rock,
    SandSource,
    DeadSand,
    ActiveSand,
}

struct Cave {
    minx: usize,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Cave {
    fn idx(&self, point: Point) -> Option<usize> {
        let x: usize = point.0.try_into().ok()?;
        let y: usize = point.1.try_into().ok()?;
        if x >= self.width {
            None
        } else if y >= self.height {
            None
        } else {
            Some(x + y * self.width)
        }
    }

    fn get(&self, point: Point) -> Option<&Cell> {
        let i = self.idx(point)?;
        Some(&self.cells[i])
    }

    fn get_mut(&mut self, point: Point) -> Option<&mut Cell> {
        let i = self.idx(point)?;
        Some(&mut self.cells[i])
    }
}

impl Index<Point> for Cave {
    type Output=Cell;

    fn index(&self, index: Point) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<Point> for Cave {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl FromStr for Cave {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths : Vec<Path> = s.lines().map(|l| l.parse().unwrap()).collect();
        let mut cave = {
            let points : Vec<Point> = paths.clone().into_iter().flatten().collect();
            let width: usize = points.iter().map(|p| usize::try_from(p.0).unwrap()).max().unwrap() + 1;
            let height: usize = points.iter().map(|p| usize::try_from(p.1).unwrap()).max().unwrap() + 1;
            let minx: usize = points.iter().map(|p| usize::try_from(p.0).unwrap()).min().unwrap();
            let mut cells : Vec<Cell> = Vec::new();
            cells.resize(width * height, Cell::Empty);
            Cave{minx, width, height, cells}
        };
        for path in paths {
            let mut points = path.into_iter();
            let mut current = points.next().unwrap();
            *cave.get_mut(current).unwrap() = Cell::Rock;
            for dst in points {
                while dst != current {
                    let dx = (dst.0 - current.0).signum();
                    let dy = (dst.1 - current.1).signum();
                    current = (current.0 + dx, current.1 + dy);
                    cave[current] = Cell::Rock;
                }
            }
        }
        cave[GENERATOR] = Cell::SandSource;
        Ok(cave)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let y: isize = y.try_into().unwrap();
            if y != 0 {
                write!(f, "\n")?;
            }
            for x in self.minx..self.width {
                let x: isize = x.try_into().unwrap();
                let c = match self[(x,y)] {
                    Cell::Empty => '.',
                    Cell::Rock => '#',
                    Cell::SandSource => '+',
                    Cell::DeadSand => 'o',
                    Cell::ActiveSand => '~',
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
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.trim().split(" -> ").map(|pt| {
            let (x, y) = pt.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x,y)
        });
        Ok(Path{points: points.collect()})
    }
}

impl IntoIterator for Path {
    type Item=Point;

    type IntoIter=<Vec<Point> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

pub mod p1 {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cave() {
        let s = "498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9";
        let cave: Cave = s.parse().unwrap();
        assert_eq!(cave.width, 504);
        assert_eq!(cave.height, 10);
        println!("{cave}");
        assert_eq!(format!("{cave}"), "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.");
    }

}