use helper::{aoc1, aoc2, InputReader};

fn main() {
    aoc1!(Container, "day12", 31);
    aoc2!(Container, "day12", 31, 425, 29);
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Pos {
    col: usize,
    row: usize,
}

#[derive(Default, Debug)]
struct Container {
    map: Vec<Vec<usize>>,
    path: Vec<Vec<Option<usize>>>,
    current: Vec<Pos>,
    target: Option<Pos>,
    start: Pos,
}

impl InputReader for Container {
    fn after_all_line(&mut self) {
        let max = Pos {
            col: self.map[0].len() - 1,
            row: self.map.len() - 1,
        };

        let mut step = 1;

        while !self.iterate(max, step, true) {
            step += 1;
        }

        self.print_map();
        self.print_path();
    }

    fn add_line(&mut self, line: &str) {
        let mut row: Vec<char> = line.chars().collect();
        if row.is_empty() {
            return;
        }
        self.path.push(vec![None; row.len()]);
        if row.contains(&'S') || row.contains(&'E') {
            for i in 0..row.len() {
                if row[i] == 'S' {
                    self.current.push(Pos {
                        col: i,
                        row: self.map.len(),
                    });
                    self.path[i][self.map.len()] = Some(0);
                    row[i] = 'a';
                    self.start = Pos {
                        col: i,
                        row: self.map.len(),
                    };
                } else if row[i] == 'E' {
                    self.target = Some(Pos {
                        col: i,
                        row: self.map.len(),
                    });
                    row[i] = 'z';
                }
            }
        }

        self.map
            .push(row.into_iter().map(|c| c as usize - 97).collect());
    }

    fn star1(self) -> String {
        self.path[self.target.unwrap().row][self.target.unwrap().col]
            .unwrap_or(0)
            .to_string()
    }

    fn star2(mut self) -> String {
        self.path = vec![vec![None; self.map[0].len()]; self.map.len()];

        self.start = self.target.unwrap();
        self.target = None;
        let max = Pos {
            col: self.map[0].len() - 1,
            row: self.map.len() - 1,
        };

        let mut step = 1;

        while !self.iterate(max, step, false) {
            step += 1;
        }

        self.print_map();
        self.print_path();

        step.to_string()
    }
}

impl Container {
    fn iterate(&mut self, max: Pos, step: usize, go_up: bool) -> bool {
        let current = self.current.clone();
        self.current = Vec::new();

        for pos in current.iter() {
            let heigt = self.map[pos.row][pos.col];
            for n in pos.get_neighbour(max).iter() {
                if self.path[n.row][n.col].is_some() {
                    // already passed here
                    continue;
                }
                if go_up {
                    if self.map[n.row][n.col] > heigt + 1 {
                        // to high
                        continue;
                    }
                } else {
                    if self.map[n.row][n.col] < heigt - 1 {
                        // to low
                        continue;
                    }
                }
                self.current.push(n.clone());
                self.path[n.row][n.col] = Some(step);
                match self.target {
                    None => {
                        if self.map[n.row][n.col] == 0 {
                            return true;
                        }
                    }
                    Some(target) => {
                        if n == &target {
                            return true;
                        }
                    }
                }
            }
        }

        // self.print_path();
        false
    }

    fn print_path(&self) {
        for i in 0..self.path.len() {
            for j in 0..self.path[i].len() {
                print!(" {} ", self.path[i][j].unwrap_or(0));
            }
            println!();
        }
        println!();
        println!();
    }

    fn print_map(&self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                print!(" {} ", self.map[i][j]);
            }
            println!();
        }
        println!();
        println!();
    }
}

impl Pos {
    fn get_neighbour(&self, max: Pos) -> Vec<Pos> {
        let mut n = vec![];

        if self.row > 0 {
            n.push(Pos {
                row: self.row - 1,
                col: self.col,
            })
        }
        if self.col > 0 {
            n.push(Pos {
                row: self.row,
                col: self.col - 1,
            })
        }
        if self.row < max.row {
            n.push(Pos {
                row: self.row + 1,
                col: self.col,
            })
        }
        if self.col < max.col {
            n.push(Pos {
                row: self.row,
                col: self.col + 1,
            })
        }

        n
    }
}
