use grid::{grid, Grid};
pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    let mut field = parse(lines);

    field.shift_load_north();
    field.print();
    field.calculate_load()
}

fn answer_part_2(lines: &str) -> usize {
    let mut field = parse(lines);

    let mut load_history = vec![];
    for _i in 0..1_000 {
        // north
        field.shift_load_north();
        field.grid.rotate_right();
        // west
        field.shift_load_north();
        field.grid.rotate_right();
        // south
        field.shift_load_north();
        field.grid.rotate_right();
        // east
        field.shift_load_north();
        field.grid.rotate_right();
        let new_load = field.calculate_load();
        load_history.push(new_load);
    }
    println!("{load_history:?}");
    let size = find_load_history_pattern_size(&load_history);
    println!("{size}");
    let history_len = load_history.len();
    let pattern = load_history[(history_len - size)..history_len].to_vec();

    let location = 1_000_000_000 % size;
    let current = (size - 1) - (history_len % size);
    println!("{pattern:?}");
    println!("{history_len} {location} {current}");
    pattern[location + current]
}

fn find_load_history_pattern_size(load_history: &Vec<usize>) -> usize {
    let history_len = load_history.len() - 1;
    for i in 3..500 {
        let mut comparison = load_history[(history_len - i)..history_len].to_vec();
        comparison.append(&mut comparison.clone());
        let comparison_size = comparison.len();
        if comparison
            .iter()
            .enumerate()
            .any(|(j, v)| {
                let offset = comparison_size - j;
                load_history[history_len - offset] != *v
            })
        {
            // we never found a mismatch, so this is the size of the pattern
            return i;
        }
    }
    0
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day14.txt")
}

#[derive(PartialEq, Clone, Copy)]
enum Space {
    Empty,
    RoundRock,
    CubeRock,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            'O' => Space::RoundRock,
            '#' => Space::CubeRock,
            _ => panic!("no such space type"),
        }
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Space::RoundRock => write!(f, "O"),
            Space::CubeRock => write!(f, "#"),
            Space::Empty => write!(f, "."),
        }
    }
}

struct FocusField {
    grid: Grid<Space>,
}

impl FocusField {
    fn calculate_load(&self) -> usize {
        let rows = self.grid.rows();
        let _cols = self.grid.cols();
        let mut total_load = 0;
        for ((row, _column), v) in self.grid.indexed_iter() {
            if v == &Space::RoundRock {
                let row_value = rows - row;
                total_load += row_value;
            }
        }

        total_load
    }

    fn shift_load_north(&mut self) {
        let rows = self.grid.rows();
        let cols = self.grid.cols();
        let _total_load = 0;

        for i in 0..(rows * cols) {
            let row = i / cols;
            let col = i % cols;
            let v = self.grid[(row, col)];
            if row == 0 {
                // don't need to shift first row
                continue;
            }
            if v == Space::RoundRock {
                // assume we can't move
                let mut last_possible_move = row;
                for j in (0..row).rev() {
                    let item_above = self.grid[(j, col)];
                    if item_above == Space::RoundRock || item_above == Space::CubeRock {
                        // can't move up more, use last possible
                        break;
                    } else {
                        last_possible_move = j;
                    }
                }

                if row != last_possible_move {
                    self.grid[(last_possible_move, col)] = Space::RoundRock;
                    self.grid[(row, col)] = Space::Empty;
                }
            }
        }
    }

    fn print(&self) {
        let mut output = String::new();
        let cols = self.grid.cols() - 1;
        for ((_, col), v) in self.grid.indexed_iter() {
            output.push_str(&v.to_string());
            if col == cols {
                output.push('\n');
            }
        }

        println!("{output}");
    }
}

fn parse(lines: &str) -> FocusField {
    let mut grid = grid![];
    let lines: Vec<Vec<Space>> = lines
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Space>>())
        .collect();
    for line in lines {
        grid.push_row(line);
    }

    FocusField { grid }
}

#[cfg(test)]
mod tests {
    use crate::day14::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 102497);
        assert_eq!(answer_part_2(lines), 105008);
    }

    const SAMPLE_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_reflector_dish_load() {
        assert_eq!(answer_part_1(SAMPLE_INPUT), 136);
        assert_eq!(answer_part_2(SAMPLE_INPUT), 64);
    }
}
