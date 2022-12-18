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
    for y in 1..=std::cmp::max(shape.maxy + shape_pos.y, tower.maxy + tower_pos.y) {
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

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct StateKey {
    jet_idx: u32,
    shape_idx: u8,
    top_lines: Vec<u8>,
}

impl StateKey {
    fn new(jet_idx: usize, shape_idx: usize, tower: &Rock) -> Self {
        let mut top_lines = Vec::<u8>::new();
        for dy in 0..10 {
            let y = tower.maxy - dy;
            let mut line = 0u8;
            for x in 1..=WIDTH {
                if tower.positions.contains(&Pos::new(x, y)) {
                    line += 1;
                }
                line <<= 2;
            }
            top_lines.push(line);
        }
        Self { jet_idx: jet_idx as u32, shape_idx: shape_idx as u8, top_lines }
    }
}

struct StateValue {
    height: usize,
}

pub fn simulate(input: &str, rock_limit: usize) -> isize {
    let mut tower = Rock::new([Pos::new(0, 0); 0]);
    let mut tower_pos = Pos::new(0, 0);

    let mut shape = Rock::newi(0);
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

            // Check if we can delete any lines.
            // XXX this code isn't quite right, but also there's never a single complete
            // line.
            /*
            for y in (shape_pos.y + shape.miny)..=(shape_pos.y + shape.maxy) {
                if (1..=WIDTH).all(|x| tower.positions.contains(&Pos::new(x, y + tower_pos.y))) {
                    tower_pos = Pos::new(tower_pos.x, tower_pos.y + y);
                    tower.positions = tower
                        .positions
                        .iter()
                        .copied()
                        .filter(|p| p.y > y)
                        .collect();
                }
            }
            */

            rocks_processed += 1;
            // Spawn new shape
            shape = Rock::newi(rocks_processed);
            shape_pos = Pos::new(LEFT_SPACE + 1, tower.maxy + BOTTOM_SPACE + 1);

            if rocks_processed % 100_000 == 0 {
                println!(
                    "Handled {rocks_processed} rocks; trimmed {} lines; {}% done",
                    tower_pos.y,
                    100.0 * rocks_processed as f32 / rock_limit as f32
                );
            }
            if rocks_processed == rock_limit {
                // println!("After {rocks_processed}");
                //print_tower(&shape, &shape_pos, &tower, &tower_pos);
                break;
            }
        }
    }
    println!("Final tower pos: {:?}", tower_pos);
    print_tower(&shape, &shape_pos, &tower, &tower_pos);
    tower.maxy
}
pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> isize {
        simulate(input, 2022)
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> isize {
        let rocks = 1000000000000;
        let min_cycle_len = input.len() as isize * 5;

        let mut cycle_len = min_cycle_len;
        loop {
            let cycle_height = simulate(input, cycle_len as usize) as isize;
            let cycle2_height = simulate(input, cycle_len as usize * 2) as isize;
            let cycle_height2 = cycle_height * 2;
            let diff = cycle_height2 - cycle2_height;
            println!("cycle_len: {cycle_len}, diff: {diff}");
            if diff == 0 {
                break;
            }
            cycle_len += min_cycle_len;
        }
        let cycle_height = simulate(input, cycle_len as usize) as isize;

        let cycle_height_2 = simulate(input, cycle_len as usize * 2) as isize;
        println!(
            "cycle height * 2: {}, (cycle*2) height: {}",
            cycle_height * 2,
            cycle_height_2
        );

        let complete_cycles = rocks / cycle_len;
        println!("cycle len {cycle_len}; complete cycles {complete_cycles}");

        let rem_cycles = rocks % cycle_len;

        println!(
            "cycle len {cycle_len}; complete cycles {complete_cycles} + {rem_cycles} remaining"
        );

        cycle_height * (rocks / cycle_len) // + simulate(input, rocks as usize % cycle_len as usize)
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

    #[test]
    fn test_solvep2() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(p2::solve(input), 1514285714288);
    }
}
