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
        let solution = solve2_naive(&full);
        let solution2 = solve2(&full);
        assert_eq!(solution, solution2);
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
fn is_valid_pair(a: i32, b: i32) -> bool {
    let diff = (b - a).abs();
    1 <= diff && diff <= 3
}
fn are_sequential(a: i32, b: i32, c: i32) -> bool {
    (a - b).signum() == (b - c).signum()
}

fn report_is_safeish(report: &Report) -> bool {
    match report.len() {
        0 => true,
        1 => true,
        2 => true,
        3 => {
            is_valid_pair(report[0], report[1])
                || is_valid_pair(report[0], report[2])
                || is_valid_pair(report[1], report[2])
        }
        _ => {
            let a = report[0];
            let b = report[1];
            let c = report[2];
            let d = report[3];
            let expected_signum = ((b - a).signum() + (c - b).signum() + (d - c).signum()).signum();

            if expected_signum == 0 {
                return false;
            }
            let is_valid_pair_with_dir =
                |a: i32, b: i32| is_valid_pair(a, b) && (b - a).signum() == expected_signum;

            #[derive(Debug)]
            enum Error {
                Either(usize),
                Must(usize),
                Unknown,
            }
            let mut error_found_at = Error::Unknown;
            for i in 0..report.len()-1 {
                if !is_valid_pair_with_dir(report[i], report[i + 1]) {
                    match error_found_at {
                        Error::Must(index) if index == i => {
                            /* OK */
                        }
                        Error::Either(prev_index) if prev_index == i - 1 => {
                            let can_remove =
                                is_valid_pair_with_dir(report[i - 1], report[i + 1]);
                            if (can_remove) {
                                error_found_at = Error::Must(i)
                            } else {
                                return false;
                            }
                        }
                        Error::Unknown => {
                            let can_remove_2nd = i == report.len() - 2
                                || is_valid_pair_with_dir(report[i], report[i + 2]);
                            let can_remove_1st =
                                i == 0 || is_valid_pair_with_dir(report[i - 1], report[i + 1]);
                            if can_remove_2nd && can_remove_1st {
                                error_found_at = Error::Either(i)
                            } else if can_remove_1st {
                                error_found_at = Error::Must(i)
                            } else if can_remove_2nd {
                                error_found_at = Error::Must(i + 1)
                            } else {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    }
                }
            }
            return true;
        }
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
fn solve2_naive(reports: &Input) -> String {
    for report in reports {
        let a = report_is_safeish_dumb_version(report);
        let b = report_is_safeish(report);
        assert_eq!(a, b);
    }
    reports
        .iter()
        .filter(|r| report_is_safeish_dumb_version(r))
        .count()
        .to_string()
}
fn solve2(reports: &Input) -> String {
    reports
        .iter()
        .filter(|r| report_is_safeish(r))
        .count()
        .to_string()
}
