use helper::{aoc1, aoc2, InputReader};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering::Less;
use std::cmp::{min, Ordering};

fn main() {
    aoc1!(Container, "day13", 13);
    aoc2!(Container, "day13", 13, 5806, 140);
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Ord)]
#[serde(untagged)]
enum Packet {
    I(usize),
    L(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;
        match self {
            I(x) => match other {
                I(y) => x.partial_cmp(y),
                L(yyy) => L(vec![I(x.clone())]).partial_cmp(&L(yyy.clone())),
            },
            L(xxx) => match other {
                I(y) => L(xxx.clone()).partial_cmp(&L(vec![I(y.clone())])),
                L(yyy) => {
                    let min = min(xxx.len(), yyy.len());

                    for i in 0..min {
                        let compare = xxx[i].partial_cmp(&yyy[i]);
                        match compare {
                            None => return None,
                            Some(ord) => {
                                if ord.is_ne() {
                                    return compare;
                                }
                            }
                        }
                    }
                    xxx.len().partial_cmp(&yyy.len())
                }
            },
        }
    }
}

#[derive(Default)]
struct Container {
    first: Option<Packet>,
    index: usize,
    sum1: usize,
    all_packet: Vec<Packet>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            println!();
            return;
        }

        let v: Packet = serde_json::from_str(line).unwrap();
        self.all_packet.push(v.clone());
        println!("{v:?}");

        match self.first.clone() {
            None => {
                self.first = Some(v);
                return;
            }
            Some(first) => {
                self.index += 1;

                if first.partial_cmp(&v) == Some(Less) {
                    self.sum1 += self.index;
                }

                println!("{} - {:?}", self.index, first.partial_cmp(&v));
            }
        }
        self.first = None;
    }

    fn star1(self) -> String {
        self.sum1.to_string()
    }

    fn star2(mut self) -> String {
        let start: Packet = serde_json::from_str("[[2]]").unwrap();
        let end: Packet = serde_json::from_str("[[6]]").unwrap();

        self.all_packet.push(start.clone());
        self.all_packet.push(end.clone());

        self.all_packet.sort();
        println!();
        for p in self.all_packet.clone() {
            println!("{p:?}");
        }

        let start_pos = self
            .all_packet
            .clone()
            .iter()
            .position(|p| p == &start)
            .unwrap()
            + 1;
        let end_pos = self
            .all_packet
            .clone()
            .iter()
            .position(|p| p == &end)
            .unwrap()
            + 1;

        (start_pos * end_pos).to_string()
    }
}
