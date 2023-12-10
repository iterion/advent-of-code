pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    let map = Map::parse(lines);
    map.steps_along_path() / 2
}

fn answer_part_2(lines: &str) -> usize {
    let map = Map::parse(lines);
    map.count_all_inside_coords()
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day10.txt")
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

// `|` is a *vertical pipe* connecting north and south.
// `-` is a *horizontal pipe* connecting east and west.
// `L` is a *90-degree bend* connecting north and east.
// `J` is a *90-degree bend* connecting north and west.
// `7` is a *90-degree bend* connecting south and west.
// `F` is a *90-degree bend* connecting south and east.
// `.` is *ground*; there is no pipe in this tile.
// `S` is the *starting position* of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Pipe {
    fn parse(item: char) -> Self {
        match item {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("weird map"),
        }
    }

    fn connects_to(&self) -> Vec<Direction> {
        match *self {
            Pipe::Vertical => vec![Direction::North, Direction::South],
            Pipe::Horizontal => vec![Direction::East, Direction::West],
            Pipe::NorthEast => vec![Direction::North, Direction::East],
            Pipe::NorthWest => vec![Direction::North, Direction::West],
            Pipe::SouthWest => vec![Direction::South, Direction::West],
            Pipe::SouthEast => vec![Direction::South, Direction::East],
            Pipe::Ground => vec![],
            Pipe::Start => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
        }
    }
}

struct Map {
    grid: Vec<Vec<Pipe>>,
    row_count: usize,
    col_count: usize,
}

impl Map {
    fn parse(lines: &str) -> Self {
        let grid: Vec<Vec<Pipe>> = lines.lines().map(Map::parse_line).collect();
        let row_count = grid.len();
        let col_count = grid.first().unwrap().len();
        Self {
            grid,
            row_count,
            col_count,
        }
    }

    fn parse_line(line: &str) -> Vec<Pipe> {
        line.chars().map(Pipe::parse).collect()
    }

    fn count_all_inside_coords(&self) -> usize {
        let path = self.path_points();
        let mut num_inside = 0;
        for x in 0..self.row_count - 1 {
            for y in 0..self.col_count - 1 {
                if self.is_coord_in_path((x, y), path.clone()) {
                    num_inside += 1;
                    println!("found coord in path ({x}, {y})");
                }
            }
        }

        num_inside
    }

    fn is_coord_in_path(&self, coord: Coord, path: Vec<Coord>) -> bool {
        let mut min_x = 0;
        let mut max_x = self.row_count;
        let mut min_y = 0;
        let mut max_y = self.col_count;
        for p in &path {
            // quit early if on path, we only want inside
            if coord == *p {
                return false;
            }
            min_x = std::cmp::min(min_x, p.0);
            max_x = std::cmp::max(max_x, p.0);
            min_y = std::cmp::min(min_y, p.1);
            max_y = std::cmp::max(max_y, p.1);
        }

        if coord.0 < min_x || coord.0 > max_x || coord.1 < min_y || coord.1 > max_y {
            return false;
        }

        let mut inside = false;
        // prepend last so we can iterate once
        let mut new_path = vec![*path.last().unwrap()];
        let mut path_copy = path.clone();
        new_path.append(&mut path_copy);

        // ray intersection algorithm I stole, adapted to exclude points on path above
        for window in new_path.windows(2) {
            let p1 = window[0];
            let p2 = window[1];
            let p1_x = p1.0 as f64;
            let p2_x = p2.0 as f64;
            let p1_y = p1.1 as f64;
            let p2_y = p2.1 as f64;
            let coord_x = coord.0 as f64;
            let coord_y = coord.1 as f64;
            if (p1_y > coord_y) != (p2_y > coord_y)
                && coord_x < (p2_x - p1_x) * (coord_y - p1_y) / (p2_y - p1_y) + p1_x
            {
                inside = !inside;
            }
        }

        inside
    }

    fn path_points(&self) -> Vec<Coord> {
        let start = self.find_starting_point();
        let mut prev = start;
        // pick any direction from start
        let mut current = *self.find_possible_paths(start).first().unwrap();
        let mut steps = vec![start, current];

        while current != start {
            let possible = self.find_possible_paths(current);
            let next = possible.iter().find(|path| **path != prev).unwrap();
            prev = current;
            current = *next;
            steps.push(*next);
        }

        steps
    }

