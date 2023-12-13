
pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    lines.len()
}

fn answer_part_2(lines: &str) -> usize {
    lines.len()
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day13.txt")
}

#[cfg(test)]
mod tests {
    use crate::day13::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }

    const SAMPLE_INPUT: &'static str = r"#
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
;

    #[test]
    fn test_point_of_incidence() {
        assert_eq!(answer_part_1(SAMPLE_INPUT), 405);
    }

}
