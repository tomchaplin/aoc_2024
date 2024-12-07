use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

struct Equation {
    target: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn start_work(&self) -> WorkingEquation<'_> {
        WorkingEquation {
            equation: self,
            left_evaluated: self.numbers[0],
            remaining_idx: 1,
        }
    }

    fn has_solution<const ALLOW_CONCAT: bool>(&self) -> bool {
        self.start_work().has_solution::<ALLOW_CONCAT>()
    }
}

struct WorkingEquation<'a> {
    equation: &'a Equation,
    left_evaluated: usize,
    remaining_idx: usize,
}

// From https://www.reddit.com/r/rust/comments/191l3ot/concatinate_two_numbers/
fn concat(a: usize, b: usize) -> usize {
    a as usize * 10usize.pow(b.ilog10() + 1) + b as usize
}

impl<'a> WorkingEquation<'a> {
    fn has_solution<const ALLOW_CONCAT: bool>(&self) -> bool {
        if self.remaining_idx == self.equation.numbers.len() {
            return self.left_evaluated == self.equation.target;
        }
        if self.left_evaluated > self.equation.target {
            return false;
        }
        let add_attempt = WorkingEquation {
            equation: self.equation,
            left_evaluated: self.left_evaluated + self.equation.numbers[self.remaining_idx],
            remaining_idx: self.remaining_idx + 1,
        };
        let mult_attempt = WorkingEquation {
            equation: self.equation,
            left_evaluated: self.left_evaluated * self.equation.numbers[self.remaining_idx],
            remaining_idx: self.remaining_idx + 1,
        };
        if !ALLOW_CONCAT {
            add_attempt.has_solution::<false>() || mult_attempt.has_solution::<false>()
        } else {
            let concat_attempt = WorkingEquation {
                equation: self.equation,
                left_evaluated: concat(
                    self.left_evaluated,
                    self.equation.numbers[self.remaining_idx],
                ),
                remaining_idx: self.remaining_idx + 1,
            };
            add_attempt.has_solution::<true>()
                || mult_attempt.has_solution::<true>()
                || concat_attempt.has_solution::<true>()
        }
    }
}

fn parse(input: &str) -> Vec<Equation> {
    parser!(
        lines(target:usize ": " numbers:repeat_sep(usize, " ") => Equation { target, numbers })
    )
    .parse(input)
    .unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let equations = parse(input);
        let answer: usize = equations
            .into_iter()
            .filter(|eq| eq.has_solution::<false>())
            .map(|eq| eq.target)
            .sum();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let equations = parse(input);
        let answer: usize = equations
            .into_iter()
            .filter(|eq| eq.has_solution::<true>())
            .map(|eq| eq.target)
            .sum();
        Some(answer.to_string())
    }
}
