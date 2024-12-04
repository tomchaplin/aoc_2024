use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

const OFFSETS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn get(&self, pos: (isize, isize)) -> Option<char> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        self.0.get(pos.0 as usize)?.get(pos.1 as usize).copied()
    }

    fn check_for_string(&self, pos: (isize, isize), dir: (isize, isize), check: &str) -> bool {
        match (self.get(pos), check.chars().nth(0)) {
            // Nothing left to check
            (_, None) => true,
            // Ran out of bounds but got string left to check
            (None, Some(_)) => false,
            (Some(c1), Some(c2)) => {
                if c1 == c2 {
                    self.check_for_string((pos.0 + dir.0, pos.1 + dir.1), dir, &check[1..])
                } else {
                    false
                }
            }
        }
    }

    fn iter_pos(&self) -> impl Iterator<Item = (isize, isize)> {
        let dim1 = self.0.len();
        let dim2 = self.0[0].len();
        (0..dim1).flat_map(move |i| (0..dim2).map(move |j| (i as isize, j as isize)))
    }

    // offset_idx is an index into OFFSETS
    // The matching direction for an x-mas appears two later
    fn check_for_x_mas(&self, pos: (isize, isize), offset_idx: usize) -> bool {
        const MAS: &str = "MAS";

        let rotate_by_2_idx = (offset_idx + 2) % 8;
        let dir0 = OFFSETS[offset_idx];
        let dir1 = OFFSETS[rotate_by_2_idx];

        let mas0_start = (pos.0 - dir0.0, pos.1 - dir0.1);
        let mas1_start = (pos.0 - dir1.0, pos.1 - dir1.1);

        self.check_for_string(mas0_start, dir0, &MAS)
            && self.check_for_string(mas1_start, dir1, &MAS)
    }
}

fn parse_input(input: &str) -> Grid {
    let p = parser!(lines(upper+));
    Grid(p.parse(input).unwrap())
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        const XMAS: &str = "XMAS";

        let grid = parse_input(input);
        let n_matches = grid
            .iter_pos()
            .flat_map(|pos| OFFSETS.iter().copied().map(move |off| (pos, off)))
            .filter(|(pos, off)| grid.check_for_string(*pos, *off, &XMAS))
            .count();
        Some(n_matches.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let grid = parse_input(input);
        let diagonal_offset_idxs = [1, 3, 5, 7];

        let n_matches = grid
            .iter_pos()
            .flat_map(|pos| {
                diagonal_offset_idxs
                    .iter()
                    .copied()
                    .map(move |idx| (pos, idx))
            })
            .filter(|(pos, idx)| grid.check_for_x_mas(*pos, *idx))
            .count();
        Some(n_matches.to_string())
    }
}
