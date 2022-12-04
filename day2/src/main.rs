use helper::{aoc1, aoc2, InputReader};
use Choice::*;

fn main() {
    aoc1!(Container, "day2", 15);
    aoc2!(Container, "day2", 15, 14163, 12);
}

#[derive(Default)]
struct Container {
    score1: usize,
    score2: usize,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        let mut s = line.split(' ');

        let elf = to_choice(s.next().unwrap());
        let a = s.next().unwrap();
        let response = to_choice(a);
        let response2 = to_choice2(elf, a);

        self.score1 += calculate_score(elf, response);
        self.score2 += calculate_score(elf, response2);
    }

    fn star1(self) -> String {
        self.score1.to_string()
    }

    fn star2(self) -> String {
        self.score2.to_string()
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

fn to_choice(i: &str) -> Choice {
    match i {
        "A" => Rock,
        "X" => Rock,
        "B" => Paper,
        "Y" => Paper,
        "C" => Scissor,
        "Z" => Scissor,
        _ => {
            panic!("bad letter {}", i)
        }
    }
}
fn to_choice2(elf: Choice, i: &str) -> Choice {
    match i {
        "X" => match elf {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
        },
        "Y" => match elf {
            Rock => Rock,
            Paper => Paper,
            Scissor => Scissor,
        },
        "Z" => match elf {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
        },
        _ => {
            panic!("bad letter {}", i)
        }
    }
}

fn win(a: Choice, b: Choice) -> Option<bool> {
    if a == b {
        return None;
    }

    if a == Rock && b == Scissor {
        return Some(true);
    }
    if a == Scissor && b == Paper {
        return Some(true);
    }
    if a == Paper && b == Rock {
        return Some(true);
    }

    Some(false)
}

fn calculate_score(elf: Choice, response: Choice) -> usize {
    let mut score = 0;

    match response {
        Rock => {
            score += 1;
        }
        Paper => {
            score += 2;
        }
        Scissor => {
            score += 3;
        }
    }

    match win(response, elf) {
        Some(false) => {
            score += 0;
        }
        None => {
            score += 3;
        }
        Some(true) => {
            score += 6;
        }
    }

    score
}
