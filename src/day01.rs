use regex::Regex;
use std::error::Error;

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let input_string = include_str!("../inputs/day01.txt");
    let answer_part_1: usize = input_string
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut digits = s.chars().filter(|c| c.is_ascii_digit());
            let first = digits.next().unwrap();
            let maybe_last = digits.last();
            let last = if let Some(last) = maybe_last {
                last
            } else {
                first
            };
            let num: usize = if let Ok(num) = format!("{first}{last}").parse() {
                num
            } else {
                0
            };
            num
        })
        .sum();
    let answer_part_2 = parse_all_lines_v2(input_string);
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}

fn parse_all_lines_v2(lines: &str) -> usize {
    lines
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(calibration_line_to_number)
        .sum()
}

fn calibration_line_to_number(line: &str) -> usize {
    let cleaned_calibration_value = parse_word_digits_with_dupes(line);
    let mut digits = cleaned_calibration_value
        .chars()
        .filter(|c| c.is_ascii_digit());
    let digits2 = digits.clone();
    let first = digits.next().unwrap();
    let maybe_last = digits.last();
    let last = if let Some(last) = maybe_last {
        last
    } else {
        first
    };
    let str_version = format!("{first}{last}");
    println!(
        "{line} - {cleaned_calibration_value} - {:?} - {str_version}",
        digits2.collect::<Vec<char>>()
    );
    if let Ok(num) = str_version.parse() {
        num
    } else {
        panic!("WTF");
    }
}

fn parse_word_digits_with_dupes(cal_val: &str) -> String {
    let re = Regex::new("one|two|three|four|five|six|seven|eight|nine")
        .expect("failed to compile regex");
    let mut prev_end = 0;
    let mut cur_index = 0;
    let original_length = cal_val.len();
    let mut replaced_string = cal_val.to_owned();
    while let Some(match_val) = re.find_at(cal_val, cur_index) {
        let (start_index, end_index) = (match_val.start(), match_val.end());
        let offset = original_length - replaced_string.len();
        println!(
            "0 - {offset}, {prev_end}, {start_index}, {end_index}, {cur_index}, {replaced_string}"
        );
        let (start_offset, end_offset) = if prev_end >= start_index && start_index != 0 {
            // we're overlapping words, so we need to handle that
            //let original_offset = prev_end - (cur_index - 1);
            println!("subtracting !!");
            (
                end_index - (end_index - start_index + 1),
                end_index - offset,
            )
        } else {
            (start_index - offset, end_index - offset)
        };
        println!("1 - {offset}, {prev_end}, {start_offset}, {end_offset}, {cur_index}, {replaced_string}");
        let (start, _) = replaced_string.split_at(start_offset);
        let (_, end) = replaced_string.split_at(end_offset);
        cur_index = start_index + 1;
        let word_digit = parse_word_digits_safely(match_val.as_str());

        replaced_string = start.to_owned() + &word_digit + end;
        println!("2 - {offset}, {prev_end}, {start_offset}, {end_offset}, {cur_index}, {replaced_string}");
        prev_end = end_offset;
    }
    replaced_string.to_owned()
}

fn parse_word_digits_safely(cal_val: &str) -> String {
    cal_val
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

#[cfg(test)]
mod tests {
    use crate::day01::parse_all_lines_v2;
    #[test]
    fn test_parse_calibration_line() {
        let lines = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        assert_eq!(parse_all_lines_v2(lines), 281);
        let lines = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
        assert_eq!(parse_all_lines_v2(lines), 142);
        assert_eq!(parse_all_lines_v2("eighthree"), 83);
    }
}