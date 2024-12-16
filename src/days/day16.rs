use std::iter;

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

use rustc_hash::{FxHashMap, FxHashSet};
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

    fn moves<'a>(&'a self, maze: &'a Maze) -> impl Iterator<Item = (ReindeerState, usize)> + 'a {
        let cw = iter::once((
            ReindeerState {
                position: self.position,
                direction: self.direction.rotate_cw(),
            },
            1000,
        ));
        let ccw = iter::once((
            ReindeerState {
                position: self.position,
                direction: self.direction.rotate_ccw(),
            },
            1000,
        ));
        let in_front = self.try_in_front(maze).map(|in_fr| (in_fr, 1));
        cw.chain(ccw).chain(in_front)
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
    unvisited: FxHashSet<ReindeerState>,
    paths: FxHashMap<ReindeerState, (FxHashSet<Vec<ReindeerState>>, usize)>,
    current_node: ReindeerState,
    current_paths: FxHashSet<Vec<ReindeerState>>,
    current_cost: usize,
    target: (usize, usize),
}

// TODO: Very slow, could speed up by ignoring any paths longer than minimal path found to endpoint
impl<'a> Dijkstra<'a> {
    fn advance(&mut self) -> Option<(ReindeerState, (FxHashSet<Vec<ReindeerState>>, usize))> {
        let moves = self.current_node.moves(self.maze);
        for (next_state, add_cost) in moves {
            let total_cost = self.current_cost + add_cost;
            let (current_paths, current_dist) = self
                .paths
                .entry(next_state)
                .or_insert((Default::default(), usize::MAX));
            if total_cost > *current_dist {
                continue;
            } else if total_cost < *current_dist {
                *current_dist = total_cost;
                *current_paths = self
                    .current_paths
                    .iter()
                    .cloned()
                    .map(|mut p| {
                        p.push(next_state);
                        p
                    })
                    .collect();
            } else {
                for p in self.current_paths.iter() {
                    let mut new_p = p.clone();
                    new_p.push(next_state);
                    current_paths.insert(new_p);
                }
            }
        }
        self.unvisited.remove(&self.current_node);
        self.unvisited
            .iter()
            .filter_map(|st| Some((*st, self.paths.get(st)?.clone())))
            .min_by_key(|(_st, (_paths, dist))| *dist)
    }

    fn run(&mut self) {
        while let Some((next_node, (next_paths, next_cost))) = self.advance() {
            if next_node.position == self.target {
                return;
            }
            self.current_node = next_node;
            self.current_cost = next_cost;
            self.current_paths = next_paths;
        }
    }

    fn init(maze: &'a Maze, start: ReindeerState, target: (usize, usize)) -> Self {
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
        let empty_path = vec![];
        let mut current_paths = FxHashSet::default();
        current_paths.insert(empty_path);
        Self {
            maze,
            unvisited,
            paths: Default::default(),
            current_node: start,
            current_cost: 0,
            current_paths,
            target,
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

// These solutions are embarassingly slow
impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (maze, start, end) = parse(input);
        let mut algo = Dijkstra::init(
            &maze,
            ReindeerState {
                position: start,
                direction: East,
            },
            end,
        );
        algo.run();
        let answer = algo
            .paths
            .into_iter()
            .filter(|(k, _v)| k.position == end)
            .map(|(_k, v)| v.1)
            .min()
            .unwrap();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (maze, start, end) = parse(input);
        let mut algo = Dijkstra::init(
            &maze,
            ReindeerState {
                position: start,
                direction: East,
            },
            end,
        );
        algo.run();
        let (_, (paths, _)) = algo
            .paths
            .into_iter()
            .filter(|(k, _v)| k.position == end)
            .min_by_key(|(_k, v)| v.1)
            .unwrap();
        let on_minimal: FxHashSet<_> = paths
            .into_iter()
            .flat_map(|p| p.into_iter().map(|s| s.position))
            .collect();
        let answer = on_minimal.len();
        Some(answer.to_string())
    }
}
