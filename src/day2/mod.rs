use nom;
use nom::{IResult, Parser};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use crate::utils::read_input_files;

// region input
type Level = i32;
type Report = Vec<Level>;
type Input = Vec<Report>;
fn parse_line(input: &str) -> IResult<&str, Report> {
    nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::digit1.map_res(i32::from_str)
    )
        .parse(input)
}
fn parse_file(input: &str) -> IResult<&str, Input> {
    nom::multi::separated_list1(nom::character::complete::line_ending, parse_line)
        .parse(input)
}
// endregion
pub fn solve_day2() {
    let inputs = read_input_files("day2");
    let demo = parse_file(&inputs.demo).unwrap().1;
    let full = parse_file(&inputs.full).unwrap().1;

    {
        let actual = solve(&demo);
        let expected = inputs.expected;
        assert_eq!(actual, expected);
        let solution = solve(&full);
        println!("Part1: {}", solution);
    }
    {
        let actual = solve2(&demo);
        let expected = inputs.expected2;
        assert_eq!(actual, expected);
        let solution = solve(&full);
        println!("Part1: {}", solution);
    }
}

pub struct Day2 {

}

impl Day2 {
    fn parse_file(&self, input: &str) -> Result<Input, Box<dyn Error>> {
        Ok(parse_file(input).unwrap().1)
    }
}

fn report_is_safe(report: &Report) -> bool {
    if report.is_empty() {
        return true;
    }

    let diffs = report.windows(2)
        .map(|pair| { pair[1]-pair[0] });
    let mut expect_signum = 0;

    for diff in diffs {
        if expect_signum == 0 {
            expect_signum = diff.signum()
        }
        if diff.signum() != expect_signum {
            return false;
        }
        let abs = diff.abs();
        if ! (1 <= abs && abs <= 3) {
            return false;
        }
    }
    return true;
}

fn solve(reports: &Input) -> String {
    reports
        .iter()
        .filter(|r| {report_is_safe(r)})
        .count()
        .to_string()
}
fn solve2(reports: &Input) -> String {
    todo!()
}