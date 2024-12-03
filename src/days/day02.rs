use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
pub struct Solution {}

trait SafetyCheck {
    fn deltas<'a>(&'a self) -> impl Iterator<Item = isize> + 'a;

    fn is_monotonic(&self) -> bool {
        self.deltas().map(|d| d.signum()).all_equal()
    }

    fn is_bounded(&self) -> bool {
        self.deltas().all(|d| d.abs() >= 1 && d.abs() <= 3)
    }

    fn is_safe(&self) -> bool {
        self.is_monotonic() && self.is_bounded()
    }
}

struct Report(Vec<usize>);

impl SafetyCheck for Report {
    fn deltas<'a>(&'a self) -> impl Iterator<Item = isize> + 'a {
        self.0
            .iter()
            .tuple_windows()
            .map(|(a, b)| *a as isize - *b as isize)
    }
}

impl Report {
    fn mask<'a>(&'a self, masked_idx: usize) -> MaskedReport<'a> {
        MaskedReport {
            report: self,
            masked_idx,
        }
    }

    fn has_safe_mask(&self) -> bool {
        (0..self.0.len()).any(|idx| self.mask(idx).is_safe())
    }
}

struct MaskedReport<'a> {
    report: &'a Report,
    masked_idx: usize,
}

impl<'r> SafetyCheck for MaskedReport<'r> {
    fn deltas<'a>(&'a self) -> impl Iterator<Item = isize> + 'a {
        self.report
            .0
            .iter()
            .enumerate()
            .filter(|(idx, _num)| *idx != self.masked_idx)
            .map(|(_idx, num)| num)
            .tuple_windows()
            .map(|(a, b)| *a as isize - *b as isize)
    }
}

fn parse_input(input: &str) -> Vec<Report> {
    let p = parser!(
        lines(
            rep:repeat_sep(usize, " ") => Report(rep)
        )
    );
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let reports = parse_input(input);
        let n_safe = reports.iter().filter(|rep| rep.is_safe()).count();
        Some(n_safe.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let reports = parse_input(input);
        let n_safe = reports
            .iter()
            .filter(|rep| rep.is_safe() || rep.has_safe_mask())
            .count();
        Some(n_safe.to_string())
    }
}
