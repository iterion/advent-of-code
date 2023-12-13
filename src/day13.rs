pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    AllMaps::parse(lines).sum_values()
}

fn answer_part_2(lines: &str) -> usize {
    lines.len()
}

struct Map {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Map {
    fn from_rows(rows: Vec<String>) -> Self {
        let mut rows_iter = rows.iter();
        let first_row = rows_iter.next().unwrap();
        let mut cols: Vec<String> = first_row.chars().map(|c| c.to_string()).collect();
        for row in rows_iter {
            for (i, c) in row.chars().enumerate() {
                cols[i].push(c);
            }
        }
        Self { rows, cols }
    }

    fn find_value(&self) -> usize {
        if let Some(value) = self.horizontal_mirror_value() {
            value
        } else if let Some(value) = self.vertical_mirror_value() {
            value
        } else {
            println!("Couldn't horizontal or vertical mirror!");
            for r in &self.rows {
                println!("{r}");
            }
            0
        }
    }

    fn horizontal_mirror_value(&self) -> Option<usize> {
        let row_count = self.rows.len();
        for row in 0..row_count - 1 {
            if self.rows[row] == self.rows[row + 1] {
                if self.is_valid_horizontal_mirror(row) {
                    return Some((row + 1) * 100);
                }
            }
        }
        None
    }

    fn is_valid_horizontal_mirror(&self, start: usize) -> bool {
        let mut a = start as isize - 1;
        let mut b = start + 2;
        while a >= 0 && b < self.rows.len() {
            if self.rows[a as usize] != self.rows[b] {
                return false;
            }
            a -= 1;
            b += 1;
        }

        true
    }

    fn vertical_mirror_value(&self) -> Option<usize> {
        let col_count = self.cols.len();
        for col in 0..col_count - 1 {
            if self.cols[col] == self.cols[col + 1] {
                if self.is_valid_vertical_mirror(col) {
                    return Some(col + 1);
                }
            }
        }
        None
    }

    fn is_valid_vertical_mirror(&self, start: usize) -> bool {
        let mut a = start as isize - 1;
        let mut b = start + 2;
        while a >= 0 && b < self.cols.len() {
            if self.cols[a as usize] != self.cols[b] {
                return false;
            }
            a -= 1;
            b += 1;
        }

        true
    }
}

struct AllMaps {
    maps: Vec<Map>,
}

impl AllMaps {
    fn parse(lines: &str) -> Self {
        let mut maps = vec![];
        let mut rows = vec![];
        for line in lines.lines() {
            if line.len() == 0 {
                maps.push(Map::from_rows(rows.clone()));
                rows = vec![];
            } else {
                rows.push(line.to_owned())
            }
        }
        // may not have a trailing empty line
        if rows.len() != 0 {
            maps.push(Map::from_rows(rows.clone()));
        }
        Self { maps }
    }

    fn sum_values(&self) -> usize {
        self.maps.iter().map(|m| m.find_value()).sum()
    }
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day13.txt")
}

#[cfg(test)]
mod tests {
    use crate::day13::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }

    const SAMPLE_INPUT: &'static str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_point_of_incidence() {
        assert_eq!(answer_part_1(SAMPLE_INPUT), 405);
    }
}
