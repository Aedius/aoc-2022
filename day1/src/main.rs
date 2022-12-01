use helper::{aoc1, aoc2, InputReader};

fn main() {
    aoc1!(Container, "day1", 24000);
    aoc2!(Container, "day1", 24000, 68775, 45000);
}

#[derive(Default)]
struct Container{
    elves: Vec<usize>,
    current_elf: usize,
    max : usize,
}

impl InputReader for Container{
    fn add_line(&mut self, line: &str) {
        if line == "" {
            if self.current_elf > self.max {
                self.max = self.current_elf;
            }
            self.elves.push(self.current_elf);
            self.current_elf = 0;
        } else {
            self.current_elf += line.parse::<usize>().unwrap();
        }
    }

    fn star1(self) -> String {
        format!("{}",  self.max)
    }

    fn star2(mut self) -> String {
        self.elves.sort_by(|a,b| b.cmp(a));

        let mut  sum = 0;
        for i in 0..3{
            sum += self.elves[i];
        }

        format!("{}",  sum)
    }
}

