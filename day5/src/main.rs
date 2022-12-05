use helper::{aoc1, aoc2, InputReader};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    aoc1!(Container, "day5", "CMZ");
    //    aoc2!(Container, "day5", 0, 0, 0);
}

struct Container {
    crane: HashMap<usize, Vec<char>>,
    regex_crane: Regex,
    regex_move: Regex,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            crane: Default::default(),
            regex_crane: Regex::new(r"( {4}|\[([A-Z])\])").unwrap(),
            regex_move: Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap(),
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if self.regex_crane.is_match(line) {
            let mut index = 1;
            for cap in self.regex_crane.captures_iter(line) {
                if &cap[0] != "    " {
                    let char_vec: Vec<char> = cap[2].chars().collect();
                    let column = self.crane.entry(index).or_insert(Vec::new());
                    column.push(*char_vec.first().unwrap());
                }
                index += 1;
            }
        }
        if let Some(found) = self.regex_move.captures(line) {
            let nb = found[1].parse::<usize>().unwrap();
            let from = found[2].parse::<usize>().unwrap();
            let to = found[3].parse::<usize>().unwrap();

            let mut to_move = Vec::new();
            {
                let from_crane = self.crane.entry(from).or_insert(Vec::new());

                to_move = from_crane[0..nb].to_owned();
                let rest = from_crane[nb..].to_owned();
                self.crane.insert(from, rest);
            }
            {
                let to_crane = self.crane.entry(to).or_insert(Vec::new());
                to_move.reverse();
                to_move.append(to_crane);
                self.crane.insert(to, to_move);
            }
        }
    }

    fn star1(self) -> String {
        let mut list = Vec::new();

        let cap = self.crane.capacity();
        for i in 1..cap + 1 {
            list.push(
                self.crane
                    .get(&i)
                    .map(|list| list.first().unwrap())
                    .unwrap_or(&'0'),
            );
        }

        list.into_iter().filter(|c| *c != &'0').collect()
    }

    fn star2(self) -> String {
        todo!()
    }
}
