mod parse;
mod test;

use utils::read_input_file;

const FULL: &'static str = include_str!("inputs/full.txt");

fn parse_full_part1() -> Vec<parse::Mul> {
    parse::parse_part1(FULL).expect("Full input to parse")
}
#[test]
fn full_input_parses_part1() {
    parse_full_part1();
}

fn parse_full_part2() -> Vec<parse::Mul> {
    parse::parse_part1(FULL).expect("Full input to parse")
}
#[test]
fn full_input_parses_part2() {
    parse_full_part2();
}


fn solve(input: Vec<parse::Mul>) -> usize {
    input
        .into_iter()
        .map(|mul| mul.value())
        .sum()
}


pub fn part1() -> usize {
    solve(parse_full_part1())
}
pub fn part2() -> usize {
    solve(parse_full_part2())
}