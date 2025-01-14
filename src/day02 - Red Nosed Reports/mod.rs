mod parse;
mod test;

fn report_is_safe(report: &parse::Report) -> bool {
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
    true
}
fn is_valid_pair(a: isize, b: isize) -> bool {
    let diff = (b - a).abs();
    1 <= diff && diff <= 3
}
fn are_sequential(a: isize, b: isize, c: isize) -> bool {
    (a - b).signum() == (b - c).signum()
}

fn report_is_safeish(report: &parse::Report) -> bool {
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
                |a: isize, b: isize| is_valid_pair(a, b) && (b - a).signum() == expected_signum;

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
                            if can_remove {
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
            true
        }
    }
}

type Input = Vec<parse::Report>;
fn solve(reports: &Input) -> usize {
    reports
        .iter()
        .filter(|r| report_is_safe(r))
        .count()
}
fn solve2(reports: &Input) -> usize {
    reports
        .iter()
        .filter(|r| report_is_safeish(r))
        .count()
}

const FULL: &'static str = include_str!("inputs/full.txt");

fn parse_full() -> Vec<parse::Report> {
    parse::parse_reports_file(FULL).expect("Full input to parse")
}
#[test]
fn full_input_parses() {
    parse_full();
}

pub fn part1() -> usize {
    solve(&parse_full())
}
pub fn part2() -> usize {
    solve2(&parse_full())
}