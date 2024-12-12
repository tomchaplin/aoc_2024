use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

type Position = (usize, usize);

struct Grid {
    labels: Vec<Vec<char>>,
}

fn rotate(delta: (isize, isize)) -> (isize, isize) {
    match delta {
        (1, 0) => (0, -1),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (0, -1) => (-1, 0),
        _ => panic!(),
    }
}

impl Grid {
    fn init(labels: Vec<Vec<char>>) -> Self {
        Self { labels }
    }

    fn label(&self, pos: &Position) -> char {
        self.labels[pos.0][pos.1]
    }

    fn bounds(&self) -> (usize, usize) {
        (self.labels.len(), self.labels[0].len())
    }

    fn try_position(&self, pos: (isize, isize)) -> Option<Position> {
        let (height, width) = self.bounds();
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        let pos = (pos.0 as usize, pos.1 as usize);
        if pos.0 >= height || pos.1 >= width {
            None
        } else {
            Some(pos)
        }
    }

    fn nbrs<'a>(&'a self, pos: &'a Position) -> impl Iterator<Item = Position> + 'a {
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let pos = (pos.0 as isize, pos.1 as isize);
        deltas
            .into_iter()
            .map(move |d| (pos.0 + d.0, pos.1 + d.1))
            .filter_map(|p| self.try_position(p))
    }

    fn is_outward(&self, pos: &Position, delta: (isize, isize)) -> bool {
        let posi = (pos.0 as isize, pos.1 as isize);
        let other_pos = (posi.0 + delta.0, posi.1 + delta.1);
        match self.try_position(other_pos) {
            // Nbr on grid => check if label matches
            Some(other) => {
                if self.label(&other) != self.label(&pos) {
                    true
                } else {
                    false
                }
            }
            // Nbr off grid => perimeter
            None => true,
        }
    }

    fn outward_directions<'a>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = (isize, isize)> + 'a {
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas
            .into_iter()
            .filter(move |d| self.is_outward(&pos, *d))
    }

    fn perimiter_contribution(&self, pos: Position) -> usize {
        self.outward_directions(&pos).count()
    }

    fn corner_count(&self, pos: &Position) -> usize {
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let posi = (pos.0 as isize, pos.1 as isize);
        deltas
            .into_iter()
            .filter(|d| self.is_outward(&pos, *d))
            .filter(|d| {
                // Given an outward direction (dir)
                // Move over one position and we should still be in the same region and dir should still be an outward direction
                // Otherwise we have turned a corner
                let nbr_direction = rotate(*d);
                let nbr = (posi.0 + nbr_direction.0, posi.1 + nbr_direction.1);
                let Some(nbr) = self.try_position(nbr) else {
                    // Must be a corner direction because we left region
                    return true;
                };
                if self.label(&nbr) != self.label(pos) {
                    // Must be a corner direction because we left region
                    return true;
                }
                // Final check that direction is still outward (this checks for interior corners)
                !self.is_outward(&nbr, *d)
            })
            .count()
    }

    fn get_region(&self, pos: Position) -> HashSet<Position> {
        let mut region = HashSet::new();
        let mut to_visit = HashSet::new();
        to_visit.insert(pos);
        while let Some(next_pos) = to_visit.iter().next().cloned() {
            // Remove from to_visit
            to_visit.remove(&next_pos);
            // Look for nbrs with same label
            for nbr in self.nbrs(&next_pos) {
                if self.label(&nbr) == self.label(&next_pos) && !region.contains(&nbr) {
                    to_visit.insert(nbr);
                }
            }
            // Add to the region
            region.insert(next_pos);
        }
        region
    }

    fn get_all_regions(&self) -> Vec<HashSet<Position>> {
        let mut regions = vec![];
        let (height, width) = self.bounds();
        let mut remaining: HashSet<_> = (0..height)
            .flat_map(|i| (0..width).map(move |j| (i, j)))
            .collect();
        while let Some(next_pos) = remaining.iter().next().cloned() {
            let next_region = self.get_region(next_pos);
            for pos in next_region.iter() {
                remaining.remove(&pos);
            }
            regions.push(next_region);
        }
        regions
    }

    fn price_region_a(&self, region: &HashSet<Position>) -> usize {
        let area = region.len();
        let perim: usize = region.iter().map(|p| self.perimiter_contribution(*p)).sum();
        area * perim
    }

    fn price_region_b(&self, region: &HashSet<Position>) -> usize {
        let area = region.len();
        let corners: usize = region.iter().map(|p| self.corner_count(p)).sum();
        area * corners
    }
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let labels = parser!(lines(upper+)).parse(input).unwrap();
        let grid = Grid::init(labels);
        let regions = grid.get_all_regions();
        let price: usize = regions.into_iter().map(|r| grid.price_region_a(&r)).sum();
        Some(price.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let labels = parser!(lines(upper+)).parse(input).unwrap();
        let grid = Grid::init(labels);
        let regions = grid.get_all_regions();
        let price: usize = regions.into_iter().map(|r| grid.price_region_b(&r)).sum();
        Some(price.to_string())
    }
}
