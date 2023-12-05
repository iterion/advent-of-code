use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bootstrap { day: usize },
    PrintSolution { day: usize },
    Submit { day: usize, part: usize },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Bootstrap { day } => {
            println!("bootstrapping day {day}!");
            let formatted_day = format!("{day:02}");
            Command::new("aoc")
                .args([
                    "--day",
                    &day.to_string(),
                    "--overwrite",
                    "--input-file",
                    &format!("inputs/day{formatted_day}.txt"),
                    "--puzzle-file",
                    &format!("puzzles/day{formatted_day}.md"),
                    "download",
                ])
                .output()
                .expect("failed to execute process");
            let mut hb = handlebars::Handlebars::new();
            hb.register_template_string("day_tmpl", DAY_FILE)
                .expect("Invalid template");

            let day_rs_name = format!("src/day{formatted_day}.rs");
            let rs_path = Path::new(&day_rs_name);
            if !rs_path.exists() {
                let mut output_file = File::create(rs_path).expect("Could not open file");
                hb.render_to_write(
                    "day_tmpl",
                    &json!({"formatted_day": formatted_day}),
                    &mut output_file,
                )
                .expect("Could not write template");
            }
        }
        Commands::PrintSolution { day } => {
            let (part_1, part_2) = match day {
                1 => day01::run(),
                2 => day02::run(),
                3 => day03::run(),
                4 => day04::run(),
                5 => day05::run(),
                _ => panic!("no such day"),
            };

            println!("answer_part_1: {part_1:?}");
            println!("answer_part_2: {part_2:?}");
        }
        Commands::Submit { day, part } => {
            let (part_1, part_2) = match day {
                1 => day01::run(),
                2 => day02::run(),
                3 => day03::run(),
                4 => day04::run(),
                5 => day05::run(),
                _ => panic!("no such day"),
            };

            let answer = if part == &1 { part_1 } else { part_2 };
            println!("submitting answer for part {part}: {answer}");

            Command::new("aoc")
                .args([
                    "--day",
                    &day.to_string(),
                    "submit",
                    &part.to_string(),
                    &answer.to_string(),
                ])
                .output()
                .expect("failed to submit results");
        }
    }
}

const DAY_FILE: &str = r#"
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
    include_str!("../inputs/day{{formatted_day}}.txt")
}

#[cfg(test)]
mod tests {
    use crate::day{{formatted_day}}::{answer_part_1, answer_part_2, get_input_string};
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }
}
"#;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
