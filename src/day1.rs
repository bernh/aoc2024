#![allow(unused)]

use std::fs;

use steel::steel_vm::{engine::Engine, primitives::steel_private_structp, register_fn::RegisterFn};
use steel::SteelVal;
use steel_repl;

use crate::Solution;
use regex::Regex;

fn get_input_lines() -> String {
    fs::read_to_string("inputs/day1.txt").unwrap_or("reading input file went wrong".to_string())
}

// looks like there is no sort in the steel std lib
fn list_sort(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    list
}

pub fn solve(input: &str) -> Solution {
    let mut steel = Engine::new();

    let lines: Vec<&str> = input.lines().collect();
    // let get_input_lines = || &lines;

    steel.register_fn("get-input-lines", get_input_lines);
    steel.register_fn("list-sort", list_sort);
    let solutions: SteelVal = steel
        .run(include_str!("day1.scm"))
        .expect("scheme code should be correct")
        .pop()
        .expect("there should be a return value");
    // steel_repl::run_repl(steel).unwrap();

    if let SteelVal::ListV(sol) = solutions {
        Solution {
            one: sol[0].to_string(),
            two: sol[1].to_string(),
        }
    } else {
        panic!("no solution");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {}
}
