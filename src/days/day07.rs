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
        self.start_work().yields_solution::<ALLOW_CONCAT>()
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
    fn add_attempt(&self) -> Self {
        WorkingEquation {
            equation: self.equation,
            left_evaluated: self.left_evaluated + self.equation.numbers[self.remaining_idx],
            remaining_idx: self.remaining_idx + 1,
        }
    }

    fn mult_attempt(&self) -> Self {
        WorkingEquation {
            equation: self.equation,
            left_evaluated: self.left_evaluated * self.equation.numbers[self.remaining_idx],
            remaining_idx: self.remaining_idx + 1,
        }
    }

    fn concat_attempt(&self) -> Self {
        WorkingEquation {
            equation: self.equation,
            left_evaluated: concat(
                self.left_evaluated,
                self.equation.numbers[self.remaining_idx],
            ),
            remaining_idx: self.remaining_idx + 1,
        }
    }

    fn yields_solution<const ALLOW_CONCAT: bool>(&self) -> bool {
        // Used all the numbers, did we get the right answer?
        if self.remaining_idx == self.equation.numbers.len() {
            return self.left_evaluated == self.equation.target;
        }
        // We can only increase so short-circuit failure
        if self.left_evaluated > self.equation.target {
            return false;
        }

        self.add_attempt().yields_solution::<ALLOW_CONCAT>()
            || self.mult_attempt().yields_solution::<ALLOW_CONCAT>()
            || (ALLOW_CONCAT && self.concat_attempt().yields_solution::<ALLOW_CONCAT>())
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
