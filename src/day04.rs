use std::{collections::HashSet, error::Error};

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let input_string = get_input_string();
    let answer_part_1 = answer_part_1(input_string);
    let answer_part_2 = answer_part_2(input_string);
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}

fn answer_part_1(lines: &str) -> usize {
    lines
        .lines()
        .map(ScratchCard::parse)
        .map(|sc| sc.card_value())
        .sum()
}

fn answer_part_2(_lines: &str) -> usize {
    //lines.lines().map(parse_game_row).sum()
    0
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day04.txt")
}

#[derive(Debug, PartialEq)]
struct ScratchCard {
    number: usize,
    winning_numbers: HashSet<usize>,
    your_numbers: HashSet<usize>,
}

impl ScratchCard {
    fn parse(line: &str) -> Self {
        let (card, all_numbers) = line
            .strip_prefix("Card")
            .expect("line must have 'Card'")
            .trim()
            .split_once(':')
            .expect("Must have colon");
        let (winning, your_numbers) = all_numbers.split_once('|').expect("Must have |");
        Self {
            number: card.parse().expect("couldn't parse card as num"),
            winning_numbers: winning
                .split_whitespace()
                .map(|s| s.parse().expect("winning number unparseable"))
                .collect(),
            your_numbers: your_numbers
                .split_whitespace()
                .map(|s| s.parse().expect("your number unparseable"))
                .collect(),
        }
    }

    fn your_winning_numbers(&self) -> HashSet<usize> {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .copied()
            .collect()
    }

    fn card_value(&self) -> usize {
        let total_wins = self.your_winning_numbers().len();
        if total_wins < 1 {
            0
        } else {
            2_usize.saturating_pow(total_wins as u32 - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day04::{answer_part_1, answer_part_2, get_input_string, ScratchCard};
    use std::collections::HashSet;
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 23847);
        assert_eq!(answer_part_2(lines), 0);
    }

    #[test]
    fn test_short_example() {
        let lines = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(answer_part_1(lines), 13);
        assert_eq!(answer_part_2(lines), 0);
    }

    #[test]
    fn test_scratch_card_parse() {
        let line = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"#;
        let sc = ScratchCard::parse(line);
        let expected = ScratchCard {
            number: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            your_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        let your_winning_numbers = HashSet::from([48, 83, 86, 17]);
        assert_eq!(sc, expected);
        assert_eq!(sc.your_winning_numbers(), your_winning_numbers);
        assert_eq!(sc.your_winning_numbers(), your_winning_numbers);
        assert_eq!(sc.card_value(), 8);
    }
}
