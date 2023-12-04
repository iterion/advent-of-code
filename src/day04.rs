use std::error::Error;

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let input_string = get_input_string();
    let answer_part_1 = answer_part_1(input_string);
    let answer_part_2 = answer_part_2(input_string);
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}

fn answer_part_1(_lines: &str) -> usize {
    //lines.lines().map(get_row_power).sum()
    0
}

fn answer_part_2(_lines: &str) -> usize {
    //lines.lines().map(parse_game_row).sum()
    0
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day04.txt")
}

#[cfg(test)]
mod tests {
    use crate::day04::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }
}
