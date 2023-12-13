use itertools::Itertools;
use rayon::prelude::*;
pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    parse_all_condition_reports(lines)
        .par_iter()
        .map(|r| r.valid_condition_count(&r.condition, &r.criteria))
        .sum()
}

fn answer_part_2(lines: &str) -> usize {
    let reports = parse_all_condition_reports(lines);
    let iter = reports.iter();
    let mut total = 0;
    let mut n = 1;
    for report in iter {
        let count = report.v3_valid_condition_count();
        total += count;
        println!("finished #{} - {}", n, total);
        n += 1;
    }

    total
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day12.txt")
}

fn parse_all_condition_reports(lines: &str) -> Vec<ConditionReport> {
    lines.lines().map(|l| ConditionReport::parse(l)).collect()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SpringCondition {
    Healthy,
    Damaged,
    Unknown,
}

impl SpringCondition {
    fn parse(item: char) -> Self {
        match item {
            '.' => SpringCondition::Healthy,
            '#' => SpringCondition::Damaged,
            '?' => SpringCondition::Unknown,
            _ => panic!("Unknown spring condition"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Condition {
    springs: Vec<SpringCondition>,
}

impl Condition {
    fn parse(line: &str) -> Self {
        let springs = line.chars().map(|c| SpringCondition::parse(c)).collect();
        Self { springs }
    }

    fn repeated_version(&self) -> Self {
        let mut springs = self.springs.clone();
        springs.push(SpringCondition::Unknown);
        springs.append(&mut self.springs.clone());
        Self { springs }
    }
}

struct ConditionReport {
    condition: Condition,
    criteria: Vec<usize>,
}

impl ConditionReport {
    fn parse(line: &str) -> Self {
        let (condition_string, criteria_string) = line.split_once(' ').unwrap();
        let condition = Condition::parse(condition_string);
        let criteria = criteria_string
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            condition,
            criteria,
        }
    }

    fn check_condition_fits_criteria(&self, condition: &Condition, criteria: &Vec<usize>) -> bool {
        let mut last_was_damaged = false;
        let mut running_count = 0;
        let mut found_criteria = vec![];
        for c in &condition.springs {
            if c == &SpringCondition::Damaged {
                last_was_damaged = true;
                running_count += 1;
            } else {
                if last_was_damaged {
                    found_criteria.push(running_count);
                    running_count = 0;
                }
                last_was_damaged = false;
            }
        }
        if last_was_damaged {
            found_criteria.push(running_count);
        }

        &found_criteria == criteria
    }

    fn valid_condition_count(&self, base_condition: &Condition, criteria: &Vec<usize>) -> usize {
        let unknown_indices: Vec<_> = base_condition
            .springs
            .iter()
            .enumerate()
            .filter(|(_, c)| *c == &SpringCondition::Unknown)
            .map(|(i, _)| i)
            .collect();
        let num_unknown_indices = unknown_indices.len();
        //println!("unknown indices: {num_unknown_indices}");

        let mut possible_condition_count = 0;
        let possible_unknown_states = itertools::repeat_n(
            [SpringCondition::Healthy, SpringCondition::Damaged].iter(),
            num_unknown_indices,
        )
        .multi_cartesian_product();
        for possible in possible_unknown_states {
            //println!("{possible:?}");
            let mut condition = base_condition.clone();
            for (p, i) in possible.iter().zip(unknown_indices.clone()) {
                condition.springs[i] = **p;
            }
            //println!("{condition:?}");
            if self.check_condition_fits_criteria(&condition, criteria) {
                possible_condition_count += 1;
            }
        }

        possible_condition_count
    }

    fn valid_conditions(
        &self,
        base_condition: &Condition,
        criteria: &Vec<usize>,
    ) -> Vec<Condition> {
        let unknown_indices: Vec<_> = base_condition
            .springs
            .iter()
            .enumerate()
            .filter(|(_, c)| *c == &SpringCondition::Unknown)
            .map(|(i, _)| i)
            .collect();
        let num_unknown_indices = unknown_indices.len();
        //println!("unknown indices: {num_unknown_indices}");

        let mut possible_conditions = vec![];
        let possible_unknown_states = itertools::repeat_n(
            [SpringCondition::Healthy, SpringCondition::Damaged].iter(),
            num_unknown_indices,
        )
        .multi_cartesian_product();
        for possible in possible_unknown_states {
            //println!("{possible:?}");
            let mut condition = base_condition.clone();
            for (p, i) in possible.iter().zip(unknown_indices.clone()) {
                condition.springs[i] = **p;
            }
            //println!("{condition:?}");
            if self.check_condition_fits_criteria(&condition, criteria) {
                possible_conditions.push(condition.clone());
            }
        }

        possible_conditions
    }

    fn v2_valid_condition_count(&self) -> usize {
        let base_count = self.valid_condition_count(&self.condition, &self.criteria);
        let mut repeat_criteria = self.criteria.clone();
        repeat_criteria.append(&mut self.criteria.clone());
        let expanded_count =
            self.valid_condition_count(&self.condition.repeated_version(), &repeat_criteria);
        base_count * (expanded_count / base_count).pow(4)
    }

    fn v3_valid_condition_count(&self) -> usize {
        let base_count = self.valid_condition_count(&self.condition, &self.criteria);
        let _last_spring = self.condition.springs.last().unwrap().clone();
        let mut end_with_unknown = self.condition.springs.clone();
        end_with_unknown.push(SpringCondition::Unknown);
        let end = Condition {
            springs: end_with_unknown,
        };
        let end_count = self.valid_conditions(&end, &self.criteria).len();

        let mut original_springs = self.condition.springs.clone();
        let mut start_with_unknown = vec![SpringCondition::Unknown];
        start_with_unknown.append(&mut original_springs);
        let start = Condition {
            springs: start_with_unknown,
        };
        let conditions = self.valid_conditions(&start, &self.criteria);
        let start_count = conditions.len();

        let expanded_count = std::cmp::max(start_count, end_count);

        base_count * (expanded_count).pow(4)
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::{
        answer_part_1, answer_part_2, get_input_string, parse_all_condition_reports, Condition,
        ConditionReport,
    };
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 7195);
        assert_eq!(answer_part_2(lines), 3497545717240); // too low!
    }

    const SAMPLE_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_spring_arrangements() {
        assert_eq!(answer_part_1(SAMPLE_INPUT), 21);
        assert_eq!(answer_part_2(SAMPLE_INPUT), 525152);
    }

    #[test]
    fn test_check_condition_report_match() {
        let condition_reports = parse_all_condition_reports(SAMPLE_INPUT);
        let first_report = condition_reports.first().unwrap();
        let check_condition = Condition::parse("#.#.###");
        assert!(
            first_report.check_condition_fits_criteria(&check_condition, &first_report.criteria)
        );
    }

    #[test]
    fn test_expanded_combo_count() {
        let condition_reports = parse_all_condition_reports(SAMPLE_INPUT);
        let mut report_iter = condition_reports.iter();
        let report = report_iter.next().unwrap();
        assert_eq!(
            report.valid_condition_count(&report.condition, &report.criteria),
            1
        );
        assert_eq!(report.v2_valid_condition_count(), 1);
        //assert_eq!(report.v3_valid_condition_count(), 1);

        let report = report_iter.next().unwrap();
        assert_eq!(
            report.valid_condition_count(&report.condition, &report.criteria),
            4
        );
        assert_eq!(report.v2_valid_condition_count(), 16384);
        assert_eq!(report.v3_valid_condition_count(), 16384);

        let report = report_iter.next().unwrap();
        assert_eq!(
            report.valid_condition_count(&report.condition, &report.criteria),
            1
        );
        assert_eq!(report.v2_valid_condition_count(), 1);
        assert_eq!(report.v3_valid_condition_count(), 1);

        let report = report_iter.next().unwrap();
        assert_eq!(
            report.valid_condition_count(&report.condition, &report.criteria),
            1
        );
        assert_eq!(report.v2_valid_condition_count(), 16);
        assert_eq!(report.v3_valid_condition_count(), 16);

        let report = report_iter.next().unwrap();
        assert_eq!(
            report.valid_condition_count(&report.condition, &report.criteria),
            4
        );
        assert_eq!(report.v2_valid_condition_count(), 2500);
        assert_eq!(report.v3_valid_condition_count(), 2500);

        let report = report_iter.next().unwrap();
        assert_eq!(
            report.valid_condition_count(&report.condition, &report.criteria),
            10
        );
        assert_eq!(report.v2_valid_condition_count(), 506250);
        assert_eq!(report.v3_valid_condition_count(), 506250);
    }

    #[test]
    fn test_shifted_case() {
        //trying to discover the fucking relation between these patterns
        let cr = ConditionReport::parse(".??..??...?##. 1,1,3");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 4);
        let cr = ConditionReport::parse("???.### 1,1,3");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 1);
        let cr = ConditionReport::parse("???.###????.### 1,1,3,1,1,3");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 1);
        let cr = ConditionReport::parse("###????.### 3,1,1,3");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 1);
        let cr = ConditionReport::parse("?###??????????###???????? 3,2,1,3,2,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 150);
        let cr = ConditionReport::parse("?###????????? 3,2,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 15);
        let cr = ConditionReport::parse("??###???????? 3,2,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 10);
        let cr = ConditionReport::parse(".??..??...?##.? 1,1,3");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 4);
        let cr = ConditionReport::parse("?.??..??...?##. 1,1,3");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 8);
        let cr = ConditionReport::parse("?????????.??????? 4,1,1,1,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 166);
        let cr = ConditionReport::parse("?????????.???????? 4,1,1,1,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 314);
        let cr = ConditionReport::parse("??????????.??????? 4,1,1,1,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 314);
        let cr = ConditionReport::parse("?????????.????????????????.??????? 4,1,1,1,1,4,1,1,1,1");
        assert_eq!(cr.valid_condition_count(&cr.condition, &cr.criteria), 166);
    }
}
