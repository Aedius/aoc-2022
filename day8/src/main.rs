use helper::{aoc1, aoc2, InputReader};

fn main() {
    aoc1!(Container, "day8", 21);
    aoc2!(Container, "day8", 21, 1647, 8);
}

#[derive(Default)]
struct Container {
    forest: Vec<Vec<char>>,
    is_visible: Vec<Vec<bool>>,
    nb_visible: Vec<Vec<usize>>,
}

impl InputReader for Container {
    fn add_line(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() == 0 {
            return;
        }
        self.forest.push(chars.clone());

        self.is_visible.push(Self::check_is_visible(&chars));

        self.nb_visible.push(Self::check_nb_visible(&chars));
    }

    fn star1(self) -> String {
        let rotate_forest = Self::rotate(self.forest);
        let row_visible = Self::rotate(self.is_visible);

        let mut column_visible = vec![];

        for i in 0..rotate_forest.len() {
            column_visible.push(Self::check_is_visible(&rotate_forest[i].clone()))
        }

        let mut total_visible = 0;

        for i in 0..row_visible.len() {
            for j in 0..row_visible[0].len() {
                if row_visible[i][j] || column_visible[i][j] {
                    total_visible += 1;
                }
            }
        }

        total_visible.to_string()
    }

    fn star2(self) -> String {
        let rotate_forest = Self::rotate(self.forest);
        let row_visible = Self::rotate(self.nb_visible);

        let mut column_visible = vec![];

        for i in 0..rotate_forest.len() {
            column_visible.push(Self::check_nb_visible(&rotate_forest[i].clone()))
        }

        let mut max_visible = 0;

        for i in 0..row_visible.len() {
            for j in 0..row_visible[0].len() {
                let nb_visible = row_visible[i][j] * column_visible[i][j];
                if nb_visible > max_visible {
                    max_visible = nb_visible;
                }
            }
        }

        max_visible.to_string()
    }
}

impl Container {
    fn check_is_visible_one_side(chars: &Vec<char>) -> Vec<bool> {
        let mut left_visible = vec![true];
        let mut previous_biggest = chars[0];
        for i in 1..chars.len() {
            if chars[i] <= previous_biggest {
                left_visible.push(false);
            } else {
                left_visible.push(true);
                previous_biggest = chars[i]
            }
        }
        left_visible
    }

    fn check_is_visible(chars: &Vec<char>) -> Vec<bool> {
        let left_visible = Self::check_is_visible_one_side(&chars);
        let mut reverse_chars = chars.clone();
        reverse_chars.reverse();
        let mut right_visible = Self::check_is_visible_one_side(&reverse_chars);
        right_visible.reverse();
        let mut visible = vec![];
        for i in 0..chars.len() {
            visible.push(left_visible[i] || right_visible[i]);
        }
        visible
    }

    fn check_nb_visible_one_side(chars: &Vec<char>) -> Vec<usize> {
        let mut left_visible = vec![0];
        for i in 1..chars.len() {
            let mut nb_visible = 1;
            for j in 1..i {
                let k = i - j;
                if chars[k] >= chars[i] {
                    break;
                }
                nb_visible += 1;
            }
            left_visible.push(nb_visible);
        }
        left_visible
    }

    fn check_nb_visible(chars: &Vec<char>) -> Vec<usize> {
        let left_visible = Self::check_nb_visible_one_side(&chars);
        let mut reverse_chars = chars.clone();
        reverse_chars.reverse();
        let mut right_visible = Self::check_nb_visible_one_side(&reverse_chars);
        right_visible.reverse();
        let mut visible = vec![];
        for i in 0..chars.len() {
            visible.push(left_visible[i] * right_visible[i]);
        }

        visible
    }

    fn rotate<T: Clone>(forest: Vec<Vec<T>>) -> Vec<Vec<T>> {
        let mut rotate_forest = vec![];
        for i in 0..forest[0].len() {
            let mut row = vec![];
            for j in 0..forest.len() {
                row.push(forest[j][i].clone());
            }
            rotate_forest.push(row);
        }
        rotate_forest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let input = vec!['3', '0', '3', '7', '3'];
        assert_eq!(
            Container::check_nb_visible_one_side(&input),
            vec![0, 1, 2, 3, 1]
        );
    }
}
