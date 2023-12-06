use clap::{Parser, Subcommand};
use serde_json::json;
use std::{fs::File, io::Read, path::Path, process::Command};

use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};

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
    TestCompletion { day: usize },
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

            // using async for just this bit
            tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap()
                .block_on(async {
                    let day_rs_name = format!("src/day{formatted_day}.rs");
                    let rs_path = Path::new(&day_rs_name);
                    if !rs_path.exists() {
                        println!("generating day {day} tests!");
                        let test_case = generate_sample_test_case(*day).await;
                        let mut output_file = File::create(rs_path).expect("Could not open file");
                        hb.render_to_write(
                            "day_tmpl",
                            &json!({"formatted_day": formatted_day, "test_case": test_case}),
                            &mut output_file,
                        )
                        .expect("Could not write template");
                    }
                });
            Command::new("cargo")
                .args(["fmt"])
                .status()
                .expect("expected fmt to work");
        }
        Commands::PrintSolution { day } => {
            let (part_1, part_2) = match day {
                1 => day01::run(),
                2 => day02::run(),
                3 => day03::run(),
                4 => day04::run(),
                5 => day05::run(),
                6 => day06::run(),
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
                6 => day06::run(),
                _ => panic!("no such day"),
            };

            let answer = if part == &1 { part_1 } else { part_2 };
            println!("submitting answer for part {part}: {answer}");

            let submission_result = Command::new("aoc")
                .args([
                    "--day",
                    &day.to_string(),
                    "submit",
                    &part.to_string(),
                    &answer.to_string(),
                ])
                .output()
                .expect("failed to submit results");
            println!("{submission_result:?}");
        }
        Commands::TestCompletion { day } => {
            // using async for just this bit
            tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap()
                .block_on(async {
                    let test_case = generate_sample_test_case(*day).await;
                    println!("got test case:");
                    println!("{test_case}");
                })
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

    {{test_case}}
}
"#;

const DAY_3_TEST_EXAMPLE: &str = r##"
    const SAMPLE_INPUT: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_short_example() {
        assert_eq!(answer_part_1(SAMPLE_INPUT), 4361);
        assert_eq!(answer_part_2(SAMPLE_INPUT), 467835);
    }
"##;

async fn generate_sample_test_case(day: usize) -> String {
    let client = Client::new();
    let puzzle_input_str = format!("puzzles/day{day:02}.md");
    let puzzle_input_path = Path::new(&puzzle_input_str);
    let day_3_puzzle = include_str!("../puzzles/day03.md");
    let mut current_puzzle = String::new();
    File::open(puzzle_input_path)
        .unwrap_or_else(|_| panic!("{puzzle_input_str} not found"))
        .read_to_string(&mut current_puzzle)
        .expect("couldn't read file to string");

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1024u16)
        .model("gpt-4-1106-preview")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a puzzle sample test creation assistant. You take puzzle inputs and return a valid test case in Rust. Return only sample rust test cases and rust constants")
                .build().unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(day_3_puzzle)
                .build().unwrap()
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(DAY_3_TEST_EXAMPLE)
                .build().unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(current_puzzle)
                .build().unwrap()
                .into(),
        ])
        .build().unwrap();

    client
        .chat()
        .create(request)
        .await
        .expect("should have gotten a successful response")
        .choices
        .first()
        .expect("didn't return any chat responses")
        .message
        .content
        .clone()
        .expect("content was unexpectedly empty")
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
