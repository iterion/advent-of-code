use std::error::Error;
const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let input_string = include_str!("../inputs/day02.txt");
    let answer_part_1 = sum_all_possible_games(input_string);
    let answer_part_2 = sum_all_possible_line_powers(input_string);
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}

fn get_row_power(lines: &str) -> usize {
    let mut game_split = lines.split(":");
    let game = game_split.next().unwrap();
    let value = game_split.next().unwrap().trim();
    let possible = get_min_for_colors(value);
    println!("game: {game:?}, value: {value:?}, possible: {possible:?}");
    possible.0 * possible.1 * possible.2
}

fn get_min_for_colors(line: &str) -> (usize, usize, usize) {
    let mut colors = (0, 0, 0);
    let mut drawings = line.split(";");
    while let Some(drawing) = drawings.next() {
        let mut color_values = drawing.split(",");
        while let Some(color_value) = color_values.next() {
            let mut splits = color_value.split_whitespace();
            let num: usize = splits.next().unwrap().parse().unwrap();
            match splits.next().unwrap() {
                "red" => colors.0 = std::cmp::max(colors.0, num),
                "green" => colors.1 = std::cmp::max(colors.1, num),
                "blue" => colors.2 = std::cmp::max(colors.2, num),
                _ => panic!("couldn't find color"),
            };
        }
    }
    colors
}

fn sum_all_possible_line_powers(lines: &str) -> usize {
    lines.lines().map(get_row_power).sum()
}

fn sum_all_possible_games(lines: &str) -> usize {
    lines.lines().map(parse_game_row).sum()
}

fn parse_game_row(lines: &str) -> usize {
    let mut game_split = lines.split(":");
    let game = get_game_value(game_split.next().unwrap());
    let value = game_split.next().unwrap().trim();
    let possible = are_games_possible(value);
    println!("game: {game:?}, value: {value:?}, possible: {possible}");
    if possible {
        game
    } else {
        0
    }
}

fn get_game_value(game_value: &str) -> usize {
    let mut splits = game_value.split_whitespace();
    let _ = splits.next();
    let val = splits.next().unwrap();
    let return_val: usize = val.parse().unwrap();
    return_val
}

fn are_games_possible(line: &str) -> bool {
    line.split(";").all(is_game_possible)
}

fn is_game_possible(line: &str) -> bool {
    line.splitn(3, ",").all(is_valid_color)
}

fn is_valid_color(draw: &str) -> bool {
    let mut splits = draw.split_whitespace();
    let num: usize = splits.next().unwrap().parse().unwrap();
    let color_limit = match splits.next().unwrap() {
        "blue" => MAX_BLUE,
        "red" => MAX_RED,
        "green" => MAX_GREEN,
        _ => panic!("couldn't find color"),
    };
    num <= color_limit
}

#[cfg(test)]
mod tests {
    use crate::day02::{sum_all_possible_games, get_min_for_colors, are_games_possible, is_game_possible, sum_all_possible_line_powers, parse_game_row, get_row_power};
    #[test]
    fn test_all_lines() {
        let lines = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(sum_all_possible_games(&lines), 8);
        assert_eq!(sum_all_possible_line_powers(&lines), 2286);
    }

    #[test]
    fn test_parse_game_row() {
        assert_eq!(
            parse_game_row("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            parse_game_row(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            0
        );
    }

    #[test]
    fn test_get_row_powers() {
        assert_eq!(
            get_row_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            get_row_power(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            1560
        );
    }

    #[test]
    fn test_get_min_for_colors() {
        assert_eq!(
            get_min_for_colors("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (4, 2, 6)
        );
        assert_eq!(
            get_min_for_colors(
                "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            (20, 13, 6)
        );
    }

    #[test]
    fn test_are_games_possible() {
        assert!(are_games_possible(
            "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        ));
        assert!(!are_games_possible(
            "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
        ));
    }

    #[test]
    fn test_is_game_possible() {
        assert!(is_game_possible("3 blue, 4 red"));
        assert!(!is_game_possible("8 green, 6 blue, 20 red"));
    }
}
