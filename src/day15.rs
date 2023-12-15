pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    lines.replace('\n', "").split(',').map(hash_algorithm).sum()
}

fn answer_part_2(lines: &str) -> usize {
    let ops: Vec<_> = lines
        .replace('\n', "")
        .split(',')
        .map(LabelOperation::parse)
        .collect();

    let mut storage = Storage {
        boxes: std::collections::HashMap::new(),
    };
    println!("{ops:?}");
    for op in ops {
        storage.run_operation(&op);
    }

    let mut sum = 0;
    for i in 0..256 {
        if let Some(lenses) = storage.boxes.get(&i) {
            for (j, lens) in lenses.iter().enumerate() {
                sum += lens.value * (j + 1) * (i + 1)
            }
        }
    }
    sum
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day15.txt")
}

struct Storage {
    boxes: std::collections::HashMap<usize, Vec<LabelOperation>>,
}

impl Storage {
    fn run_operation(&mut self, operation: &LabelOperation) {
        let hash = hash_algorithm(&operation.label);
        let contents = self.boxes.entry(hash).or_default();
        match operation.operation {
            Operation::Remove => contents.retain(|l| operation.label != l.label),
            Operation::Set => {
                for op in contents.iter_mut() {
                    if op.label == operation.label {
                        op.value = operation.value;
                        return;
                    }
                }
                contents.push(operation.clone());
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Remove,
    Set,
}

#[derive(Debug, Clone)]
struct LabelOperation {
    label: String,
    operation: Operation,
    value: usize,
}

impl LabelOperation {
    fn parse(op: &str) -> Self {
        if op.ends_with('-') {
            let label = &op[0..(op.len() - 1)];
            Self {
                label: label.to_owned(),
                operation: Operation::Remove,
                value: 0,
            }
        } else {
            let (label, value) = op.split_once('=').unwrap();
            Self {
                label: label.to_owned(),
                operation: Operation::Set,
                value: value.parse().unwrap(),
            }
        }
    }
}

fn hash_algorithm(input: &str) -> usize {
    let mut hash = 0usize;
    for char in input.chars() {
        hash += char as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[cfg(test)]
mod tests {
    use crate::day15::{answer_part_1, answer_part_2, get_input_string, hash_algorithm};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 517965);
        assert_eq!(answer_part_2(lines), 267372);
    }

    const SAMPLE_INITIALIZATION_SEQUENCE: &str =
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

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
        assert_eq!(answer_part_1(SAMPLE_INITIALIZATION_SEQUENCE), 1320);
        assert_eq!(answer_part_2(SAMPLE_INITIALIZATION_SEQUENCE), 145);
    }
}
