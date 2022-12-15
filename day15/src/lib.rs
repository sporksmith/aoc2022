use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    pub x: u32,
    pub y: u32,
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
}
