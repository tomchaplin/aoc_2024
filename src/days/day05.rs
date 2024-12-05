use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(PartialEq, Eq)]
struct Rule(usize, usize);

struct RuleSet(Vec<Rule>);

impl RuleSet {
    // Return the index of a minimal element, starting at starting_idx
    fn minimize(&self, amongst: &[usize], starting_idx: usize) -> usize {
        let starting_value = amongst[starting_idx];
        for (other_idx, other_value) in amongst.iter().enumerate() {
            if self.0.contains(&Rule(*other_value, starting_value)) {
                return self.minimize(amongst, other_idx);
            }
        }
        starting_idx
    }
}

struct Update(Vec<usize>);

// Have to do custom sort because we only have a partial order
// Essentially just find a minimal element and put it first then recurse
fn sort(numbers: &mut [usize], rule_set: &RuleSet) {
    if numbers.len() == 0 {
        return;
    }
    let minimal_idx = rule_set.minimize(&numbers, 0);
    numbers.swap(0, minimal_idx);
    sort(&mut numbers[1..], rule_set)
}

impl Update {
    fn satisfies(&self, rule: &Rule) -> bool {
        let Some(lhs) = self.0.iter().position(|e| *e == rule.0) else {
            return true;
        };
        let Some(rhs) = self.0.iter().position(|e| *e == rule.1) else {
            return true;
        };
        lhs <= rhs
    }

    fn satisfies_set(&self, rule_set: &RuleSet) -> bool {
        rule_set.0.iter().all(|rule| self.satisfies(rule))
    }

    fn middle_page(&self) -> usize {
        self.0[self.0.len() / 2]
    }

    fn sort(&mut self, rule_set: &RuleSet) {
        sort(&mut self.0, rule_set)
    }
}

fn parse(input: &str) -> (RuleSet, Vec<Update>) {
    let rule = parser!(a:usize "|" b:usize => Rule(a, b));
    let update = parser!(pgs:repeat_sep(usize, ",") => Update(pgs));
    let p = parser!(
        section(rs:lines(rule) => RuleSet(rs))
        section(lines(update))
    );
    p.parse(input).unwrap()
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (rule_set, updates) = parse(input);
        let answer: usize = updates
            .into_iter()
            .filter(|up| up.satisfies_set(&rule_set))
            .map(|up| up.middle_page())
            .sum();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (rule_set, updates) = parse(input);
        let answer: usize = updates
            .into_iter()
            .filter(|up| !up.satisfies_set(&rule_set))
            .map(|mut up| {
                up.sort(&rule_set);
                up.middle_page()
            })
            .sum();
        Some(answer.to_string())
    }
}
