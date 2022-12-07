use helper::{aoc1, aoc2, InputReader};
use regex::Regex;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    aoc1!(Container, "day5", "CMZ");
    aoc2!(Container, "day5", "CMZ", "ZRLJGSCTR", "MCD");
}

type Crane = HashMap<usize, Vec<char>>;

struct Container {
    crane_9000: Crane,
    crane_9001: Crane,
    regex_crane: Regex,
    regex_move: Regex,

    max_size: usize,
    load_complete: bool,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            crane_9000: Default::default(),
            crane_9001: Default::default(),
            regex_crane: Regex::new(r"( {4}|\[([A-Z])\])").unwrap(),
            regex_move: Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap(),
            max_size: 0,
            load_complete: false,
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
                    Self::add_crane_row(index, char_vec.clone(), &mut self.crane_9000);
                    Self::add_crane_row(index, char_vec, &mut self.crane_9001);
                    self.max_size += 1;
                }
                index += 1;
            }
            return;
        }
        if !self.load_complete {
            self.print();
        }

        self.load_complete = true;

        if let Some(found) = self.regex_move.captures(line) {
            let nb = found[1].parse::<usize>().unwrap();
            let from = found[2].parse::<usize>().unwrap();
            let to = found[3].parse::<usize>().unwrap();
            Self::move_crate(&mut self.crane_9000, nb, from, to, true);
            Self::move_crate(&mut self.crane_9001, nb, from, to, false);
            self.print();
        }
    }

    fn star1(self) -> String {
        Self::get_result(self.crane_9000)
    }

    fn star2(self) -> String {
        Self::get_result(self.crane_9001)
    }
}

impl Container {
    fn add_crane_row(index: usize, char_vec: Vec<char>, crane: &mut HashMap<usize, Vec<char>>) {
        let column = crane.entry(index).or_insert(Vec::new());
        column.push(*char_vec.first().unwrap());
    }

    fn move_crate(
        crane: &mut HashMap<usize, Vec<char>>,
        nb: usize,
        from: usize,
        to: usize,
        one_by_one: bool,
    ) {
        let mut to_move: Vec<char>;
        {
            let from_crane = crane.entry(from).or_insert(Vec::new());

            to_move = from_crane[0..nb].to_owned();
            let rest = from_crane[nb..].to_owned();
            crane.insert(from, rest);
        }
        {
            let to_crane = crane.entry(to).or_insert(Vec::new());
            if one_by_one {
                to_move.reverse();
            }
            to_move.append(to_crane);
            crane.insert(to, to_move);
        }
    }

    fn get_result(crane: HashMap<usize, Vec<char>>) -> String {
        let mut list = Vec::new();

        let cap = crane.capacity();
        for i in 1..cap + 1 {
            list.push(
                crane
                    .get(&i)
                    .map(|list| list.first().unwrap())
                    .unwrap_or(&'0'),
            );
        }

        list.into_iter().filter(|c| *c != &'0').collect()
    }

    fn print(&self) {
        let cap = self.crane_9000.len();

        let crane_9000 = self.complete_crane(self.crane_9000.clone());
        let crane_9001 = self.complete_crane(self.crane_9001.clone());

        for i in 0..self.max_size {
            Self::print_row(cap, &crane_9000, i);
            print!("  ||  ");
            Self::print_row(cap, &crane_9001, i);
            print!("\n");
        }

        for i in 1..cap + 1 {
            print!(" {}  ", i);
        }
        print!("  ||  ");
        for i in 1..cap + 1 {
            print!(" {}  ", i);
        }
        print!("\n");

        sleep(Duration::from_millis(200));
    }

    fn print_row(cap: usize, crane: &HashMap<usize, Vec<char>>, i: usize) {
        for j in 1..cap + 1 {
            let c = crane.get(&j).unwrap();
            if c[i] == ' ' {
                print!("    ");
            } else {
                print!("[{}] ", c[i]);
            }
        }
    }

    fn complete_crane(&self, crane: HashMap<usize, Vec<char>>) -> HashMap<usize, Vec<char>> {
        let mut crane_9000 = HashMap::new();
        for (i, stack) in crane.clone() {
            let mut stack = stack.clone();
            stack.reverse();
            stack.resize(self.max_size, ' ');
            stack.reverse();
            crane_9000.insert(i, stack);
        }
        crane_9000
    }
}
