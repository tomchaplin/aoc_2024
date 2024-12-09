use crate::ProblemSolution;
#[allow(unused_imports)]
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
use std::iter;
pub struct Solution {}

#[derive(Clone, Copy)]
enum BlockDescriptor {
    File(usize, usize), // (id, len)
    Empty(usize),       // (len)
}

impl BlockDescriptor {
    fn len(&self) -> usize {
        match self {
            BlockDescriptor::File(_, len) => *len,
            BlockDescriptor::Empty(len) => *len,
        }
    }

    fn id(&self) -> Option<usize> {
        match self {
            BlockDescriptor::File(id, _len) => Some(*id),
            BlockDescriptor::Empty(_len) => None,
        }
    }
}

fn get_checksum(blocks: &Vec<BlockDescriptor>) -> usize {
    let counts = blocks.iter().map(|bl| bl.len());
    let ids = blocks.iter().map(|bl| bl.id());
    let disk_map = ids
        .zip(counts)
        .flat_map(|(id, count)| iter::repeat(id).take(count));
    disk_map
        .enumerate()
        .filter_map(|(idx, val)| val.map(|v| v * idx))
        .sum()
}

fn execute_defrag(descriptors: &mut Vec<BlockDescriptor>, defrag: (usize, usize)) {
    let len_empty = descriptors[defrag.0].len();
    let len_file = descriptors[defrag.1].len();
    if len_empty == len_file {
        descriptors.swap(defrag.0, defrag.1);
        return;
    }
    let new_empty_0 = BlockDescriptor::Empty(len_file);
    let new_empty_1 = BlockDescriptor::Empty(len_empty - len_file);
    descriptors[defrag.0] = new_empty_0;
    descriptors.insert(defrag.0 + 1, new_empty_1);
    descriptors.swap(defrag.0, defrag.1 + 1);
}

fn find_defrag_opportunity(descriptors: &Vec<BlockDescriptor>) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = descriptors.len() - 1;
    loop {
        if j == 0 {
            // Attempted to move all file blocks
            return None;
        }
        if i >= j {
            // Moved i forward far enough, reset and try to move next file
            i = 0;
            j -= 1;
        }
        match (&descriptors[i], &descriptors[j]) {
            (BlockDescriptor::File(_, _), _) => {
                i += 1;
            }
            (_, BlockDescriptor::Empty(_)) => j -= 1,
            (BlockDescriptor::Empty(len_empty), BlockDescriptor::File(_id, len_file)) => {
                if len_empty < len_file {
                    i += 1;
                } else {
                    return Some((i, j));
                }
            }
        }
    }
}

fn produce_block_descriptors(input: &str) -> impl Iterator<Item = BlockDescriptor> + '_ {
    let counts = parse(input);
    let ids = produce_identifiers();
    ids.zip(counts).map(|(id, count)| match id {
        Some(id) => BlockDescriptor::File(id, count),
        None => BlockDescriptor::Empty(count),
    })
}

fn sort_disk_map(map: &mut Vec<Option<usize>>) {
    let mut i = 0;
    let mut j = map.len() - 1;
    loop {
        if i >= j {
            break;
        }
        if !map[i].is_none() {
            i += 1;
            continue;
        }
        if !map[j].is_some() {
            j -= 1;
            continue;
        }
        map.swap(i, j);
    }
}

fn produce_disk_map(input: &str) -> impl Iterator<Item = Option<usize>> + '_ {
    let counts = parse(input);
    let ids = produce_identifiers();
    ids.zip(counts)
        .flat_map(|(id, count)| iter::repeat(id).take(count))
}

fn produce_identifiers() -> impl Iterator<Item = Option<usize>> {
    let ids = (0..).map(|i| Some(i));
    let nones = iter::repeat(None);
    ids.interleave(nones)
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
}

impl ProblemSolution for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let mut map: Vec<_> = produce_disk_map(input).collect();
        sort_disk_map(&mut map);
        let checksum: usize = map
            .into_iter()
            .enumerate()
            .filter_map(|(idx, val)| val.map(|v| v * idx))
            .sum();
        Some(checksum.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let mut blocks: Vec<_> = produce_block_descriptors(input).collect();
        while let Some(defrag) = find_defrag_opportunity(&blocks) {
            execute_defrag(&mut blocks, defrag);
        }
        let checksum = get_checksum(&blocks);
        Some(checksum.to_string())
    }
}
