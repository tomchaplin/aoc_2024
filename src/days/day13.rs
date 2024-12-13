use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};

use rational::Rational;

pub struct Solution {}

#[derive(PartialEq, Eq, Hash)]
struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    target: (u64, u64),
}

#[allow(dead_code)]
fn print_row(mat: &[Rational; 3]) {
    print!("{} {} {}", mat[0], mat[1], mat[2])
}

#[allow(dead_code)]
fn print_mat(mat: &[[Rational; 3]; 2]) {
    print_row(&mat[0]);
    println!("");
    print_row(&mat[1]);
    println!("");
}

fn mult_row(row: &[Rational; 3], mult: Rational) -> [Rational; 3] {
    [row[0] * mult, row[1] * mult, row[2] * mult]
}
fn divide_row(row: &[Rational; 3], div: Rational) -> [Rational; 3] {
    [row[0] / div, row[1] / div, row[2] / div]
}

fn add_row(a: &[Rational; 3], b: &[Rational; 3]) -> [Rational; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

// Return RREF
fn row_reduce(mut am: [[Rational; 3]; 2]) -> Option<[[Rational; 3]; 2]> {
    am[0] = divide_row(&am[0], am[0][0]);
    am[1] = add_row(&am[1], &mult_row(&am[0], -am[1][0]));
    if am[1][1] == Rational::new(0, 1) {
        // Inconsistent system, no solution over Q?
        return None;
    }
    am[1] = divide_row(&am[1], am[1][1]);
    am[0] = add_row(&am[0], &mult_row(&am[1], -am[0][1]));

    return Some(am);
}

impl Machine {
    fn augmented_matrix(&self) -> [[Rational; 3]; 2] {
        [
            [
                self.button_a.0.into(),
                self.button_b.0.into(),
                self.target.0.into(),
            ],
            [
                self.button_a.1.into(),
                self.button_b.1.into(),
                self.target.1.into(),
            ],
        ]
    }

    // Try and solve over Q (assume unique solution)
    // Only return solution of it happens to lie in Z^2
    fn fancy_solve(&self) -> Option<u64> {
        let am = self.augmented_matrix();
        let red = row_reduce(am).unwrap();
        if red[0][2].denominator() == 1 && red[1][2].denominator() == 1 {
            let (a_presses, _) = red[0][2].mixed_fraction();
            let (b_presses, _) = red[1][2].mixed_fraction();
            let a_presses = a_presses as u64;
            let b_presses = b_presses as u64;
            Some(3 * a_presses + b_presses)
        } else {
            None
        }
    }

    fn solve(&self) -> Option<u64> {
        (0..100)
            .take_while(|a_presses| {
                let a_endpoint = (self.button_a.0 * a_presses, self.button_a.1 * a_presses);
                a_endpoint.0 <= self.target.0 && a_endpoint.1 <= self.target.1
            })
            .filter_map(|a_presses| {
                let a_endpoint = (self.button_a.0 * a_presses, self.button_a.1 * a_presses);
                let remaining = (self.target.0 - a_endpoint.0, self.target.1 - a_endpoint.1);
                if remaining.0.rem_euclid(self.button_b.0) != 0 {
                    None
                } else {
                    let b_presses = remaining.0 / self.button_b.0;
                    let b_total_1 = self.button_b.1 * b_presses;
                    if b_total_1 != remaining.1 {
                        None
                    } else {
                        Some(3 * a_presses + b_presses)
                    }
                }
            })
            .min()
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let p = parser!(sections(
        button_a:line("Button A: X+" u64 ", Y+" u64)
        button_b:line("Button B: X+" u64 ", Y+" u64)
        target:line("Prize: X=" u64 ", Y=" u64)
        => Machine {
            button_a ,
            button_b,
            target,
        }
    ));
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let machines = parse(input);
        let total_cost: u64 = machines.into_iter().filter_map(|m| m.solve()).sum();
        Some(total_cost.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let mut machines = parse(input);
        for m in machines.iter_mut() {
            m.target.0 += 10000000000000;
            m.target.1 += 10000000000000;
        }
        let total_cost: u64 = machines.into_iter().filter_map(|m| m.fancy_solve()).sum();
        Some(total_cost.to_string())
    }
}
