use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex;
use std::error::Error;

lazy_static! {
    static ref SCHEMATIC_RE: Regex = Regex::new(r"(\d+)|([\*$&\#\-=+%/@])").unwrap();
}

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let input_string = get_input_string();
    let answer_part_1 = answer_part_1(input_string);
    let answer_part_2 = answer_part_2(input_string);
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}

fn answer_part_1(lines: &str) -> usize {
    let rows = lines.lines().enumerate().map(|(i, line)| get_schematic_values(i, line));
    let mut symbols: HashMap<usize, Vec<SchematicValue>> = HashMap::new();
    let mut parts: HashMap<usize, Vec<SchematicValue>> = HashMap::new();
    for row_items in rows {
        for value in row_items {
            match value {
                SchematicValue::Symbol(ref s) => {
                    if let Some(r) = symbols.get_mut(&s.row) {
                        r.push(value);
                    } else {
                        symbols.insert(s.row, vec![value]);
                    }
                },
                SchematicValue::PartNumber(ref p) => {
                    if let Some(r) = parts.get_mut(&p.row) {
                        r.push(value);
                    } else {
                        parts.insert(p.row, vec![value]);
                    }
                },
            };
        }
    }
    for (i, part_list) in parts.iter() {
        let mut relevant_symbols = vec![];
        if let Some(s) = symbols.get_mut(&(i-1)) {
            relevant_symbols.append(s)
        }
        //let this_row_symbols = symbols.get(i).unwrap_or(&vec![]);
        //let next_row_symbols = symbols.get(&(i+1)).unwrap_or(&vec![]);
        for part in part_list {
            for symbol in &relevant_symbols {
            }
        }
    }
    //println!("{values:?}");
    0
}

fn answer_part_2(_lines: &str) -> usize {
    //lines.lines().map(parse_game_row).sum()
    0
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day03.txt")
}

fn get_schematic_values(row: usize, line: &str) -> Vec<SchematicValue> {
    SCHEMATIC_RE
        .find_iter(line)
        .map(|val| {
            let s = val.as_str();
            if s.starts_with('*') ||
                s.starts_with('#') ||
                s.starts_with('+') ||
                s.starts_with('&') ||
                s.starts_with('$') ||
                s.starts_with('-') ||
                s.starts_with('+') ||
                s.starts_with('%') ||
                s.starts_with('@') ||
                s.starts_with('=') ||
                s.starts_with('/')
            {
                SchematicValue::Symbol(Symbol { row, start: val.start() })
            } else {
                let num: usize = match s.parse() {
                    Ok(n) => n,
                    Err(_) => panic!("did not expect number {s}"),
                };
                SchematicValue::PartNumber(PartNumber { row, number: num, start: val.start(), end: val.end() - 1 })
            }

        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
enum SchematicValue {
    PartNumber(PartNumber),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
struct PartNumber {
        row: usize,
        number: usize,
        start: usize,
        end: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Symbol {
        row: usize,
        start: usize,
}

#[cfg(test)]
mod tests {
    use crate::day03::{
        answer_part_1, answer_part_2, get_input_string, get_schematic_values, SchematicValue, PartNumber, Symbol,
    };
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }

    #[test]
    fn test_short_example() {
        let lines = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(answer_part_1(lines), 4361);
        assert_eq!(answer_part_2(lines), 0);
    }

    #[test]
    fn test_line_one() {}

    #[test]
    fn test_get_schematic_values() {
        let line = "467..114..";
        let values = get_schematic_values(0, line);
        let expected_values = vec![
            SchematicValue::PartNumber(PartNumber {
                row: 0,
                number: 467,
                start: 0,
                end: 2,
            }),
            SchematicValue::PartNumber(PartNumber {
                row: 0,
                number: 114,
                start: 5,
                end: 7,
            }),
        ];
        assert_eq!(values, expected_values);
        let line = "617*......";
        let values = get_schematic_values(0, line);
        let expected_values = vec![
            SchematicValue::PartNumber(PartNumber {
                row: 0,
                number: 617,
                start: 0,
                end: 2,
            }),
            SchematicValue::Symbol(Symbol {
                row: 0,
                start: 3,
            }),
        ];
        assert_eq!(values, expected_values);
    }
}
