use std::collections::HashSet;

type Point = (i32, i32, i32);

fn adjacent_points(p: Point) -> impl Iterator<Item = Point> {
    [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .into_iter()
    .map(move |dp| (p.0 + dp.0, p.1 + dp.1, p.2 + dp.2))
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> u32 {
        let points: HashSet<Point> = input
            .lines()
            .map(|l| {
                let mut l = l.trim().split(',');
                let x = l.next().unwrap().parse().unwrap();
                let y = l.next().unwrap().parse().unwrap();
                let z = l.next().unwrap().parse().unwrap();
                (x, y, z)
            })
            .collect();
        let mut sides = 0;
        for p in &points {
            for p in adjacent_points(*p) {
                if !points.contains(&p) {
                    sides += 1;
                }
            }
        }
        sides
    }
}

pub mod p2 {
    use super::*;

    type Point = (i32, i32, i32);
    fn is_interior(
        empty_point: Point,
        liquid_points: &HashSet<Point>,
        lower_corner: Point,
        upper_corner: Point,
        interior_points: &HashSet<Point>,
    ) -> bool {
        let mut to_explore = HashSet::<Point>::from([empty_point]);
        let mut already_explored = HashSet::<Point>::new();

        loop {
            let Some(p) = to_explore.iter().copied().next() else {
                // We weren't able to find the outside
                return true;
            };
            to_explore.remove(&p);
            if interior_points.contains(&p) {
                // Connected to a point we already know is interior
                return true;
            }
            assert!(!liquid_points.contains(&p));
            if p.0 < lower_corner.0 || p.1 < lower_corner.1 || p.2 < lower_corner.2 {
                // found route to outside
                return false;
            }
            if p.0 > upper_corner.0 || p.1 > upper_corner.1 || p.2 > upper_corner.2 {
                // found route to outside
                return false;
            }
            already_explored.insert(p);
            to_explore.extend(
                adjacent_points(p)
                    .filter(|p| !already_explored.contains(p) && !liquid_points.contains(p)),
            );
        }
    }

    pub fn solve(input: &str) -> u32 {
        let liquid_points: HashSet<(i32, i32, i32)> = input
            .lines()
            .map(|l| {
                let mut l = l.trim().split(',');
                let x = l.next().unwrap().parse().unwrap();
                let y = l.next().unwrap().parse().unwrap();
                let z = l.next().unwrap().parse().unwrap();
                (x, y, z)
            })
            .collect();

        use std::cmp::max;
        use std::cmp::min;
        let lower_bound = liquid_points
            .iter()
            .copied()
            .reduce(|l, r| (min(l.0, r.0), min(l.1, r.1), min(l.2, r.2)))
            .unwrap();
        let upper_bound = liquid_points
            .iter()
            .copied()
            .reduce(|l, r| (max(l.0, r.0), max(l.1, r.1), max(l.2, r.2)))
            .unwrap();
        //let upper_bound = liquit_points.iter().min().unwrap();

        let mut interior_points = HashSet::<Point>::new();
        let mut outside_sides = 0;

        for p in &liquid_points {
            //println!("Checking liquid at {:?}", p);
            for p in adjacent_points(*p) {
                //println!("  Checking adjacent at {:?}", p);
                if liquid_points.contains(&p) || interior_points.contains(&p) {
                    //println!("  is liquid");
                    continue;
                }
                if is_interior(
                    p,
                    &liquid_points,
                    lower_bound,
                    upper_bound,
                    &interior_points,
                ) {
                    interior_points.insert(p);
                    //println!("  is interior");
                    continue;
                }
                //println!("  is exterior");
                outside_sides += 1;
            }
        }
        outside_sides
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5";

    #[test]
    fn test_solvep1() {
        assert_eq!(p1::solve(INPUT), 64);
    }

    #[test]
    fn test_solvep2() {
        assert_eq!(p2::solve(INPUT), 58);
    }
}
