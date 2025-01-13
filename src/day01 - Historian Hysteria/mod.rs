mod parse;
mod test;

use parse::Pair;

use std::cmp::Reverse;
use std::collections::hash_map::Iter;
use std::collections::{BinaryHeap, HashMap};
use std::iter::zip;

/// Given a list of pairs, adds together the absolute differences of each pair
///
/// For example
/// ```txt
/// inputs     absolute_diff
/// 10  19  |  4
/// 15  20  |  5
/// 15  20  |  5
/// 20  5   |  15
/// 10  3   |  7
/// 5   1   |  4
/// ```
/// Will sum up to a total of 45
fn solve_part1(input: &Vec<Pair>) -> usize {
    // Collect both lists into a heap, allowing for easy&fast access to the elements in order of smallest to largest
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    input.iter().for_each(|pair| {
        left.push(Reverse(pair.left));
        right.push(Reverse(pair.right));
    });

    // SANITY CHECK: Lists are equal (zip will silently drop extra elements)
    // Should be guaranteed by the parser, but no harm being explicit (and reminding ourselves that this is checked)
    assert_eq!(left.len(), right.len());

    zip(left.into_sorted_vec(), right.into_sorted_vec())
        .map(|(Reverse(l), Reverse(r))| l.abs_diff(r))
        .sum()
}

struct Counter(HashMap<usize, usize>);
impl Counter {
    fn new() -> Self {
        Counter(HashMap::new())
    }
    fn inc(&mut self, key: usize) {
        self.0.insert(key, self.get(key) + 1);
    }
    fn get(&self, key: usize) -> usize {
        *self.0.get(&key).unwrap_or(&0usize)
    }
    fn iter(&self) -> Iter<'_, usize, usize> {
        self.0.iter()
    }
}
/// Given a list of pairs that represents two lists, adds together all values which are present in both lists
/// multiplied by the count in each list
/// 
/// For example
/// ```txt
/// 10  19
/// 15  20
/// 15  20
/// 20  5
/// 10  3
/// 5   1
/// ```
/// | value | left       | right       | total value |
/// |-------|------------|-------------|-------------|
/// | 20    | 1x         | 2x          |  40         |
/// | 5     | 1x         | 1x          |  5          |
/// 
/// Will sum up to a total of 45
fn solve_part2(input: &Vec<Pair>) -> usize {
    let mut left = Counter::new();
    let mut right = Counter::new();
    input.iter().for_each(|pair| {
        left.inc(pair.left);
        right.inc(pair.right);
    });

    left
        .iter()
        .map(|(&value, &count_left)| {
            let count_right = right.get(value);
            return value * count_left * count_right;
        })
        .sum()
}

const FULL: &'static str = include_str!("inputs/full.txt");

fn parse_full() -> Vec<Pair> {
    parse::parse_input(FULL).expect("Full input to parse")
}
#[test]
fn full_input_parses() {
    parse_full();
}

pub fn part1() -> usize {
    solve_part1(&parse_full())
}
pub fn part2() -> usize {
    solve_part2(&parse_full())
}
