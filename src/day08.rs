use rayon::prelude::*;
use std::collections::HashMap;

pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    let map = Map::parse(lines);

    map.count_steps_on_path()
}

fn answer_part_2(lines: &str) -> usize {
    let map = Map::parse(lines);

    map.count_ghost_steps_on_path()
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day08.txt")
}

struct Map {
    instruction_list: InstructionList,
    path: String,
}

impl Map {
    fn parse(lines: &str) -> Self {
        let mut l = lines.lines();
        let path = l.next().unwrap().trim().to_owned();
        let _ = l.next();
        let instruction_list = InstructionList::parse(l.collect());

        Map {
            path,
            instruction_list,
        }
    }

    fn count_steps_on_path(&self) -> usize {
        let mut current_location = "AAA".to_owned();
        let mut number_of_steps = 0;
        let path_iter = self.path.chars().cycle();

        for direction in path_iter {
            current_location = self
                .instruction_list
                .next_location(&current_location, direction);
            number_of_steps += 1;

            if current_location == *"ZZZ" {
                return number_of_steps;
            }
        }
        0
    }

    fn count_ghost_steps_on_path_to_any_z(&self, source: &str) -> usize {
        let mut current_location = source.to_string();
        let mut number_of_steps = 0;
        let path_iter = self.path.chars().cycle();
        for direction in path_iter {
            current_location = self
                .instruction_list
                .next_location(&current_location, direction);
            number_of_steps += 1;
            if current_location.ends_with('Z') {
                return number_of_steps;
            }
        }
        0
    }

    fn count_ghost_steps_on_path(&self) -> usize {
        let locations: Vec<String> = self
            .instruction_list
            .instructions
            .keys()
            .filter(|k| k.ends_with('A'))
            .cloned()
            .collect();
        println!("{locations:?}");

        let min_to_zs: Vec<usize> = locations
            .par_iter()
            .map(|l| self.count_ghost_steps_on_path_to_any_z(l))
            .collect();

        let min_to_zs_lcm = min_to_zs
            .iter()
            .copied()
            .reduce(num::integer::lcm)
            .unwrap();

        min_to_zs_lcm
    }
}

struct InstructionList {
    instructions: HashMap<String, InstructionDestination>,
}

impl InstructionList {
    fn next_location(&self, current_location: &str, direction: char) -> String {
        let current_instruction = self.instructions.get(current_location).unwrap();

        if direction == 'L' {
            current_instruction.left.clone()
        } else {
            current_instruction.right.clone()
        }
    }
    fn parse(lines: Vec<&str>) -> Self {
        let instructions = lines
            .iter()
            .map(|s| InstructionList::parse_line(s))
            .collect();
        InstructionList { instructions }
    }

    fn parse_line(line: &str) -> (String, InstructionDestination) {
        let (location, instructions) = line.split_once(" = ").unwrap();
        let cleaned_instructions = instructions.replace(['(', ')', ' '], "");
        let (left, right) = cleaned_instructions.split_once(',').unwrap();

        (
            location.to_owned(),
            InstructionDestination {
                left: left.to_owned(),
                right: right.to_owned(),
            },
        )
    }
}

struct InstructionDestination {
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use crate::day08::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 17873);
        assert_eq!(answer_part_2(lines), 15746133679061);
    }

    const SAMPLE_INSTRUCTIONS: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn test_navigation_simple() {
        assert_eq!(answer_part_1(SAMPLE_INSTRUCTIONS), 2);
    }

    #[test]
    fn test_navigation_repeat_instructions() {
        const REPEAT_INSTRUCTIONS: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;
        assert_eq!(answer_part_1(REPEAT_INSTRUCTIONS), 6);
    }

    const SIMULTANEOUS_INSTRUCTIONS: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn test_simultaneous_ghost_nav() {
        assert_eq!(answer_part_2(SIMULTANEOUS_INSTRUCTIONS), 6);
    }
}
