use helper::{aoc1, aoc2, InputReader};

fn main() {
    aoc1!(Container, "day6", 7);
    aoc2!(Container, "day6", 7, 1647, 19);
}

#[derive(Default)]
struct Container {
    res1: usize,
    res2: usize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        if line.len() == 0 {
            return;
        }
        let chars: Vec<char> = line.chars().into_iter().collect();

        self.res1 = Self::calculate(chars.clone(), 4);
        self.res2 = Self::calculate(chars.clone(), 14);
    }

    fn star1(self) -> String {
        self.res1.to_string()
    }

    fn star2(self) -> String {
        self.res2.to_string()
    }
}

impl Container {
    fn calculate(chars: Vec<char>, n: usize) -> usize {
        let mut res = 0;
        for list in chars.windows(n) {
            let mut l: Vec<char> = list.into_iter().map(|c| *c).collect();
            l.sort();
            l.dedup();
            if l.len() == n {
                res += n;
                break;
            }
            res += 1;
        }
        res
    }
}
