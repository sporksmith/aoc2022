use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn dist(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y.strip_prefix("y=").unwrap().parse().unwrap();
        Ok(Point { x, y })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SensorData {
    sensor: Point,
    beacon: Point,
}

impl SensorData {
    fn p1_excludes(&self, point: &Point) -> bool {
        self.beacon != *point && self.sensor.dist(point) <= self.sensor.dist(&self.beacon)
    }

    fn excluded_xs_for_y(&self, y: i64) -> RangeInclusive<i64> {
        let dist_to_sensor = self.sensor.dist(&self.beacon);
        let dist_to_closest_y = self.sensor.dist(&Point::new(self.sensor.x, y));
        let slack = dist_to_sensor as i64 - dist_to_closest_y as i64;
        (self.sensor.x - slack)..=(self.sensor.x + slack)
    }
}

impl FromStr for SensorData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Sensor at ").unwrap();
        let (sensor, beacon) = s.split_once(": closest beacon is at ").unwrap();
        Ok(SensorData {
            sensor: sensor.parse().unwrap(),
            beacon: beacon.parse().unwrap(),
        })
    }
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str, y: i64) -> u32 {
        let data: Vec<SensorData> = input.lines().map(|s| s.trim().parse().unwrap()).collect();
        let minx = data
            .iter()
            .map(|sd| sd.sensor.x - sd.beacon.x)
            .min()
            .unwrap();
        let maxx = data
            .iter()
            .map(|sd| sd.sensor.x + sd.beacon.x)
            .max()
            .unwrap();
        let mut excluded_count = 0;
        //let y = 2000000;
        //let mut vis = String::new();
        for x in minx..=maxx {
            let excluding_data = data.iter().find(|sd| sd.p1_excludes(&Point::new(x, y)));
            if excluding_data.is_some() {
                //vis.push('#');
                excluded_count += 1;
            } else {
                //vis.push('.');
            }
        }
        //println!("From {minx}: {vis}");
        excluded_count
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str, max: i64) -> i64 {
        let mut data: Vec<SensorData> = input.lines().map(|s| s.trim().parse().unwrap()).collect();
        data.sort_by(|l, r| l.sensor.x.cmp(&r.sensor.x));
        for y in 0..=max {
            let mut x = 0;
            for sd in data.iter() {
                let excluded_xs = sd.excluded_xs_for_y(y);
                if excluded_xs.contains(&x) {
                    let next_x = excluded_xs.end() + 1;
                    //println!("{:?} excludes {:?}; skipping to {}", sd, excluded_xs, next_x);
                    x = next_x;
                }
                if x > max {
                    break;
                }
            }
            if x <= max {
                println!("Found at {x},{y}");
                return x * 4000000 + y;
            }
        }
        panic!("Not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Sensor at x=1638847, y=3775370: closest beacon is at x=2498385, y=3565515".parse(),
            Ok(SensorData {
                sensor: Point {
                    x: 1638847,
                    y: 3775370
                },
                beacon: Point {
                    x: 2498385,
                    y: 3565515
                }
            })
        );
    }

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_excluded_range() {
        let sd: SensorData = "Sensor at x=8, y=7: closest beacon is at x=2, y=10"
            .parse()
            .unwrap();
        assert_eq!(sd.excluded_xs_for_y(-2), 8..=8);
        assert_eq!(sd.excluded_xs_for_y(-1), 7..=9);
        assert_eq!(sd.excluded_xs_for_y(0), 6..=10);
        assert_eq!(sd.excluded_xs_for_y(1), 5..=11);
        assert_eq!(sd.excluded_xs_for_y(2), 4..=12);
        assert_eq!(sd.excluded_xs_for_y(3), 3..=13);
        assert_eq!(sd.excluded_xs_for_y(4), 2..=14);
        assert_eq!(sd.excluded_xs_for_y(5), 1..=15);
        assert_eq!(sd.excluded_xs_for_y(6), 0..=16);
        assert_eq!(sd.excluded_xs_for_y(7), -1..=17);
        assert_eq!(sd.excluded_xs_for_y(8), 0..=16);
    }

    #[test]
    fn test_solvep1() {
        assert_eq!(p1::solve(TEST_INPUT, 10), 26)
    }

    #[test]
    fn test_solvep2() {
        assert_eq!(p2::solve(TEST_INPUT, 20), 56000011)
    }
}
