use std::collections::HashSet;
use std::ops::{Add, Sub};

// The tall, vertical chamber is exactly seven units wide.
const WIDTH: isize = 7;

// Each rock appears so that its left edge is two units away from the left wall
const LEFT_SPACE: isize = 2;

// and its bottom edge is three units above the highest rock in the room (or the
// floor, if there isn't one).
const BOTTOM_SPACE: isize = 3;

// "quadrant 1" position
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Self { x, y }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(isize, isize)> for Pos {
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug)]
struct Rock {
    // bottom left corner is 0,0
    positions: HashSet<Pos>,
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
}

impl Rock {
    fn new(positions: impl IntoIterator<Item = impl Into<Pos>>) -> Self {
        let positions: HashSet<Pos> = positions.into_iter().map(Into::into).collect();
        let minx = positions
            .iter()
            .map(|p: &Pos| p.x)
            .min()
            .unwrap_or_default();
        let maxx = positions
            .iter()
            .map(|p: &Pos| p.x)
            .max()
            .unwrap_or_default();
        let miny = positions
            .iter()
            .map(|p: &Pos| p.y)
            .min()
            .unwrap_or_default();
        let maxy = positions
            .iter()
            .map(|p: &Pos| p.y)
            .max()
            .unwrap_or_default();
        Self {
            minx,
            maxx,
            miny,
            maxy,
            positions,
        }
    }

