use helper::{aoc1, aoc2, InputReader};
use regex::Regex;
use Move::*;

fn main() {
    // aoc1!(Container, "day9", 13);
    aoc2!(Container, "day9/part2", 13, 36, 0);
}

// const SIZE: usize = 1001;
// const START: usize = 500;
// const PRINT :bool= false;

const SIZE: usize = 51;
const START: usize = 25;

const PRINT: bool = false;

struct Container {
    regex: Regex,
    field1: Vec<Vec<bool>>,
    field9: Vec<Vec<bool>>,
    rope: Vec<(usize, usize)>,
}

#[derive(Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_str(i: &str) -> Self {
        match i {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("{i} not nfound"),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self {
            regex: Regex::new(r"^(U|D|R|L) ([0-9]+)$").unwrap(),
            field1: vec![vec![false; SIZE]; SIZE],
            field9: vec![vec![false; SIZE]; SIZE],
            rope: vec![(START, START); 10],
        }
    }
}

impl Container {
    fn print(&self) {
        if !PRINT {
            return;
        }
        for i in 0..SIZE {
            for j in 0..SIZE {
                for p in 0..self.rope.len() {
                    if self.rope[p] == (i, j) {
                        print!("{p} ");
                    }
                }
                if self.head == (i, j) {
                    print!("H ");
                } else if self.tail == (i, j) {
                    print!("T ");
                } else if self.field1[i][j] {
                    print!("x ");
                } else {
                    print!(". ");
                }
            }
            print!("\n");
        }
        print!("\n---\n");
    }

    fn move_head(&mut self, m: Move, step: usize) {
        for _ in 0..step {
            self.move_one(m);
        }
    }

    fn move_one(&mut self, m: Move) {
        match m {
            Up => {
                self.head.0 -= 1;
            }
            Down => {
                self.head.0 += 1;
            }
            Left => {
                self.head.1 -= 1;
            }
            Right => {
                self.head.1 += 1;
            }
        }

        let previous = self.head;
        let mut to_move = self.tail[0];

        Self::calculate_next(previous, to_move);

        self.field1[self.rope[1].0][self.rope[1].0] = true;
        self.field9[self.rope[9].0][self.rope[9].0] = true;
    }

    fn calculate_next(previous: (usize, usize), mut to_move: (usize, usize)) {
        if previous.0.abs_diff(to_move.0) <= 1 && previous.1.abs_diff(to_move.1) <= 1 {
            return;
        }
        if previous.0.abs_diff(to_move.0) == 2 {
            to_move.0 = (to_move.0 + previous.0) / 2;
            if previous.1.abs_diff(to_move.1) == 1 {
                to_move.1 = previous.1;
            }
        }
        if previous.1.abs_diff(to_move.1) == 2 {
            to_move.1 = (to_move.1 + previous.1) / 2;
            if previous.0.abs_diff(to_move.0) == 1 {
                to_move.0 = previous.0;
            }
        }
    }
}

impl InputReader for Container {
    fn on_start(&mut self) {
        self.field1[self.tail.0][self.tail.1] = true;
    }

    fn add_line(&mut self, line: &str) {
        if let Some(cap) = self.regex.captures(line) {
            let m = Move::from_str(&cap[1]);
            let nb: usize = cap[2].parse().unwrap();
            self.move_head(m, nb);
        }
        self.print();
    }

    fn star1(self) -> String {
        let mut res = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.field1[i][j] {
                    res += 1;
                }
            }
        }

        res.to_string()
    }

    fn star2(self) -> String {
        let mut res = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.field9[i][j] {
                    res += 1;
                }
            }
        }

        res.to_string()
    }
}
