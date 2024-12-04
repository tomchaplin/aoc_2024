use std::process::Command;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

// Fancied doing today in awk

impl ProblemSolution for Solution {
    fn solve_a(&self, _input: &str) -> Option<String> {
        String::from_utf8(
            Command::new("bash")
                .arg("./extra/day3a.sh")
                .output()
                .unwrap()
                .stdout,
        )
        .ok()
        .map(|s| s.trim().to_string())
    }

    fn solve_b(&self, _input: &str) -> Option<String> {
        String::from_utf8(
            Command::new("bash")
                .arg("./extra/day3b.sh")
                .output()
                .unwrap()
                .stdout,
        )
        .ok()
        .map(|s| s.trim().to_string())
    }
}
