pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    let answer: isize = lines
        .lines()
        .map(SensorReport::parse)
        .map(|sr| sr.get_next_value())
        .sum();
    answer as usize
}

fn answer_part_2(lines: &str) -> usize {
    let answer: isize = lines
        .lines()
        .map(SensorReport::parse)
        .map(|sr| sr.get_previous_value())
        .sum();
    answer as usize
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day09.txt")
}

struct SensorReport {
    history: Vec<isize>,
}

impl SensorReport {
    fn parse(line: &str) -> Self {
        Self {
            history: line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    fn get_next_value(&self) -> isize {
        let mut diffs = self.history.clone();
        let mut found_zeros = false;
        let mut last_values = vec![*self.history.last().unwrap()];
        while !found_zeros {
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
            last_values.push(*diffs.last().unwrap());
            found_zeros = diffs.iter().all(|n| n == &0);
        }
        last_values.iter().copied().reduce(|a, b| a + b).unwrap()
    }

    fn get_previous_value(&self) -> isize {
        let mut diffs = self.history.clone();
        let mut found_zeros = false;
        let mut first_values = vec![*self.history.first().unwrap()];
        while !found_zeros {
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
            first_values.push(*diffs.first().unwrap());
            found_zeros = diffs.iter().all(|n| n == &0);
        }
        first_values
            .iter()
            .rev()
            .copied()
            .reduce(|a, b| b - a)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::day09::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 1853145119);
        assert_eq!(answer_part_2(lines), 923);
    }

    const SAMPLE_REPORT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_oasis_predictions() {
        assert_eq!(answer_part_1(SAMPLE_REPORT), 114);
        assert_eq!(answer_part_2(SAMPLE_REPORT), 2);
    }
}
