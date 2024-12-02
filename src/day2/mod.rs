use crate::utils::read_input_files;
use nom;
use nom::{IResult, Parser};
use std::str::FromStr;

// region input
type Level = i32;
type Report = Vec<Level>;
type Input = Vec<Report>;
fn parse_line(input: &str) -> IResult<&str, Report> {
    nom::multi::separated_list1(
        nom::character::complete::space1,
        nom::character::complete::digit1.map_res(i32::from_str),
    )
    .parse(input)
}
fn parse_file(input: &str) -> IResult<&str, Input> {
    nom::multi::separated_list1(nom::character::complete::line_ending, parse_line).parse(input)
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
        let solution = solve2(&full);
        println!("Part2: {}", solution);
    }
}

fn report_is_safe(report: &Report) -> bool {
    if report.is_empty() {
        return true;
    }

    let diffs = report.windows(2).map(|pair| pair[1] - pair[0]);
    let mut expect_signum = 0;

    for diff in diffs {
        if expect_signum == 0 {
            expect_signum = diff.signum()
        }
        if diff.signum() != expect_signum {
            return false;
        }
        let abs = diff.abs();
        if !(1 <= abs && abs <= 3) {
            return false;
        }
    }
    return true;
}
fn valid_pair(a: i32, b: i32) -> bool {
    let diff = (b - a).abs();
    1 <= diff && diff <= 3
}

fn report_is_safeish(report: &Report) -> bool {
    #[derive(PartialEq, Debug)]
    enum ErrorLocation {
        Unknown,
        Between(usize, usize),
        Error,
    }
    impl ErrorLocation {
        fn narrow(&self, start: usize, end: usize) -> ErrorLocation {
            let newvalue = self._narrow(start, end);
            return newvalue;
        }
        fn _narrow(&self, start: usize, end: usize) -> ErrorLocation {
            match self {
                ErrorLocation::Unknown => ErrorLocation::Between(start, end),
                ErrorLocation::Between(from, to) => {
                    let &from = from.max(&start);
                    let &to = to.min(&(end));
                    if to > from {
                        ErrorLocation::Between(from, to)
                    } else {
                        ErrorLocation::Error
                    }
                }
                _ => ErrorLocation::Error,
            }
        }
    }

    if report.is_empty() {
        return true;
    }
    let mut error_location = ErrorLocation::Unknown;

    for (i, pair) in report.windows(3).enumerate() {
        let a = pair[0];
        let b = pair[1];
        let c = pair[2];
        if (b - a).signum() != (c - b).signum() {
            error_location = error_location.narrow(i, i + 2)
        }
        if !valid_pair(a, b) {
            error_location = error_location.narrow(i, i + 1)
        }
        if !valid_pair(b, c) {
            error_location = error_location.narrow(i + 1, i + 2)
        }
        if error_location == ErrorLocation::Error {
            return false;
        }
    }
    match error_location {
        ErrorLocation::Error => false,
        ErrorLocation::Between(start, end) => {
            for i in start..end+1 {
                if i == 0 {
                    return true;
                } else if i == report.len()-1 {
                    return true;
                } else {
                    if let Some(left) = report.get(i - 1) {
                        if let Some(right) = report.get(i + 1) {
                            if valid_pair(*left, *right) {
                                let mut subreport = report.clone();
                                subreport.remove(i);
                                return report_is_safe(&subreport);
                            }
                        }
                    }
                }
            }
            false
        }
        ErrorLocation::Unknown => true,
    }
}
fn report_is_safeish_dumb_version(report: &Report) -> bool {
    for i in 0..report.len() {
        let mut candidate = report.clone();
        candidate.remove(i);
        if report_is_safe(&candidate) {
            return true;
        }
    }
    return false;
}

fn solve(reports: &Input) -> String {
    reports
        .iter()
        .filter(|r| report_is_safe(r))
        .count()
        .to_string()
}
fn solve2(reports: &Input) -> String {
    reports
        .iter()
        .filter(|r| report_is_safeish_dumb_version(r))
        .count()
        .to_string()
}
