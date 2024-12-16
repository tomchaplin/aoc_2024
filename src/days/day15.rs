use std::fmt::Display;

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use colored::Colorize;
pub struct Solution {}

#[derive(Clone, Copy, Debug)]
enum State {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

fn move_to_delta(mv: char) -> (isize, isize) {
    match mv {
        'v' => (1, 0),
        '>' => (0, 1),
        '^' => (-1, 0),
        '<' => (0, -1),
        _ => panic!(),
    }
}

struct Warehouse {
    grid: Vec<Vec<State>>,
    robot_pos: (usize, usize),
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(
                    f,
                    "{}",
                    match cell {
                        State::Empty => ".".dimmed().black(),
                        State::Wall => "#".red(),
                        State::Box => "O".white(),
                        State::BoxLeft => "[".white(),
                        State::BoxRight => "]".white(),
                        State::Robot => "@".bold().green(),
                    }
                )?
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Warehouse {
    fn get_state(&self, pos: (usize, usize)) -> State {
        self.grid[pos.0][pos.1]
    }

    fn set_state(&mut self, pos: (usize, usize), state: State) {
        self.grid[pos.0][pos.1] = state;
    }

    fn try_bump(&mut self, pos: (usize, usize), delta: (isize, isize)) -> bool {
        match self.get_state(pos) {
            State::Empty => return true,
            State::Wall => return false,
            State::Box | State::Robot => {
                let target = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
                let target = (target.0 as usize, target.1 as usize);
                if self.try_bump(target, delta) {
                    self.set_state(target, self.get_state(pos));
                    self.set_state(pos, State::Empty);
                    true
                } else {
                    false
                }
            }
            _ => panic!(),
        }
    }

    fn get_directly_upstream(
        &self,
        pos: (usize, usize),
        delta: (isize, isize),
    ) -> Vec<(usize, usize)> {
        let target = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
        let target = (target.0 as usize, target.1 as usize);
        match self.get_state(pos) {
            State::Empty => vec![],
            State::Wall => panic!(),
            State::Box | State::Robot => {
                vec![target]
            }
            State::BoxLeft => {
                let right = (pos.0, pos.1 + 1);
                vec![target, right]
            }
            State::BoxRight => {
                let left = (pos.0, pos.1 - 1);
                vec![target, left]
            }
        }
    }

    // Run a breadth-first search so that furthest away appears last
    fn get_upstream(
        &self,
        pos: (usize, usize),
        delta: (isize, isize),
    ) -> Option<Vec<(usize, usize)>> {
        let mut upstream = vec![];
        let mut to_add = vec![pos];
        loop {
            if to_add.is_empty() {
                break;
            }
            let mut new_to_add = vec![];
            for p in to_add {
                if matches!(self.get_state(p), State::Wall) {
                    return None;
                }
                if matches!(self.get_state(p), State::Empty) {
                    continue;
                }
                if !upstream.contains(&p) {
                    upstream.push(p);
                    new_to_add.append(&mut self.get_directly_upstream(p, delta));
                }
            }
            to_add = new_to_add;
        }
        Some(upstream)
    }

    fn move_upstream(&mut self, mut stream: Vec<(usize, usize)>, delta: (isize, isize)) {
        while let Some(pos) = stream.pop() {
            let target = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);
            let target = (target.0 as usize, target.1 as usize);
            self.set_state(target, self.get_state(pos));
            self.set_state(pos, State::Empty);
        }
    }

    fn do_move_a(&mut self, mv: char) {
        let delta = move_to_delta(mv);
        if self.try_bump(self.robot_pos, delta) {
            let new_pos = (
                self.robot_pos.0 as isize + delta.0,
                self.robot_pos.1 as isize + delta.1,
            );
            self.robot_pos = (new_pos.0 as usize, new_pos.1 as usize);
        }
    }

    fn do_move_b(&mut self, mv: char) {
        let delta = move_to_delta(mv);
        if let Some(stream) = self.get_upstream(self.robot_pos, delta) {
            self.move_upstream(stream, delta);
            let new_pos = (
                self.robot_pos.0 as isize + delta.0,
                self.robot_pos.1 as isize + delta.1,
            );
            self.robot_pos = (new_pos.0 as usize, new_pos.1 as usize);
        }
    }

    fn sum_gps(&self) -> usize {
        let (height, width) = (self.grid.len(), self.grid[0].len());
        (0..height)
            .flat_map(|i| (0..width).map(move |j| (i, j)))
            .filter(|pos| matches!(self.get_state(*pos), State::Box | State::BoxLeft))
            .map(|pos| 100 * pos.0 + pos.1)
            .sum()
    }

    fn duplicate_grid(self) -> Self {
        let robot_pos = (self.robot_pos.0, self.robot_pos.1 * 2);
        let grid = self
            .grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|cell| match cell {
                        State::Empty => vec![State::Empty, State::Empty],
                        State::Wall => vec![State::Wall, State::Wall],
                        State::Box => vec![State::BoxLeft, State::BoxRight],
                        State::Robot => vec![State::Robot, State::Empty],
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Self { grid, robot_pos }
    }
}

fn parse(input: &str) -> (Warehouse, Vec<char>) {
    let grid_parser = parser!(
        lines({
            "#" => State::Wall,
            "." => State::Empty,
            "O" => State::Box,
            "@" => State::Robot
        }+)
    );

    let p = parser!(section(grid_parser) section(lines(any_char+)));
    let (grid, moves) = p.parse(input).unwrap();
    let moves = moves.into_iter().flatten().collect();
    let (height, width) = (grid.len(), grid[0].len());
    let robot_pos = (0..height)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .find(|pos| matches!(grid[pos.0][pos.1], State::Robot))
        .unwrap();
    let warehouse = Warehouse { grid, robot_pos };
    (warehouse, moves)
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (mut warehouse, moves) = parse(input);
        for mv in moves {
            warehouse.do_move_a(mv);
        }
        let answer = warehouse.sum_gps();
        //println!("{}", warehouse);
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (mut warehouse, moves) = parse(input);
        warehouse = warehouse.duplicate_grid();
        for mv in moves {
            warehouse.do_move_b(mv);
        }
        let answer = warehouse.sum_gps();
        //println!("{}", warehouse);
        Some(answer.to_string())
    }
}
