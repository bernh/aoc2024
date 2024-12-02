// use clap::{arg, Parser};
use clap::{arg, Parser};
use std::fs;
use std::time::Instant;

// declare modules: example + one per day
mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // day
    #[arg(short, long, value_name = "DAY")]
    day: Option<usize>,

    // run all days
    #[arg(short, long)]
    all: bool,
}

pub struct Solution {
    pub one: String,
    pub two: String,
}

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        solve(day);
    } else {
        // default
        solve(1);
    }
}

fn solve(day: usize) {
    let now = Instant::now();
    let solutions: Solution = solve_day(day, &input(&format!("inputs/day{}.txt", day)));
    let elapsed = now.elapsed();
    println!(
        "Day {} solutions are: {} and {} ({:.2?} elapsed)",
        day, solutions.one, solutions.two, elapsed
    );
}

pub fn solve_day(day: usize, input: &str) -> Solution {
    match day {
        1 => day1::solve(input),
        // 2 => day2::solve(input),
        // 3 => day3::solve(input),
        // 4 => day4::solve(input),
        // 5 => day5::solve(input),
        // 6 => day6::solve(input),
        // 7 => day7::solve(input),
        // 8 => day8::solve(input),
        _ => unimplemented!("Day {} not implemented", day),
    }
}

// simple but fragile file read, just panics if something goes wrong
fn input(file: &str) -> String {
    fs::read_to_string(file).unwrap_or("reading input file went wrong".to_string())
}
