use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Sensor {
    beacon: (i32, i32),
    sensor: (i32, i32),
    distance: i32,
}

fn line_to(first: (i32, i32), second: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec![];

    let delta_x = (first.0 - second.1).abs();

    let x_step = if first.0 > second.1 { -1 } else { 1 };
    let y_step = if first.1 > second.1 { -1 } else { 1 };

    let mut x = first.0;
    let mut y = first.1;

    points.push((x, y));

    for _ in 0..delta_x {
        x += x_step;
        y += y_step;

        points.push((x, y));
    }

    points
}

impl Sensor {
    fn new(beacon: (i32, i32), sensor: (i32, i32)) -> Sensor {
        Sensor {
            beacon,
            sensor,
            distance: manhattan_distance(beacon, sensor),
        }
    }

    fn distance(&self) -> i32 {
        self.distance
    }

    fn is_in_range(&self, x: i32, y: i32) -> bool {
        let distance = manhattan_distance((x, y), self.sensor);

        distance <= self.distance()
    }

    fn points(&self, y: i32) -> Vec<(i32, i32)> {
        // find all the points that are less than or equal to self.distance() but where the y value is y

        let distance = self.distance();

        let mut points = vec![];

        // find the extremes

        let delta_y = (self.sensor.1 - y).abs();

        let left_over = distance - delta_y;

        // if left over is negative, then we can't reach y

        if left_over < 0 {
            return points;
        }

        let left = self.sensor.0 - left_over;
        let right = self.sensor.0 + left_over;

        for x in left..=right {
            points.push((x, y));
        }

        points
    }

    fn range_at_y(&self, y: i32) -> RangeInclusive<i32> {
        let distance = self.distance();

        let delta_y = (self.sensor.1 - y).abs();

        let left_over = distance - delta_y;

        if left_over < 0 {
            return 0..=0;
        }

        let left = self.sensor.0 - left_over;
        let right = self.sensor.0 + left_over;

        left..=right
    }
}

pub fn part1(inp: String) {
    let sensors = inp.lines().map(|l| {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let parts = l
            .split(":")
            .next()
            .unwrap()
            .strip_prefix("Sensor at x=")
            .unwrap();

        let (x, y) = parts
            .split(", y=")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        let parts = l.split(": closest beacon is at x=").collect_vec();

        let (bx, by) = parts[1]
            .split(", y=")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        Sensor::new((bx, by), (x, y))
    });

    let row = 2000000;

    let points: Vec<(i32, i32)> = sensors.clone().flat_map(|s| s.points(row)).collect();

    let unique = points.iter().map(|f| f.0).unique();

    // println!("unique {}", unique.clone().count());

    let mut beacons_at_y = sensors
        .filter(|s| s.beacon.1 == row)
        .map(|f| f.beacon.0)
        .unique();

    // ignore the beacons

    let count = unique.clone().count() - beacons_at_y.clone().count();

    println!("{}", count);
    // println!("beacons at y: {:?}", beacons_at_y.collect_vec());
    // println!("unique points: {:?}", unique.collect_vec());
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

pub fn part2(inp: String) {
    let sensors = inp.lines().map(|l| {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let parts = l
            .split(":")
            .next()
            .unwrap()
            .strip_prefix("Sensor at x=")
            .unwrap();

        let (x, y) = parts
            .split(", y=")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        let parts = l.split(": closest beacon is at x=").collect_vec();

        let (bx, by) = parts[1]
            .split(", y=")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        Sensor::new((bx, by), (x, y))
    });

    let cave = 0..4_000_000;

    for sen in sensors.clone() {
        let up = (sen.sensor.0, sen.sensor.1 + sen.distance + 1);
        let down = (sen.sensor.0, sen.sensor.1 - sen.distance - 1);
        let left = (sen.sensor.0 - sen.distance - 1, sen.sensor.1);
        let right = (sen.sensor.0 + sen.distance + 1, sen.sensor.1);

        let mut points = line_to(up, left);

        points.extend(line_to(up, right));
        points.extend(line_to(down, left));
        points.extend(line_to(down, right));

        let points = points
            .iter()
            .unique()
            .filter(|f| cave.contains(&f.0) && cave.contains(&f.1))
            .for_each(|s| {
                let alone = sensors.clone().map(|f| f.is_in_range(s.0, s.1)).all(|f| !f);

                if alone {
                    println!("{} {}", s.0, s.1);
                } else {
                    // println!("{} {} not alone", s.0, s.1)
                }
            });

        println!("next sensor");
    }
}
