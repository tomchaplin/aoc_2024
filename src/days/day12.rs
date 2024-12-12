use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

type Position = (usize, usize);

struct Grid {
    labels: Vec<Vec<char>>,
}

struct GridPosition<'a> {
    grid: &'a Grid,
    position: Position,
}

impl<'a> GridPosition<'a> {
    fn label(&self) -> char {
        self.grid.labels[self.position.0][self.position.1]
    }

    fn nbrs(&self) -> impl Iterator<Item = GridPosition<'_>> + '_ {
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let pos = (self.position.0 as isize, self.position.1 as isize);
        deltas
            .into_iter()
            .map(move |d| (pos.0 + d.0, pos.1 + d.1))
            .filter_map(|p| self.grid.try_position(p))
    }

    fn is_outward(&self, delta: (isize, isize)) -> bool {
        let posi = (self.position.0 as isize, self.position.1 as isize);
        let other_pos = (posi.0 + delta.0, posi.1 + delta.1);
        match self.grid.try_position(other_pos) {
            // Nbr on grid => check if label matches
            Some(other) => {
                if other.label() != self.label() {
                    true
                } else {
                    false
                }
            }
            // Nbr off grid => perimeter
            None => true,
        }
    }

    fn outward_directions(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().filter(move |d| self.is_outward(*d))
    }

    fn perimiter_contribution(&self) -> usize {
        self.outward_directions().count()
    }

    fn corner_count(&self) -> usize {
        let posi = (self.position.0 as isize, self.position.1 as isize);
        self.outward_directions()
            .filter(|d| {
                // Given an outward direction (dir)
                // Move over one position and we should still be in the same region and dir should still be an outward direction
                // Otherwise we have turned a corner
                let nbr_direction = rotate(*d);
                let nbr = (posi.0 + nbr_direction.0, posi.1 + nbr_direction.1);
                let Some(nbr) = self.grid.try_position(nbr) else {
                    // Must be a corner direction because we left region
                    return true;
                };
                if nbr.label() != self.label() {
                    // Must be a corner direction because we left region
                    return true;
                }
                // Final check that direction is still outward (this checks for interior corners)
                !nbr.is_outward(*d)
            })
            .count()
    }

    fn get_region(&self) -> HashSet<Position> {
        let mut region = HashSet::new();
        let mut to_visit = HashSet::new();
        to_visit.insert(self.position);
        while let Some(next_pos) = to_visit.iter().next().cloned() {
            // Remove from to_visit
            to_visit.remove(&next_pos);
            // Look for nbrs with same label
            let next_pos = self.grid.position(next_pos);
            for nbr in next_pos.nbrs() {
                if nbr.label() == next_pos.label() && !region.contains(&nbr.position) {
                    to_visit.insert(nbr.position);
                }
            }
            // Add to the region
            region.insert(next_pos.position);
        }
        region
    }
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

    fn bounds(&self) -> (usize, usize) {
        (self.labels.len(), self.labels[0].len())
    }

    fn position(&self, position: Position) -> GridPosition<'_> {
        GridPosition {
            grid: self,
            position,
        }
    }

    fn try_position(&self, pos: (isize, isize)) -> Option<GridPosition<'_>> {
        let (height, width) = self.bounds();
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        let pos = (pos.0 as usize, pos.1 as usize);
        if pos.0 >= height || pos.1 >= width {
            None
        } else {
            Some(self.position(pos))
        }
    }

    fn get_all_regions(&self) -> Vec<HashSet<Position>> {
        let mut regions = vec![];
        let (height, width) = self.bounds();
        let mut remaining: HashSet<_> = (0..height)
            .flat_map(|i| (0..width).map(move |j| (i, j)))
            .collect();
        while let Some(next_pos) = remaining.iter().next().cloned() {
            let next_region = self.position(next_pos).get_region();
            for pos in next_region.iter() {
                remaining.remove(&pos);
            }
            regions.push(next_region);
        }
        regions
    }

    fn price_region_a(&self, region: &HashSet<Position>) -> usize {
        let area = region.len();
        let perim: usize = region
            .iter()
            .map(|p| self.position(*p).perimiter_contribution())
            .sum();
        area * perim
    }

    fn price_region_b(&self, region: &HashSet<Position>) -> usize {
        let area = region.len();
        let corners: usize = region
            .iter()
            .map(|p| self.position(*p).corner_count())
            .sum();
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
