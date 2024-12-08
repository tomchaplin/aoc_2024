use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

struct GridSummary {
    ants: HashMap<char, Vec<(usize, usize)>>,
    size: (usize, usize),
}

impl GridSummary {
    fn in_grid(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && (pos.0 as usize) < self.size.0 && (pos.1 as usize) < self.size.1
    }

    fn get_half_line(
        &self,
        base: (usize, usize),
        delta: (isize, isize),
    ) -> impl Iterator<Item = (isize, isize)> {
        let base = (base.0 as isize, base.1 as isize);
        (0..).map(move |i| (base.0 + i * delta.0, base.1 + i * delta.1))
    }

    // TODO: Shouldn't need a Box here
    fn antinodes_for_pair<'a, const PART_A: bool>(
        &'a self,
        a: (usize, usize),
        b: (usize, usize),
    ) -> Box<dyn Iterator<Item = (usize, usize)> + 'a> {
        if PART_A {
            let delta = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
            let n1 = (a.0 as isize - delta.0, a.1 as isize - delta.1);
            let n2 = (b.0 as isize + delta.0, b.1 as isize + delta.1);
            let n1 = iter::once(n1);
            let n2 = iter::once(n2);
            Box::new(
                n1.chain(n2)
                    .filter(|pos| self.in_grid(pos))
                    .map(|(p0, p1)| (p0 as usize, p1 as usize)),
            )
        } else {
            let delta = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
            let b_half_line = self
                .get_half_line(b, delta)
                .take_while(|pos| self.in_grid(pos));
            let a_half_line = self
                .get_half_line(a, (-delta.0, -delta.1))
                .take_while(|pos| self.in_grid(pos));
            Box::new(
                b_half_line
                    .chain(a_half_line)
                    .map(|(p0, p1)| (p0 as usize, p1 as usize)),
            )
        }
    }

    fn antinodes_for_positions<'a, const PART_A: bool>(
        &'a self,
        positions: &'a Vec<(usize, usize)>,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let n = positions.len();
        let idxs = (0..n).flat_map(move |i| ((i + 1)..n).map(move |j| (i, j)));
        idxs.flat_map(move |(i, j)| self.antinodes_for_pair::<PART_A>(positions[i], positions[j]))
    }

    fn get_antinodes<'a, const PART_A: bool>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.ants
            .values()
            .flat_map(|positions| self.antinodes_for_positions::<PART_A>(positions))
    }
}

fn parse(input: &str) -> GridSummary {
    let mut ants: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let size = (lines.len(), lines.first().unwrap().len());
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            ants.entry(c).or_default().push((i, j));
        }
    }
    GridSummary { ants, size }
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let grid_summary = parse(input);
        let antinodes: HashSet<_> = grid_summary.get_antinodes::<true>().collect();
        for pos in antinodes.iter() {
            println!("{:?}", pos);
        }
        let answer = antinodes.len();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let grid_summary = parse(input);
        let antinodes: HashSet<_> = grid_summary.get_antinodes::<false>().collect();
        for pos in antinodes.iter() {
            println!("{:?}", pos);
        }
        let answer = antinodes.len();
        Some(answer.to_string())
    }
}
