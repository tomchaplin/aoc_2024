use std::{collections::HashSet, iter};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

type Position = (usize, usize);
type Height = usize;

struct Grid(Vec<Vec<Height>>);

impl Grid {
    fn bounds(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn map_to_pos(&self, pos: (isize, isize)) -> Option<Position> {
        let bounds = self.bounds();
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        let pos = (pos.0 as usize, pos.1 as usize);
        if pos.0 >= bounds.0 || pos.1 >= bounds.1 {
            return None;
        }
        Some(pos)
    }

    fn height(&self, pos: &Position) -> Height {
        self.0[pos.0][pos.1]
    }

    fn nbrs(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        let deltas = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let pos = (pos.0 as isize, pos.1 as isize);
        deltas
            .into_iter()
            .filter_map(move |d| self.map_to_pos((pos.0 + d.0, pos.1 + d.1)))
    }

    fn find_peaks(&self, starting: Position) -> Box<dyn Iterator<Item = Position> + '_> {
        if self.height(&starting) == 9 {
            Box::new(iter::once(starting))
        } else {
            let start_height = self.height(&starting);
            Box::new(
                self.nbrs(starting)
                    .filter(move |nbr| self.height(nbr) == start_height + 1)
                    .flat_map(|nbr| self.find_peaks(nbr)),
            )
        }
    }

    fn find_trailheads(&self) -> impl Iterator<Item = Position> + '_ {
        let (height, width) = self.bounds();
        (0..height)
            .flat_map(move |i| (0..width).map(move |j| (i, j)))
            .filter(|pos| self.height(pos) == 0)
    }

    fn score_a_trailhead(&self, trailhead: Position) -> usize {
        let peaks: HashSet<Position> = self.find_peaks(trailhead).collect();
        peaks.len()
    }

    fn score_b_trailhead(&self, trailhead: Position) -> usize {
        self.find_peaks(trailhead).map(|_| 1).sum()
    }
}

fn parse(input: &str) -> Grid {
    parser!(gr:lines(digit+) => Grid(gr)).parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let answer: usize = grid
            .find_trailheads()
            .map(|t| grid.score_a_trailhead(t))
            .sum();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let answer: usize = grid
            .find_trailheads()
            .map(|t| grid.score_b_trailhead(t))
            .sum();
        Some(answer.to_string())
    }
}
