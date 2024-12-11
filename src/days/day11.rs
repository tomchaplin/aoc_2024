use std::collections::HashMap;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

fn n_digits(num: &usize) -> u32 {
    num.ilog10() + 1
}

fn try_split(num: usize) -> Option<(usize, usize)> {
    let n_digits = n_digits(&num);
    if n_digits % 2 == 0 {
        let power_of_10 = 10usize.pow(n_digits / 2);
        let first_half = num / power_of_10;
        let second_half = num % power_of_10;
        Some((first_half, second_half))
    } else {
        None
    }
}

fn single_blink(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if let Some((first_half, second_half)) = try_split(stone) {
        vec![first_half, second_half]
    } else {
        vec![stone * 2024]
    }
}

fn blink(stone: usize, count: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if count == 0 {
        return 1;
    }

    if cache.contains_key(&(stone, count)) {
        return cache.get(&(stone, count)).unwrap().clone();
    }

    let answer = single_blink(stone)
        .into_iter()
        .map(|st| blink(st, count - 1, cache))
        .sum();

    cache.insert((stone, count), answer);

    answer
}

fn parse(input: &str) -> Vec<usize> {
    parser!(line(repeat_sep(usize, " "))).parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let stones = parse(input);
        let mut cache = HashMap::default();
        let answer: usize = stones.into_iter().map(|st| blink(st, 25, &mut cache)).sum();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let stones = parse(input);
        let mut cache = HashMap::default();
        let answer: usize = stones.into_iter().map(|st| blink(st, 75, &mut cache)).sum();
        Some(answer.to_string())
    }
}
