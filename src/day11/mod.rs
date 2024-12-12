use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use nom::character::complete::space1;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use crate::utils::read_input_file;

const DAY: &str = "day11";

type Stones = Vec<usize>;
fn parse_stones(input: &str) -> IResult<&str, Stones> {
    separated_list1(space1, digit1.map_res(usize::from_str)).parse(input)
}

fn produces(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec!(1);
    }
    let str =  stone.to_string();
    if str.len() % 2 == 0 {
        let (left, right) = str.split_at(str.len() / 2);
        let left: usize = left.parse().unwrap();
        let right: usize = right.parse().unwrap();
        return vec!(left, right);
    }
    return vec!(2024 * stone);
}

trait HashMapExt {
    fn add_insert(&mut self, index: &usize, value: usize);
}
impl HashMapExt for HashMap<usize, usize> {
    fn add_insert(&mut self, key: &usize, value: usize) {
        let prev: usize = *self.get(&key).unwrap_or(&0);
        self.insert(*key, prev + value);
    }
}

fn solve_simple(stones: Stones, times: usize) -> usize {
    // init
    let mut counts = HashMap::<usize, usize>::new();
    for stone in stones {
        counts.add_insert(&stone, 1);
    }
    // simulate
    for _ in 0..times {
        let mut newCounts = HashMap::<usize, usize>::new();
        for (&stone, &count) in &counts {
            let products = produces(stone);
            for product in products {
                newCounts.add_insert(&product, count);
            }
        }
        counts = newCounts;
    }
    // count
    counts.values().sum()
}
#[test]
fn test_part1() {
    let demo = read_input_file(DAY, "demo.txt");
    let stones = parse_stones(&demo).expect("Input should parse").1;

    let score = solve_simple(stones, 25);
    assert_eq!(score, 55312)
}

#[test]
fn test_rate_trailheads() {
    todo!()
}

pub fn part1() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let stones = parse_stones(&full).expect("Input should parse").1;
    let score = solve_simple(stones, 25);
    return score;
}
pub fn part2() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let stones = parse_stones(&full).expect("Input should parse").1;
    let score = solve_simple(stones, 75);
    return score;
}