    fn newi(idx: usize) -> Self {
        match idx % 5 {
            // ####
            0 => Rock::new([(0, 0), (1, 0), (2, 0), (3, 0)]),
            // .#.
            // ###
            // .#.
            1 => Rock::new([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
            // ..#
            // ..#
            // ###
            2 => Rock::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
            // #
            // #
            // #
            // #
            3 => Rock::new([(0, 0), (0, 1), (0, 2), (0, 3)]),
            // ##
            // ##
            4 => Rock::new([(0, 0), (1, 0), (0, 1), (1, 1)]),
            _ => unreachable!(),
        }
    }

    fn collides_with(&self, self_pos: &Pos, other: &Rock, other_pos: &Pos) -> bool {
        if self.positions.len() > other.positions.len() {
            other.collides_with(other_pos, self, self_pos)
            /* bounding box check; not needed for p1
            } else if !(self.minx..=self.maxx).contains(&other.minx) && !(self.minx..=self.maxx).contains(&other.maxx){
                false
            } else if !(self.miny..=self.maxy).contains(&other.miny) && !(self.miny..=self.maxy).contains(&other.maxy){
                false
                */
        } else {
            self.positions
                .iter()
                .any(|pos| other.positions.contains(&(*pos + *self_pos - *other_pos)))
        }
    }

    fn merge(&mut self, self_pos: &Pos, other: &Rock, other_pos: &Pos) {
        for p in other.positions.iter() {
            let p = *p + *other_pos - *self_pos;
            self.positions.insert(p);
            self.maxx = std::cmp::max(self.maxx, p.x);
            self.maxy = std::cmp::max(self.maxy, p.y);
            self.minx = std::cmp::min(self.minx, p.x);
            self.miny = std::cmp::min(self.miny, p.y);
        }
    }
}

pub mod p1 {
    use super::*;

    fn collides_with_tower_or_wall(
        shape: &Rock,
        shape_pos: &Pos,
        tower: &Rock,
        tower_pos: &Pos,
    ) -> bool {
        if shape.minx + shape_pos.x <= 0 {
            true
        } else if shape.maxx + shape_pos.x > WIDTH {
            true
        } else if shape.miny + shape_pos.y == 0 {
            true
        } else {
            shape.collides_with(shape_pos, tower, tower_pos)
        }
    }

    fn print_tower(shape: &Rock, shape_pos: &Pos, tower: &Rock, tower_pos: &Pos) {
        // Starting at y=0, which will be the *bottom*.
        let mut lines = Vec::<String>::new();
        for y in 1..=std::cmp::max(shape.maxy + shape_pos.y, tower.maxy + shape_pos.y) {
            let mut line = String::new();
            line.push('|');
            for x in 1..=WIDTH {
                let pos = Pos::new(x, y);
                if shape.positions.contains(&(pos - *shape_pos)) {
                    line.push('@');
                } else if tower.positions.contains(&(pos - *tower_pos)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            line.push('|');
            lines.push(line);
        }
        for line in lines.iter().rev() {
            println!("{}", line);
        }
        println!(
            "{}",
            ['-']
                .iter()
                .cycle()
                .take(WIDTH as usize + 2)
                .collect::<String>()
        )
    }

    pub fn solve(input: &str) -> isize {
        let mut tower = Rock::new([Pos::new(0, 0); 0]);
        let tower_pos = Pos::new(0, 0);

        let mut shape = Rock::newi(0);
        let mut next_shape_idx = 1;
        let mut shape_pos = Pos::new(LEFT_SPACE + 1, BOTTOM_SPACE + 1);

        let mut jets = input.trim().chars().cycle();

        let mut rocks_processed = 0;
        //println!("Initial {:?} @ {:?}", shape, shape_pos);
        //print_tower(&shape, &shape_pos, &tower, &tower_pos);
        loop {
            //println!("processed {rocks_processed}; current at {:?}", shape_pos);
            let jet = jets.next().unwrap();
            let jetted_pos = match jet {
                '>' => shape_pos + Pos::new(1, 0),
                '<' => shape_pos + Pos::new(-1, 0),
                _ => panic!("unexpected c"),
            };
            if !collides_with_tower_or_wall(&shape, &jetted_pos, &tower, &tower_pos) {
                //println!("Jet {jet} pushes");
                shape_pos = jetted_pos;
            } else {
                //println!("Jet {jet} collides");
            }
            //print_tower(&shape, &shape_pos, &tower, &tower_pos);
            let dropped_pos = shape_pos + Pos::new(0, -1);
            if !collides_with_tower_or_wall(&shape, &dropped_pos, &tower, &tower_pos) {
                shape_pos = dropped_pos;
                //println!("Rock drops");
                //print_tower(&shape, &shape_pos, &tower, &tower_pos);
            } else {
                // Combine with tower
                tower.merge(&tower_pos, &shape, &shape_pos);

                rocks_processed += 1;
                if rocks_processed == 2022 {
                    break;
                } else {
                    // Spawn new shape
                    shape = Rock::newi(next_shape_idx);
                    next_shape_idx += 1;
                    shape_pos = Pos::new(LEFT_SPACE + 1, tower.maxy + BOTTOM_SPACE + 1);

                    //println!("After {rocks_processed}");
                    //print_tower(&shape, &shape_pos, &tower, &tower_pos);
                }
            }
        }
        tower.maxy
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> isize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collides() {
        for i in 0..5 {
            // Collides with self
            assert!(Rock::newi(i).collides_with(&Pos::new(0, 0), &Rock::newi(i), &Pos::new(0, 0)));
        }
        // adjacent shape 4's
        assert!(!Rock::newi(4).collides_with(&Pos::new(1, 1), &Rock::newi(4), &Pos::new(3, 3)));
        // colliding shape 4's
        assert!(Rock::newi(4).collides_with(&Pos::new(1, 1), &Rock::newi(4), &Pos::new(2, 2)));
    }

    #[test]
    fn test_merge() {
        let mut rock = Rock::newi(4);
        // adjacent shape 4's
        rock.merge(&Pos::new(1, 1), &Rock::newi(4), &Pos::new(3, 1));
        assert_eq!(rock.minx, 0);
        assert_eq!(rock.miny, 0);
        assert_eq!(rock.maxx, 3);
        assert_eq!(rock.maxy, 1);
        rock.positions.contains(&(2, 0).into());
        rock.positions.contains(&(3, 0).into());
        rock.positions.contains(&(2, 1).into());
        rock.positions.contains(&(3, 1).into());
    }

    #[test]
    fn test_solvep1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(p1::solve(input), 3068);
    }
}
