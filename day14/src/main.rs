use helper::{aoc1, aoc1_full, aoc2, aoc2_alone, InputReader};

const DISPLAY: bool = false;

fn main() {
    aoc1_full!(Container, "day14", 24, 683);
    aoc2_alone!(Container, "day14", 93);
}

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Sand,
    Empty,
}

#[derive(Default)]
struct Container {
    map: Vec<Vec<Tile>>,
    counter1: usize,
    counter2: usize,
}

impl InputReader for Container {
    fn on_start(&mut self) {
        self.map = vec![vec![Tile::Empty; 1000]; 10]
    }

    fn add_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        let split = line.split(" -> ");
        let positions: Vec<&str> = split.into_iter().collect();

        let mut previous: Option<(usize, usize)> = None;

        for position in positions {
            let mut position = position.split(",");
            let column: usize = position.next().unwrap().parse().unwrap();
            let row: usize = position.next().unwrap().parse().unwrap();

            self.update_size(column, row);

            if let Some(p) = previous.clone() {
                if p.0 == column {
                    if row > p.1 {
                        for i in p.1..row + 1 {
                            self.map[i][column] = Tile::Wall;
                        }
                    } else {
                        for i in row..p.1 + 1 {
                            self.map[i][column] = Tile::Wall;
                        }
                    }
                } else if p.1 == row {
                    if column > p.0 {
                        for i in p.0..column + 1 {
                            self.map[row][i] = Tile::Wall;
                        }
                    } else {
                        for i in column..p.0 + 1 {
                            self.map[row][i] = Tile::Wall;
                        }
                    }
                }
            }

            previous = Some((column, row));
        }

        self.display();
    }

    fn star1(mut self) -> String {
        while self.drop1() {
            self.display()
        }
        self.counter1.to_string()
    }

    fn star2(mut self) -> String {
        self.map.push(vec![Tile::Wall; 1000]);

        while self.drop2() {
            self.display()
        }

        self.counter2.to_string()
    }
}

impl Container {
    fn update_size(&mut self, column: usize, row: usize) {
        let height = self.map.len();
        if row + 2 > height {
            for _ in 0..(row + 2 - height) {
                self.map.push(vec![Tile::Empty; 1000]);
            }
        }
    }
    fn display(&self) {
        if !DISPLAY {
            return;
        }

        for row in 0..self.map.len() {
            for col in 450..550 {
                print!(
                    "{}",
                    match self.map[row][col] {
                        Tile::Wall => {
                            "#"
                        }
                        Tile::Sand => {
                            "o"
                        }
                        Tile::Empty => {
                            "."
                        }
                    }
                )
            }
            println!();
        }
    }

    fn drop1(&mut self) -> bool {
        let mut sand = (0, 500);

        while let Some(next_position) = self.next_position(sand) {
            if next_position.0 > self.map.len() {
                return false;
            }
            sand = next_position.clone();
        }
        self.map[sand.0][sand.1] = Tile::Sand;
        self.counter1 += 1;
        true
    }
    fn drop2(&mut self) -> bool {
        let mut sand = (0, 500);

        while let Some(next_position) = self.next_position(sand) {
            sand = next_position.clone();
        }
        self.counter2 += 1;
        if sand.0 == 0 && sand.1 == 500 {
            return false;
        }
        self.map[sand.0][sand.1] = Tile::Sand;
        true
    }

    fn next_position(&mut self, sand: (usize, usize)) -> Option<(usize, usize)> {
        if sand.0 + 1 >= self.map.len() {
            return Some((sand.0 + 1, sand.1));
        }

        if self.map[sand.0 + 1][sand.1] == Tile::Empty {
            return Some((sand.0 + 1, sand.1));
        }

        if self.map[sand.0 + 1][sand.1 - 1] == Tile::Empty {
            return Some((sand.0 + 1, sand.1 - 1));
        }
        if self.map[sand.0 + 1][sand.1 + 1] == Tile::Empty {
            return Some((sand.0 + 1, sand.1 + 1));
        }

        None
    }
}
