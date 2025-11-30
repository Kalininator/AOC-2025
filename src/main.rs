use aoc_2025::solution::Solution;
use aoc_2025::solutions;
use chrono::prelude::*;
use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run solution for. Defaults to current day of the month
    #[arg(short, long)]
    day: Option<u32>,

    /// Filename of input
    #[arg(short, long)]
    file: Option<String>,
}

fn get_day(input: Option<u32>) -> u32 {
    match input {
        Some(day) => day,
        None => {
            let local: DateTime<Local> = Local::now();
            local.day()
        }
    }
}

fn get_file(input: Option<String>, day: u32) -> String {
    match input {
        Some(file) => file,
        None => {
            format!("inputs/{day}/input.txt")
        }
    }
}

fn main() {
    let Args { day, file } = Args::parse();

    let solutions: Vec<Box<dyn Solution>> = vec![Box::new(solutions::Day1)];

    let day = get_day(day);

    let file = get_file(file, day);
    println!("Getting solution for day {day} with file {file}",);

    let file_contents = fs::read_to_string(&file).expect("Unable to read file");
    let solution = solutions.get(day as usize - 1).unwrap_or_else(|| {
        panic!(
            "No solution for day {day}. Only have days 1-{}",
            solutions.len()
        )
    });
    println!("Day {day} part 1");
    solution.part1(&file_contents);
    println!("Day {day} part 2");
    solution.part2(&file_contents);
}
