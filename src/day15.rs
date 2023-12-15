
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
    include_str!("../inputs/day15.txt")
}

#[cfg(test)]
mod tests {
    use crate::day15::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }

    const SAMPLE_INITIALIZATION_SEQUENCE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_hash_on_steps() {
    let step_results = [
        ("rn=1", 30),
        ("cm-", 253),
        ("qp=3", 97),
        ("cm=2", 47),
        ("qp-", 14),
        ("pc=4", 180),
        ("ot=9", 9),
        ("ab=5", 197),
        ("pc-", 48),
        ("pc=6", 214),
        ("ot=7", 231),
    ];

    for (step, expected_result) in &step_results {
        assert_eq!(hash_algorithm(step), *expected_result);
    }
}

#[test]
fn test_hash_sum() {
    let sum: u16 = SAMPLE_INITIALIZATION_SEQUENCE
        .split(',')
        .map(hash_algorithm)
        .map(|x| x as u16)
        .sum();
    assert_eq!(sum, 1320);
}
}
