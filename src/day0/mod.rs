use std::str::FromStr;
use crate::utils::read_input_files;

pub fn solve_day0() {
    let inputs = read_input_files("day0");
    let demo = u64::from_str(&inputs.demo).unwrap();
    let full = u64::from_str(&inputs.full).unwrap();
    let expected = inputs.expected;
    let actual = solve(demo);
    assert_eq!(expected, actual);
    println!("Solution to part 1: {}", solve(full));
    let expected = inputs.expected2;
    let actual = solve2(demo);
    assert_eq!(expected, actual);
    println!("Solution to part 2: {}", solve2(full));
}
fn solve(input: u64) -> String {
    (input + input).to_string()
}
fn solve2(input: u64) -> String {
    (input * input).to_string()
}