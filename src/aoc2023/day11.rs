use itertools::Itertools;
//use rayon::prelude::*;

pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    let mut map = Map::parse(lines);
    map.expand_universe(1);
    map.all_galaxy_distances()
}

fn answer_part_2(lines: &str) -> usize {
    let map = Map::parse(lines);
    let expanded = map.expanded_universe(1_000_000);
    expanded.all_galaxy_distances()
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day11.txt")
}

#[derive(Clone, PartialEq, Debug, Copy)]
enum Space {
    Galaxy,
    Empty,
}

impl Space {
    fn parse(space: char) -> Self {
        match space {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("unallowed space item"),
        }
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Space::Galaxy => write!(f, "#"),
            Space::Empty => write!(f, "."),
        }
    }
}

struct ExpandedUniverse {
    expansion_multiplier: usize,
    empty_columns: Vec<usize>,
    empty_rows: Vec<usize>,
    galaxy_locations: Vec<Point>,
}

impl ExpandedUniverse {
    fn all_galaxy_distances(&self) -> usize {
        self.galaxy_locations
            .iter()
            .combinations(2)
            .map(|points| {
                // how many times we cross empty space?
                let a = points[0].clone();
                let b = points[1].clone();
                self.expanded_distance(a, b)
            })
            .sum()
    }

    fn expanded_distance(&self, a: Point, b: Point) -> usize {
        let empty_rows_crossed = create_range(a.x, b.x)
            .filter(|row_index| self.empty_rows.contains(row_index))
            .count();
        let empty_columns_crossed = create_range(a.y, b.y)
            .filter(|col_index| self.empty_columns.contains(col_index))
            .count();
        let row_expansion =
            (empty_rows_crossed * self.expansion_multiplier).saturating_sub(empty_rows_crossed);
        let col_expansion = (empty_columns_crossed * self.expansion_multiplier)
            .saturating_sub(empty_columns_crossed);
        a.distance(b.clone()) + row_expansion + col_expansion
    }
}

// rust range only increments, must sort params first
fn create_range(a: usize, b: usize) -> std::ops::RangeInclusive<usize> {
    if a <= b {
        a..=b
    } else {
        b..=a
    }
}

struct Map {
    grid: Vec<Vec<Space>>,
    height: usize,
    width: usize,
}

impl Map {
    fn parse(lines: &str) -> Self {
        let grid: Vec<_> = lines.lines().map(Map::parse_line).collect();
        let height = grid.len();
        let width = grid.first().unwrap().len();
        Self {
            grid,
            height,
            width,
        }
    }

    fn parse_line(line: &str) -> Vec<Space> {
        line.chars().map(Space::parse).collect()
    }

    fn expanded_universe(&self, multiplier: usize) -> ExpandedUniverse {
        let empty_rows: Vec<usize> = self
            .grid
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|space| *space == Space::Empty))
            .map(|(i, _)| i)
            .collect();
        let mut empty_columns = vec![];
        for i in 0..self.width {
            let mut all_empty = true;
            for j in 0..self.height {
                if self.get(j, i) != Space::Empty {
                    all_empty = false;
                }
            }

            if all_empty {
                empty_columns.push(i);
            }
        }

        ExpandedUniverse {
            expansion_multiplier: multiplier,
            empty_rows,
            empty_columns,
            galaxy_locations: self.find_all_galaxies(),
        }
    }

    fn expand_universe(&mut self, expansion: usize) {
        let empty_rows: Vec<usize> = self
            .grid
            .iter()
            .enumerate()
            .filter(|(_, line)| line.iter().all(|space| *space == Space::Empty))
            .map(|(i, _)| i)
            .collect();

        let new_row: Vec<Space> = std::iter::repeat(Space::Empty).take(self.width).collect();

        let mut offset = 0;
        for i in empty_rows {
            for _n in 0..expansion {
                self.grid.insert(i + offset, new_row.clone());
                self.height += 1;
                offset += 1;
            }
        }

        let mut empty_column_ids = vec![];
        for i in 0..self.width {
            let mut all_empty = true;
            for j in 0..self.height {
                if self.get(j, i) != Space::Empty {
                    all_empty = false;
                }
            }

            if all_empty {
                empty_column_ids.push(i);
            }
        }

        offset = 0;
        for i in empty_column_ids {
            for _n in 0..expansion {
                for j in 0..self.height {
                    self.grid[j].insert(i + offset, Space::Empty);
                    // need to increment as we insert
                }
                self.width += 1;
                offset += 1;
            }
        }
    }

    fn find_all_galaxies(&self) -> Vec<Point> {
        let mut galaxy_locations = vec![];
        for x in 0..self.height {
            for y in 0..self.width {
                if self.get(x, y) == Space::Galaxy {
                    galaxy_locations.push(Point { x, y });
                }
            }
        }

        galaxy_locations
    }

    fn all_galaxy_distances(&self) -> usize {
        self.find_all_galaxies()
            .iter()
            .combinations(2)
            .map(|points| points[0].distance(points[1].clone()))
            .sum()
    }

    fn get(&self, x: usize, y: usize) -> Space {
        self.grid[x][y]
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let map = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{map}")
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: Point) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::{answer_part_1, answer_part_2, get_input_string, Map, Point};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 9957702);
        assert_eq!(answer_part_2(lines), 512240933238);
    }

    const SAMPLE_INPUT: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_universe_expansion() {
        let mut map = Map::parse(SAMPLE_INPUT);
        map.expand_universe(1);
        println!("{map}");
        assert_eq!(
            map.to_string(),
            r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
        );
    }

    #[test]
    fn test_shortest_paths() {
        let paths_sum = answer_part_1(SAMPLE_INPUT);
        assert_eq!(paths_sum, 374);
        let map = Map::parse(SAMPLE_INPUT);
        let expanded = map.expanded_universe(2);
        assert_eq!(expanded.all_galaxy_distances(), 374);
    }

    #[test]
    fn test_shortest_paths_expand_10() {
        let mut map = Map::parse(SAMPLE_INPUT);
        let expanded = map.expanded_universe(10);
        map.expand_universe(9);
        println!("{map}");
        assert_eq!(map.all_galaxy_distances(), 1030);
        assert_eq!(expanded.all_galaxy_distances(), 1030);
    }

    #[test]
    fn test_point_distance() {
        let g1 = Point { x: 0, y: 4 };
        let g3 = Point { x: 2, y: 0 };
        let g6 = Point { x: 7, y: 12 };
        let g7 = Point { x: 10, y: 9 };
        let g8 = Point { x: 11, y: 0 };
        let g9 = Point { x: 11, y: 5 };
        assert_eq!(g1.distance(g7), 15);
        assert_eq!(g3.distance(g6), 17);
        assert_eq!(g8.distance(g9), 5);
    }
}
