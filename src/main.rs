use clap::{Parser, Subcommand};

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

mod day01;
mod day02;

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Bootstrap { day } => {
            println!("bootstrapping day {day}!")
            // aoc --day 1 --overwrite --input-file inputs/day01.txt --puzzle-file puzzles/day01.md download
        }
        Commands::PrintPuzzleOutput { day } => match day {
            1 => day01::run().unwrap(),
            2 => day02::run().unwrap(),
            _ => panic!("no such day"),
        },
    }
}
