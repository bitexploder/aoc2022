use std::cmp::min;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    sensor: Coord,
    cbeacon: Coord,
    bdist: i64,
}

impl Sensor {
    fn new(sensor_x: i64, sensor_y: i64, beacon_x: i64, beacon_y: i64) -> Self {
        let sc = Coord {
            x: sensor_x,
            y: sensor_y,
        };

        let bc = Coord {
            x: beacon_x,
            y: beacon_y,
        };

        Sensor {
            sensor: sc,
            cbeacon: bc,
            bdist: distance(sc, bc),
        }
    }

    fn corners(&self) -> (i64, i64, i64, i64) {
        (
            self.sensor.x - self.bdist as i64,
            self.sensor.x + self.bdist as i64,
            self.sensor.y - self.bdist as i64,
            self.sensor.y + self.bdist as i64,
        )
    }
}

fn parse_input(contents: &str) -> (Vec<Sensor>, Vec<Coord>) {
    let coords = contents
        .lines()
        .map(|line| {
            let toks = line.split(" ").collect::<Vec<&str>>();
            let coords = vec![toks[2], toks[3], toks[8], toks[9]];
            coords
                .iter()
                .map(|tok| {
                    tok.replace(&[',', 'x', 'y', '=', ':'][..], "")
                        .parse()
                        .unwrap()
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let sensors = coords
        .iter()
        .map(|coords| Sensor::new(coords[0], coords[1], coords[2], coords[3]))
        .collect::<Vec<Sensor>>();

    let beacons = coords
        .iter()
        .map(|coords| Coord {
            x: coords[2],
            y: coords[3],
        })
        .collect::<Vec<Coord>>();

    (sensors, beacons)
}

#[inline]
fn distance(c1: Coord, c2: Coord) -> i64 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}

fn get_corners(sensors: &Vec<Sensor>) -> (i64, i64, i64, i64) {
    let mut corners = (0, 0, 0, 0);
    for sensor in sensors {
        let test_coords = (
            sensor.sensor.x - sensor.bdist as i64,
            sensor.sensor.x + sensor.bdist as i64,
            sensor.sensor.y - sensor.bdist as i64,
            sensor.sensor.y + sensor.bdist as i64,
        );

        if test_coords.0 < corners.0 {
            corners.0 = test_coords.0;
        }
        if test_coords.1 > corners.1 {
            corners.1 = test_coords.1;
        }
        if test_coords.2 < corners.2 {
            corners.2 = test_coords.2;
        }
        if test_coords.3 > corners.3 {
            corners.3 = test_coords.3;
        }
    }
    corners
}

fn main() {
    let contents = include_str!("puzzle.txt");
    // let contents = include_str!("puzzle.txt");

    let (sensors, beacons) = parse_input(&contents);
    let corners = get_corners(&sensors);

    let y = 2000000;
    let mut cnt = 0;
    for x in corners.0..=corners.1 {
        let mut covered = false;
        for sensor in &sensors {
            if sensor.cbeacon.x == x && sensor.cbeacon.y == y {
                continue;
            }
            let dist = distance(sensor.sensor, Coord { x, y });
            if dist <= sensor.bdist {
                covered = true;
                cnt += 1;
                break;
            }
        }
    }
    eprintln!("cnt = {:?}", cnt);

    // Brute force will not work
    let mut negl: Vec<i64> = Vec::new();
    let mut posl: Vec<i64> = Vec::new();

    // create a series of lines that define the edges of the sensors
    for s in &sensors {
        negl.push(s.sensor.x + s.sensor.y - s.bdist);
        negl.push(s.sensor.x + s.sensor.y + s.bdist);

        posl.push(s.sensor.x - s.sensor.y - s.bdist);
        posl.push(s.sensor.x - s.sensor.y + s.bdist);
    }

    // positive slope that are close to each other
    // negative slope lines that are close to each other
    let mut positive = 0i64;
    let mut negative = 0i64;

    for i in 0..sensors.len() * 2 {
        for j in (i + 1)..sensors.len() * 2 {
            let a = posl[i];
            let b = posl[j];

            // hunt for positive slope lines that are close together
            if (a - b).abs() == 2 {
                positive = min(a, b) + 1;
            }

            let c = negl[i];
            let d = negl[j];

            // hunt for negative slope lines that are close together
            if (c - d).abs() == 2 {
                negative = min(c, d) + 1;
            }
        }
    }

    eprintln!("positive = {:?}", positive);
    eprintln!("negative = {:?}", negative);

    // system of linear equations
    let x = (positive + negative) / 2;
    let y = (negative - positive) / 2;

    println!("x={}, y={}", x, y);

    let ans = x * 4000000 + y;
    println!("sol={}", ans);
}
