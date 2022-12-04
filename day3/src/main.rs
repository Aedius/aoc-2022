use helper::{aoc1, aoc2, InputReader};

fn main() {
    aoc1!(Container, "day3", 157);
    aoc2!(Container, "day3", 157, 8349, 70);
}

#[derive(Default)]
struct Container {
    sum: usize,
    sum2: usize,
    elf1: Option<Vec<char>>,
    elf2: Option<Vec<char>>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        self.pack_of1(line);
        self.pack_of2(line);
    }

    fn star1(self) -> String {
        self.sum.to_string()
    }

    fn star2(self) -> String {
        self.sum2.to_string()
    }
}

impl Container {
    fn pack_of2(&mut self, line: &str) {
        if self.elf1.is_none() {
            let mut chars = line.chars();
            let mut pack = Vec::new();
            while let Some(c) = chars.next() {
                pack.push(c);
            }
            self.elf1 = Some(pack)
        } else if self.elf2.is_none() {
            let mut chars = line.chars();
            let mut pack = Vec::new();
            while let Some(c) = chars.next() {
                if self.elf1.as_ref().unwrap().contains(&c) && !pack.contains(&c) {
                    pack.push(c);
                }
            }
            self.elf2 = Some(pack)
        } else {
            let mut chars = line.chars();
            let mut pack = Vec::new();
            while let Some(c) = chars.next() {
                if self.elf2.as_ref().unwrap().contains(&c) && !pack.contains(&c) {
                    pack.push(c);
                }
            }

            for s in pack {
                self.sum2 += to_point(s) as usize;
            }
            self.elf1 = None;
            self.elf2 = None;
        }
    }

    fn pack_of1(&mut self, line: &str) {
        let l = line.len();
        let mut chars = line.chars();

        let mut pack1 = Vec::new();
        let mut same = Vec::new();

        for _ in 0..l / 2 {
            pack1.push(chars.next().unwrap());
        }

        for _ in 0..l / 2 {
            let c = chars.next().unwrap();
            if pack1.contains(&c) && !same.contains(&c) {
                same.push(c);
            }
        }

        for s in same {
            self.sum += to_point(s) as usize;
        }
    }
}

fn to_point(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_point() {
        assert_eq!(to_point('a'), 1);
        assert_eq!(to_point('z'), 26);
        assert_eq!(to_point('A'), 27);
        assert_eq!(to_point('Z'), 52);
        assert_eq!(to_point('p'), 16);
        assert_eq!(to_point('L'), 38);
        assert_eq!(to_point('P'), 42);
        assert_eq!(to_point('v'), 22);
        assert_eq!(to_point('t'), 20);
        assert_eq!(to_point('s'), 19);
    }
}
