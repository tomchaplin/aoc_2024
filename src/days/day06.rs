use std::collections::HashSet;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Occupado,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn delta(&self) -> [isize; 2] {
        match self {
            Direction::Up => [-1, 0],
            Direction::Right => [0, 1],
            Direction::Down => [1, 0],
            Direction::Left => [0, -1],
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Direction::Up as usize => Ok(Direction::Up),
            x if x == Direction::Right as usize => Ok(Direction::Right),
            x if x == Direction::Down as usize => Ok(Direction::Down),
            x if x == Direction::Left as usize => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn rotate(self) -> Self {
        (((self as usize) + 1) % 4).try_into().unwrap()
    }
}

struct Grid(Vec<Vec<State>>);

impl Grid {
    fn init_walker(self, pos: (usize, usize)) -> Walker {
        Walker {
            grid: self,
            position: pos,
            direction: Direction::Up,
        }
    }
}

struct Walker {
    grid: Grid,
    position: (usize, usize),
    direction: Direction,
}

impl Walker {
    fn get_pos_in_front(&self) -> Option<(usize, usize)> {
        let old_pos = (self.position.0 as isize, self.position.1 as isize);
        let delta = self.direction.delta();
        let new_pos = (old_pos.0 + delta[0], old_pos.1 + delta[1]);
        if new_pos.0 < 0 || new_pos.1 < 0 {
            return None;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if new_pos.0 >= self.grid.0.len() || new_pos.1 >= self.grid.0[0].len() {
            return None;
        }
        Some(new_pos)
    }

    fn get_state(&self, position: &(usize, usize)) -> State {
        self.grid.0[position.0][position.1]
    }
    fn set_state(&mut self, position: &(usize, usize), new_state: State) {
        self.grid.0[position.0][position.1] = new_state;
    }

    fn update(&mut self) -> bool {
        let Some(new_pos) = self.get_pos_in_front() else {
            return false;
        };
        match self.get_state(&new_pos) {
            State::Empty => {
                self.position = new_pos;
            }
            State::Occupado => {
                self.direction = self.direction.rotate();
            }
        }
        return true;
    }

    fn test_for_loop(&mut self) -> bool {
        let init_pos = self.position;
        let init_dir = self.direction;

        let Some(pos_in_front) = self.get_pos_in_front() else {
            // Cannot alter grid to force loop
            return false;
        };
        if matches!(self.get_state(&pos_in_front), State::Occupado) {
            // Cannot alter grid to force loop
            return false;
        }

        // Alter grid
        self.set_state(&pos_in_front, State::Occupado);

        //println!("Running a test at {:?} {:?}", self.position, self.direction);

        // Test for loop
        let mut seen_states = HashSet::new();
        seen_states.insert((self.position, self.direction));

        let found_loop = loop {
            if !self.update() {
                // Can't loop if we escape
                break false;
            }
            if !seen_states.insert((self.position, self.direction)) {
                // Seen this state before, found a loop
                break true;
            }
        };

        // Restore walker and grid
        self.position = init_pos;
        self.direction = init_dir;
        self.set_state(&pos_in_front, State::Empty);

        return found_loop;
    }
}

fn parse(input: &str) -> Walker {
    let mut grid = vec![];
    let mut pos = None;
    for (i, row) in input.lines().enumerate() {
        let mut new_line: Vec<State> = vec![];
        for (j, c) in row.chars().enumerate() {
            let next_val = match c {
                '#' => State::Occupado,
                '.' => State::Empty,
                '^' => {
                    pos = Some((i, j));
                    State::Empty
                }
                _ => panic!(),
            };
            new_line.push(next_val);
        }
        grid.push(new_line);
    }
    Grid(grid).init_walker(pos.unwrap())
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let mut walker = parse(input);
        let mut known_pos = HashSet::new();
        known_pos.insert(walker.position);
        while walker.update() {
            known_pos.insert(walker.position);
        }
        let answer = known_pos.len();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let mut walker = parse(input);
        let mut looping_inserts = HashSet::new();
        let mut seen_pos = HashSet::new();
        loop {
            seen_pos.insert(walker.position);
            if let Some(pos) = walker.get_pos_in_front() {
                // Guard to prevent inserting along the path
                // Because then we would have to rewrite history
                if !seen_pos.contains(&pos) {
                    let has_loop = walker.test_for_loop();
                    if has_loop {
                        looping_inserts.insert(pos.clone());
                    }
                }
            }
            if !walker.update() {
                break;
            }
        }
        let n_loops = looping_inserts.len();
        Some(n_loops.to_string())
    }
}
