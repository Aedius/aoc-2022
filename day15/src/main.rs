use helper::{aoc1, aoc1_full, aoc2, aoc2_alone, InputReader};
use regex::Regex;
use std::cmp::{max, min};

fn main() {
    // aoc1_full!(Container, "day15", 26, 4665948);
    aoc2_alone!(Container, "day15", 56000011);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Censor {
    position: Coord,
    beacon: Coord,
    dist: isize,
}

#[derive(Debug)]
struct Container {
    regex: Regex,
    sensors: Vec<Censor>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new(
                "Sensor at x=([0-9-]+), y=([0-9-]+): closest beacon is at x=([0-9-]+), y=([0-9-]+)",
            )
            .unwrap(),
            sensors: vec![],
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if let Some(cap) = self.regex.captures(line) {
            let mut sensor = Censor {
                position: Coord {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                },
                beacon: Coord {
                    x: cap[3].parse().unwrap(),
                    y: cap[4].parse().unwrap(),
                },
                dist: 0,
            };

            sensor.dist = sensor.position.dist(&sensor.beacon);

            self.min_x = min(self.min_x, sensor.position.x - sensor.dist);
            self.min_y = min(self.min_y, sensor.position.y - sensor.dist);
            self.max_x = max(self.max_x, sensor.position.x + sensor.dist);
            self.max_y = max(self.max_y, sensor.position.y + sensor.dist);

            self.sensors.push(sensor);
        }
    }

    fn star1(self) -> String {
        let y = if self.min_x == -8 { 10 } else { 2_000_000 };

        let mut counter = 0;

        println!("{} --- {}", self.min_x, self.max_x);

        for x in self.min_x..self.max_x {
            let c = Coord { x, y };

            for s in self.sensors.iter() {
                if c.eq(&s.position) {
                    continue;
                }
                if c.eq(&s.beacon) {
                    continue;
                }

                if c.dist(&s.position) <= s.dist {
                    // println!("{c:?} -- {:?}", s.position);

                    counter += 1;
                    break;
                }
            }
        }

        counter.to_string()
    }

    fn star2(self) -> String {
        let max_size = if self.min_x == -8 { 20 } else { 4_000_000 };

        let mut intersections: Vec<Coord> = vec![];

        for i in 0..self.sensors.len() {
            for j in i + 1..self.sensors.len() {
                let dist = self.sensors[i].position.dist(&self.sensors[j].position);

                // use dist +1 to intersect on result.
                let i_dist = self.sensors[i].dist + 1;
                let j_dist = self.sensors[j].dist + 1;

                if dist > i_dist + j_dist {
                    println!("disjoint {i},{j}");
                    continue;
                }
                if dist < (i_dist - j_dist).abs() {
                    println!("include {i},{j}");
                    continue;
                }

                for k in 0..i_dist {
                    let p = Coord {
                        x: self.sensors[i].position.x + k,
                        y: self.sensors[i].position.y + i_dist - k,
                    };

                    if p.dist(&self.sensors[j].position) == j_dist {
                        intersections.push(p.clone());
                    }

                    let p = Coord {
                        x: self.sensors[i].position.x + k,
                        y: self.sensors[i].position.y - i_dist + k,
                    };

                    if p.dist(&self.sensors[j].position) == j_dist {
                        intersections.push(p.clone());
                    }

                    let p = Coord {
                        x: self.sensors[i].position.x - k,
                        y: self.sensors[i].position.y - i_dist + k,
                    };

                    if p.dist(&self.sensors[j].position) == j_dist {
                        intersections.push(p.clone());
                    }

                    let p = Coord {
                        x: self.sensors[i].position.x - k,
                        y: self.sensors[i].position.y + i_dist - k,
                    };

                    if p.dist(&self.sensors[j].position) == j_dist {
                        intersections.push(p.clone());
                    }
                }
                // println!("{i},{j}");
                // println!("{intersections:?}");
            }
        }

        println!("{:?}", intersections.len());
        intersections.sort();
        intersections.dedup();
        println!("{:?}", intersections.len());

        'outer: for inter in intersections {
            if inter.x < 0 || inter.y < 0 || inter.x > max_size || inter.y > max_size {
                continue;
            }
            for s in self.sensors.iter() {
                if inter.eq(&s.position) {
                    continue 'outer;
                }
                if inter.eq(&s.beacon) {
                    continue 'outer;
                }

                if inter.dist(&s.position) <= s.dist {
                    continue 'outer;
                }
            }
            println!("{inter:?}");
            return inter.to_string();
        }

        "not found".to_string()
    }
}

impl Coord {
    fn dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn to_string(&self) -> String {
        (self.x * 4_000_000 + self.y).to_string()
    }
}
