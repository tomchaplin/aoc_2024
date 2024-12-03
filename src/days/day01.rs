use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
pub struct Solution {}

fn parse_input<L: Extend<usize> + Default, R: Extend<usize> + Default>(input: &str) -> (L, R) {
    input
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut chnk| {
            (
                chnk.next().unwrap().parse::<usize>().unwrap(),
                chnk.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (mut left_nums, mut right_nums): (Vec<_>, Vec<_>) = parse_input(input);

        left_nums.sort_unstable();
        right_nums.sort_unstable();

        // There are no repeated elements in the left list which makes my answer to part b valid
        for (a, b) in left_nums.iter().tuple_windows() {
            assert_ne!(a, b);
        }

        let total_diff: usize = left_nums
            .into_iter()
            .zip(right_nums.into_iter())
            .map(|(l, r)| r.abs_diff(l))
            .sum();

        Some(total_diff.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (left_nums, right_nums): (HashSet<_>, Vec<_>) = parse_input(input);

        let similarity: usize = right_nums
            .into_iter()
            .filter(|n| left_nums.contains(n))
            .sum();

        Some(similarity.to_string())
    }
}
