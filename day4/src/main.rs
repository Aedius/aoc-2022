use helper::{aoc1, aoc2, InputReader};
use regex::Regex;

fn main() {
    aoc1!(Container, "day4", 2);
    aoc2!(Container, "day4", 2, 542, 4);
}

struct Container {
    regex: Regex,
    contain: usize,
    overlap: usize,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            regex: Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap(),
            contain: 0,
            overlap: 0,
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if let Some(found) = self.regex.captures(line) {
            let s1 = found[1].parse::<usize>().unwrap();
            let e1 = found[2].parse::<usize>().unwrap();
            let s2 = found[3].parse::<usize>().unwrap();
            let e2 = found[4].parse::<usize>().unwrap();

            if (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2) {
                self.contain += 1;
                self.overlap += 1;
            } else if (s1 <= s2 && s2 <= e1) || (s1 <= e2 && e2 <= e1) {
                self.overlap += 1;
            }
        }
    }

    fn star1(self) -> String {
        self.contain.to_string()
    }

    fn star2(self) -> String {
        self.overlap.to_string()
    }
}
