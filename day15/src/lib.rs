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
    fn excludes(&self, point: &Point) -> bool {
        self.beacon != *point && self.sensor.dist(point) <= self.sensor.dist(&self.beacon)
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
            let excluding_data = data.iter().find(|sd| sd.excludes(&Point::new(x, y)));
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

    pub fn solve(input: &str) -> u32 {
        todo!()
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

    #[test]
    fn test_solvep1() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
        assert_eq!(p1::solve(input, 10), 26)
    }
}
