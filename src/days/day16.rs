use std::collections::{HashMap, HashSet};

use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Direction {
    fn all() -> [Self; 4] {
        [North, East, South, West]
    }

    fn to_delta(&self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }

    fn rotate_cw(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn rotate_ccw(self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ReindeerState {
    position: (usize, usize),
    direction: Direction,
}

impl ReindeerState {
    fn try_in_front(&self, maze: &Maze) -> Option<ReindeerState> {
        let delta = self.direction.to_delta();
        let in_front = (
            self.position.0 as isize + delta.0,
            self.position.1 as isize + delta.1,
        );
        let in_front = maze.try_pos(in_front)?;
        if matches!(maze.location(in_front), Location::Empty) {
            Some(ReindeerState {
                position: in_front,
                direction: self.direction,
            })
        } else {
            None
        }
    }

    fn moves(&self, maze: &Maze) -> Vec<(ReindeerState, usize)> {
        let mut states = vec![
            (
                ReindeerState {
                    position: self.position,
                    direction: self.direction.rotate_cw(),
                },
                1000,
            ),
            (
                ReindeerState {
                    position: self.position,
                    direction: self.direction.rotate_ccw(),
                },
                1000,
            ),
        ];
        if let Some(in_front) = self.try_in_front(maze) {
            states.push((in_front, 1));
        }
        states
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    Wall,
}

struct Maze {
    grid: Vec<Vec<Location>>,
}

impl Maze {
    fn location(&self, pos: (usize, usize)) -> Location {
        self.grid[pos.0][pos.1]
    }

    fn bounds(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn try_pos(&self, pos: (isize, isize)) -> Option<(usize, usize)> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        let bounds = self.bounds();
        let pos = (pos.0 as usize, pos.1 as usize);
        if pos.0 >= bounds.0 || pos.1 >= bounds.1 {
            None
        } else {
            Some(pos)
        }
    }
}

struct Dijkstra<'a> {
    maze: &'a Maze,
    unvisited: HashSet<ReindeerState>,
    distances: HashMap<ReindeerState, usize>,
    paths: HashMap<ReindeerState, (HashSet<Vec<ReindeerState>>, usize)>,
    current_node: ReindeerState,
    current_cost: usize,
}

// TODO: Very slow, could speed up by ignoring any paths longer than minimal path found to endpoint
impl<'a> Dijkstra<'a> {
    fn advance(&mut self) -> Option<(ReindeerState, usize)> {
        let moves = self.current_node.moves(self.maze);
        for (next_state, add_cost) in moves {
            let total_cost = self.current_cost + add_cost;
            let current_dist = self.distances.entry(next_state).or_insert(usize::MAX);
            *current_dist = (*current_dist).min(total_cost);
        }
        self.unvisited.remove(&self.current_node);
        self.unvisited
            .iter()
            .filter_map(|st| Some((*st, *self.distances.get(st)?)))
            .min_by_key(|(_st, dist)| *dist)
    }

    fn run(&mut self) {
        while let Some((next_node, next_cost)) = self.advance() {
            self.current_node = next_node;
            self.current_cost = next_cost;
        }
    }

    fn init(maze: &'a Maze, start: ReindeerState) -> Self {
        let (height, width) = maze.bounds();
        let unvisited = (0..height)
            .flat_map(|i| {
                (0..width)
                    .filter(move |j| matches!(maze.location((i, *j)), Location::Empty))
                    .flat_map(move |j| {
                        Direction::all().map(|d| ReindeerState {
                            position: (i, j),
                            direction: d,
                        })
                    })
            })
            .collect();
        Self {
            maze,
            unvisited,
            distances: Default::default(),
            current_node: start,
            current_cost: 0,
        }
    }
}

fn parse(input: &str) -> (Maze, (usize, usize), (usize, usize)) {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    for (i, row) in input.lines().enumerate() {
        let mut row_vec = vec![];
        for (j, cell) in row.chars().enumerate() {
            match cell {
                '.' => row_vec.push(Location::Empty),
                '#' => row_vec.push(Location::Wall),
                'S' => {
                    start = Some((i, j));
                    row_vec.push(Location::Empty)
                }
                'E' => {
                    end = Some((i, j));
                    row_vec.push(Location::Empty)
                }
                _ => panic!(),
            }
        }
        grid.push(row_vec);
    }
    (Maze { grid }, start.unwrap(), end.unwrap())
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (maze, start, end) = parse(input);
        let mut algo = Dijkstra::init(
            &maze,
            ReindeerState {
                position: start,
                direction: East,
            },
        );
        algo.run();
        let answer = algo
            .distances
            .into_iter()
            .filter(|(k, _v)| k.position == end)
            .map(|(_k, v)| v)
            .min()
            .unwrap();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        None
    }
}
