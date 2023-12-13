pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    AllMaps::parse(lines).sum_values(false)
}

fn answer_part_2(lines: &str) -> usize {
    AllMaps::parse(lines).sum_values(true)
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

    fn find_value(&self, v2: bool) -> usize {
        if let Some(value) = self.horizontal_mirror_value(v2) {
            value
        } else if let Some(value) = self.vertical_mirror_value(v2) {
            value
        } else {
            println!("Couldn't horizontal or vertical mirror!");
            for r in &self.rows {
                println!("{r}");
            }
            0
        }
    }

    fn horizontal_mirror_value(&self, v2: bool) -> Option<usize> {
        let row_count = self.rows.len();
        for row in 0..row_count - 1 {
            let a = &self.rows[row];
            let b = &self.rows[row + 1];
            if a == b {
                if self.is_valid_horizontal_mirror(row, v2, false) {
                    return Some((row + 1) * 100);
                }
            } else if v2 {
                let distance = levenshtein::levenshtein(a, b);
                if distance <= 1 {
                    if self.is_valid_horizontal_mirror(row, v2, distance == 1) {
                        return Some((row + 1) * 100);
                    }
                }
            }
        }
        None
    }

    fn is_valid_horizontal_mirror(&self, start: usize, v2: bool, was_smudged: bool) -> bool {
        let mut was_smudged = was_smudged;
        let mut a = start as isize - 1;
        let mut b = start + 2;
        while a >= 0 && b < self.rows.len() {
            let row_a = &self.rows[a as usize];
            let row_b = &self.rows[b];
            if !v2 && row_a != row_b {
                return false;
            } else if v2 {
                let distance = levenshtein::levenshtein(row_a, row_b);
                // not close enough, always reject
                if distance > 1 {
                    return false;
                } else if distance == 1 {
                    // one smudge ok, two smudge get outta here
                    // wasn't true in practice, but oh well
                    if was_smudged {
                        return false;
                    }
                    was_smudged = true;
                }
            }
            a -= 1;
            b += 1;
        }

        if v2 {
            // if it wasn't smudged, we don't want it
            was_smudged
        } else {
            true
        }
    }

    fn vertical_mirror_value(&self, v2: bool) -> Option<usize> {
        let col_count = self.cols.len();
        for col in 0..col_count - 1 {
            let a = &self.cols[col];
            let b = &self.cols[col + 1];
            if a == b {
                if self.is_valid_vertical_mirror(col, v2, false) {
                    return Some((col + 1));
                }
            } else if v2 {
                let distance = levenshtein::levenshtein(a, b);
                if distance == 1 {
                    if self.is_valid_vertical_mirror(col, v2, true) {
                        return Some(col + 1);
                    }
                }
            }
        }
        None
    }

    fn is_valid_vertical_mirror(&self, start: usize, v2: bool, was_smudged: bool) -> bool {
        let mut was_smudged = was_smudged;
        let mut a = start as isize - 1;
        let mut b = start + 2;
        while a >= 0 && b < self.cols.len() {
            let col_a = &self.cols[a as usize];
            let col_b = &self.cols[b];
            if !v2 && col_a != col_b {
                return false;
            } else if v2 {
                let distance = levenshtein::levenshtein(col_a, col_b);
                // not close enough, always reject
                if distance > 1 {
                    return false;
                } else if distance == 1 {
                    // one smudge ok, two smudge get outta here
                    if was_smudged {
                        return false;
                    }
                    was_smudged = true;
                }
            }
            a -= 1;
            b += 1;
        }

        if v2 {
            // if it wasn't smudged, we don't want it
            was_smudged
        } else {
            true
        }
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

    fn sum_values(&self, v2: bool) -> usize {
        self.maps.iter().map(|m| m.find_value(v2)).sum()
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

        assert_eq!(answer_part_1(lines), 27202);
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
        assert_eq!(answer_part_2(SAMPLE_INPUT), 400);
    }
}
