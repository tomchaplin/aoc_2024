use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use colored::Colorize;
use counter::Counter;
pub struct Solution {}

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn get_half(pos: isize, bound: isize) -> Option<usize> {
    let mid = bound / 2;

    match pos.cmp(&mid) {
        std::cmp::Ordering::Less => Some(0),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(1),
    }
}

impl Robot {
    fn advance(&mut self, time: usize, bounds: (isize, isize)) {
        let new_x = self.position.0 + (time as isize) * self.velocity.0;
        let mut new_x = new_x % bounds.0;
        if new_x < 0 {
            // Force positive;
            new_x += bounds.0;
        }
        let new_y = self.position.1 + (time as isize) * self.velocity.1;
        let mut new_y = new_y % bounds.1;
        // Force positive;
        if new_y < 0 {
            new_y += bounds.1;
        }
        self.position = (new_x, new_y);
    }

    fn quadrant(&self, bounds: (isize, isize)) -> Option<(usize, usize)> {
        let qx = get_half(self.position.0, bounds.0)?;
        let qy = get_half(self.position.1, bounds.1)?;
        Some((qx, qy))
    }
}

fn parse(input: &str) -> Vec<Robot> {
    parser!(lines(
        "p=" px:isize "," py:isize " v=" vx:isize "," vy:isize =>
        Robot {
            position: (px, py),
            velocity: (vx, vy)
        }
    ))
    .parse(input)
    .unwrap()
}

fn display_robots(robots: &Vec<Robot>, bounds: (isize, isize)) {
    let counts = robots
        .iter()
        .map(|r| r.position)
        .collect::<Counter<(isize, isize)>>();
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let count = counts.get(&(x, y)).copied().unwrap_or(0);
            if count == 0 {
                print!(".");
            } else {
                print!("{}", count.to_string().bold().green());
            }
        }
        println!("");
    }
}

fn pos_variance(coords: Vec<f64>) -> f64 {
    let mean = coords.iter().sum::<f64>() / coords.len() as f64;
    let num = coords
        .iter()
        .map(|xi| (xi - mean) * (xi - mean))
        .sum::<f64>();
    let denom = coords.len() - 1;
    num / (denom as f64)
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let width = 101;
        let height = 103;
        let bounds = (width, height);
        let mut robots = parse(input);

        for r in robots.iter_mut() {
            r.advance(100, bounds);
        }

        let counts = robots
            .iter()
            .filter_map(|r| r.quadrant(bounds))
            .collect::<Counter<(usize, usize)>>();

        let safety_factor: usize = counts.values().product();
        Some(safety_factor.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let mut t = 0;
        let width = 101;
        let height = 103;
        let bounds = (width, height);
        let mut robots = parse(input);
        let trigger = 400f64;

        loop {
            let x_poses = robots
                .iter()
                .map(|r| r.position.0 as f64)
                .collect::<Vec<f64>>();
            let x_var = pos_variance(x_poses);
            let y_poses = robots
                .iter()
                .map(|r| r.position.1 as f64)
                .collect::<Vec<f64>>();
            let y_var = pos_variance(y_poses);
            if x_var < trigger && y_var < trigger {
                break;
            }
            for r in robots.iter_mut() {
                r.advance(1, bounds);
            }
            t += 1;
        }

        display_robots(&robots, bounds);
        Some(t.to_string())
    }
}
