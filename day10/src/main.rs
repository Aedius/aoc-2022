use helper::{aoc1, aoc2, InputReader};
use regex::Regex;

fn main() {
    aoc1!(Container, "day10", 13140);
    aoc2!(
        Container,
        "day10",
        13140,
        17380,
        r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    );
}

struct Container {
    regex: Regex,
    cycle: usize,
    x: isize,
    strengths: isize,
    screen: Vec<char>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            regex: Regex::new("^addx (-?[0-9]+)$").unwrap(),
            cycle: 0,
            x: 1,
            strengths: 0,
            screen: vec![],
        }
    }
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line == "noop" {
            self.cycle();
        }
        if let Some(cap) = self.regex.captures(line) {
            self.cycle();
            self.cycle();
            self.x += cap[1].parse::<isize>().unwrap();
        }
    }

    fn star1(self) -> String {
        self.strengths.to_string()
    }

    fn star2(mut self) -> String {
        self.screen.reverse();
        self.screen.push('\n');
        self.screen.reverse();
        self.screen()
    }
}

impl Container {
    fn screen(&self) -> String {
        self.screen.iter().collect::<String>()
    }

    fn cycle(&mut self) {
        if self.x < 0 {
            self.screen.push('.');
        } else {
            let pos = self.cycle % 40;
            if pos.abs_diff(self.x as usize) <= 1 {
                self.screen.push('#');
            } else {
                self.screen.push('.');
            }
        }
        self.cycle += 1;
        if self.cycle % 40 == 20 {
            self.strengths += self.cycle as isize * self.x;
        }

        if self.cycle % 40 == 0 {
            self.screen.push('\n');
        }
    }
}
