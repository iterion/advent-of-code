pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    Races::parse(lines).ways_to_win_product()
}

fn answer_part_2(lines: &str) -> usize {
    lines.len()
}

struct Races {
    times_and_distances: Vec<(usize, usize)>,
}

impl Races {
    fn parse(lines: &str) -> Self {
        let mut line_iter = lines.lines();
        let times: Vec<usize> = line_iter
            .next()
            .expect("should have first line")
            .split_once(':')
            .expect("should have times")
            .1
            .split_whitespace()
            .map(|s| s.parse().expect("expected parseable number"))
            .collect();
        let distances: Vec<usize> = line_iter
            .next()
            .expect("should have first line")
            .split_once(':')
            .expect("should have distances")
            .1
            .split_whitespace()
            .map(|s| s.parse().expect("expected parseable number"))
            .collect();
        Races {
            times_and_distances: times.iter().copied().zip(distances).collect(),
        }
    }

    fn calculate_all_ways_to_win(&self) -> Vec<usize> {
        self.times_and_distances
            .iter()
            .map(|&(time, distance)| calculate_ways_to_beat_record(time, distance))
            .collect()
    }

    fn ways_to_win_product(&self) -> usize {
        self.calculate_all_ways_to_win().iter().product()
    }
}

fn calculate_ways_to_beat_record(time: usize, distance: usize) -> usize {
    (1..time)
        .filter(|&charge_time| charge_time * (time - charge_time) > distance)
        .count()
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day06.txt")
}

#[cfg(test)]
mod tests {
    use crate::day06::{answer_part_1, answer_part_2, get_input_string, Races};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 131376);
        assert_eq!(answer_part_2(lines), 74);
    }

    const RAW_RACE_TIME_AND_DISTANCES: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_input_parse() {
        let races = Races::parse(RAW_RACE_TIME_AND_DISTANCES);
        assert_eq!(races.times_and_distances, vec![(7, 9), (15, 40), (30, 200)]);
    }

    #[test]
    fn test_ways_to_win_races() {
        let races = Races::parse(RAW_RACE_TIME_AND_DISTANCES);
        let ways_to_win = races.calculate_all_ways_to_win();

        assert_eq!(ways_to_win, vec![4, 8, 9]);

        assert_eq!(races.ways_to_win_product(), 288);
    }
}
