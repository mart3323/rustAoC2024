use crate::utils::read_input_file;

const DAY: &str = "day0";

#[test]
fn test_part_1() {
    let demo = read_input_file(DAY, "demo.txt");
    let inputs: u64 = str::parse(&demo).expect("Expect input to parse");
    let solution = solve(inputs);
    assert_eq!(solution, 8);
}
#[test]
fn test_part_2() {
    let demo = read_input_file(DAY, "demo.txt");
    let inputs: u64 = str::parse(&demo).expect("Expect input to parse");
    let solution = solve2(inputs);
    assert_eq!(solution, 16);
}
fn solve(input: u64) -> usize {
    (input + input) as usize
}
fn solve2(input: u64) -> usize {
    (input * input) as usize
}

pub fn part1() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let inputs: u64 = str::parse(&full).expect("Expect input to parse");
    solve(inputs)
}

pub fn part2() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let inputs: u64 = str::parse(&full).expect("Expect input to parse");
    solve2(inputs)
}