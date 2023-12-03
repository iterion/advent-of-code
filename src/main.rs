use clap::{Parser, Subcommand};
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
    PrintPuzzleOutput { day: usize },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Bootstrap { day } => {
            println!("bootstrapping day {day}!");
            let formatted_day = format!("{day:02}");
            let output = Command::new("aoc")
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
            println!("{DAY_FILE}");
            println!("{output:?}");
        }
        Commands::PrintPuzzleOutput { day } => match day {
            1 => day01::run().unwrap(),
            2 => day02::run().unwrap(),
            _ => panic!("no such day"),
        },
    }
}

const DAY_FILE: &str = r#"
use std::error::Error;

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let input_string = include_str!("../inputs/day.txt");
    let answer_part_1 = answer_part_1(input_string);
    let answer_part_2 = answer_part_2(input_string);
    println!("answer_part_1: {answer_part_1:?}");
    println!("answer_part_2: {answer_part_2:?}");
    Ok(())
}

fn answer_part_1(lines: &str) -> usize {
    #lines.lines().map(get_row_power).sum()
    0
}

fn answer_part_2(lines: &str) -> usize {
    #lines.lines().map(parse_game_row).sum()
    0
}

#[cfg(test)]
mod tests {
    use crate::day::{answer_part_1, answer_part_2};
    #[test]
    fn test_all_lines() {
        let lines = r%""%;

        assert_eq!(answer_part_1(lines), 0);
        assert_eq!(answer_part_2(lines), 0);
    }
}
"#;

mod day01;
mod day02;