    fn steps_along_path(&self) -> usize {
        let mut steps = 1;
        let start = self.find_starting_point();
        let mut prev = start;
        // pick any direction from start
        let mut current = *self.find_possible_paths(start).first().unwrap();

        while current != start {
            let possible = self.find_possible_paths(current);
            println!("{steps} so far {prev:?} -> {current:?}: {possible:?}");
            let next = possible.iter().find(|path| **path != prev).unwrap();
            prev = current;
            current = *next;
            steps += 1;
        }

        steps
    }

    /// Find 0-indexed starting (row, column)
    fn find_starting_point(&self) -> Coord {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, pipe) in row.iter().enumerate() {
                if pipe == &Pipe::Start {
                    return (i, j);
                }
            }
        }
        panic!("couldn't find start");
    }

    fn find_possible_paths(&self, coord: Coord) -> Vec<Coord> {
        let pipe = self.get(coord);
        //println!("{pipe:?}, {:?}", pipe.connects_to());
        let possible_coords = pipe
            .connects_to()
            .iter()
            // check if pipes actually connect
            .filter(|d| self.pipes_connect(coord, **d))
            .map(|d| self.coordinate_in_direction(coord, *d).unwrap())
            .collect();

        possible_coords
    }

    fn coordinate_in_direction(&self, coord: Coord, direction: Direction) -> Option<Coord> {
        match direction {
            Direction::West => {
                if coord.1 == 0 {
                    None
                } else {
                    Some((coord.0, coord.1 - 1))
                }
            }
            Direction::East => {
                if coord.1 == self.col_count - 1 {
                    None
                } else {
                    Some((coord.0, coord.1 + 1))
                }
            }
            Direction::North => {
                if coord.0 == 0 {
                    None
                } else {
                    Some((coord.0 - 1, coord.1))
                }
            }
            Direction::South => {
                if coord.0 == self.row_count - 1 {
                    None
                } else {
                    Some((coord.0 + 1, coord.1))
                }
            }
        }
    }

    fn pipes_connect(&self, coord: Coord, direction: Direction) -> bool {
        let start_pipe = self.get(coord);
        let end_pipe = match self.coordinate_in_direction(coord, direction) {
            Some(pipe) => self.get(pipe),
            None => return false,
        };
        let start_directions: Vec<Direction> = start_pipe.connects_to();

        if !start_directions.contains(&direction) {
            return false;
        }

        let end_directions: Vec<Direction> = end_pipe
            .connects_to()
            .iter()
            .map(|d| d.opposite())
            .collect();
        //println!("start: {start_directions:?} - end: {end_directions:?}");

        end_directions.contains(&direction)
    }

    fn get(&self, coord: Coord) -> Pipe {
        self.grid[coord.0][coord.1]
    }
}

type Coord = (usize, usize);

#[cfg(test)]
mod tests {
    use crate::day10::{answer_part_1, answer_part_2, get_input_string, Map};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 6951);
        assert_eq!(answer_part_2(lines), 563);
    }

    const SAMPLE_INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;
    #[test]
    fn test_find_possible_paths() {
        let map = Map::parse(SAMPLE_INPUT);
        //assert_eq!(map.find_possible_paths((2, 1)), vec![(1, 1), (2, 0)]);
        assert_eq!(map.find_possible_paths((4, 0)), vec![(3, 0), (4, 1)]);
    }

    #[test]
    fn test_pipes_connect() {
        let map = Map::parse(SAMPLE_INPUT);
        assert!(map.pipes_connect((2, 1), crate::day10::Direction::North));
        assert!(!map.pipes_connect((2, 1), crate::day10::Direction::South));
        assert!(map.pipes_connect((4, 0), crate::day10::Direction::East));
    }

    #[test]
    fn test_pipe_maze_distance() {
        assert_eq!(answer_part_1(SAMPLE_INPUT), 8);
    }

    const SAMPLE_INPUT_2: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    #[test]
    fn test_contained() {
        assert_eq!(answer_part_2(SAMPLE_INPUT_2), 4);
    }
}
