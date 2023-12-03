use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex;
use std::error::Error;

lazy_static! {
    static ref SCHEMATIC_RE: Regex = Regex::new(r"(\d+)|([\*$&\#\-=\+%/@])").unwrap();
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
    let rows = lines.lines().enumerate().map(|(i, line)| get_schematic_values(i, line)).collect::<Vec<Vec<SchematicValue>>>();
    println!("{rows:?}");
    let mut symbols: HashMap<usize, Vec<Symbol>> = HashMap::new();
    let mut parts: HashMap<usize, Vec<PartNumber>> = HashMap::new();
    for row_items in rows {
        for value in row_items {
            match value {
                SchematicValue::Symbol(ref s) => {
                    if let Some(r) = symbols.get_mut(&s.row) {
                        r.push(s.clone());
                    } else {
                        symbols.insert(s.row, vec![s.clone()]);
                    }
                },
                SchematicValue::PartNumber(ref p) => {
                    if let Some(r) = parts.get_mut(&p.row) {
                        r.push(p.clone());
                    } else {
                        parts.insert(p.row, vec![p.clone()]);
                    }
                },
            };
        }
    }
    let mut running_part_total = 0;
    for (i, part_list) in parts.iter() {
        let relevant_symbols = {
            let mut relevant_symbols = vec![];
            // don't include if first row
            if i != &0 {
                if let Some(s) = symbols.get(&(i-1)) {
                    relevant_symbols.append(&mut s.clone())
                }
            }
            if let Some(s) = symbols.get(i) {
                relevant_symbols.append(&mut s.clone())
            }
            if let Some(s) = symbols.get(&(i+1)) {
                relevant_symbols.append(&mut s.clone())
            }
            relevant_symbols
        };
        for part in part_list {
            let mut part_number_valid = false;
            for symbol in &relevant_symbols.clone() {
                if part.start.saturating_sub(1) <= symbol.start && symbol.start <= part.end + 1 {
                    println!("found! row: {}, part: {}, sym_row: {}, {} <= {} <= {}", part.row, part.number, symbol.row, part.start, symbol.start, part.end);
                    part_number_valid = true;
                    break
                }
            }
            if part_number_valid {
                running_part_total += part.number;
            }
        }
    }
    running_part_total
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
        let line = ".....+.58.";
        let values = get_schematic_values(0, line);
        let expected_values = vec![
            SchematicValue::Symbol(Symbol {
                row: 0,
                start: 5,
            }),
            SchematicValue::PartNumber(PartNumber {
                row: 0,
                number: 58,
                start: 7,
                end: 8,
            }),
        ];
        assert_eq!(values, expected_values);
    }
}
