use std::fmt::{Display, Formatter};
use nom::{IResult, Parser};
use crate::utils::read_input_file;

const DAY: &str = "day19";

#[derive(Eq, PartialEq)]
enum Color {
    White,
    Red,
    Green,
    Blue,
    Black,
}
impl TryFrom<char> for Color {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Color::White),
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            'u' => Ok(Color::Blue),
            'b' => Ok(Color::Black),
            _ => Err(())
        }
    }
}
impl From<&Color> for char {
    fn from(value: &Color) -> Self {
        match value {
            Color::White => 'w',
            Color::Red => 'r',
            Color::Green => 'g',
            Color::Blue => 'u',
            Color::Black => 'b',
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

type Towels = Vec<Vec<Color>>;
type Pattern = Vec<Color>;
struct Input {
    towels: Towels,
    patterns: Vec<Pattern>
}
fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    let col = nom::character::complete::one_of("wrgub").map_res(Color::try_from);
    nom::multi::many1(col).parse(input)
}
fn parse_towels(input: &str) -> IResult<&str, Towels> {
    let tag = nom::bytes::complete::tag;
    nom::multi::separated_list1(tag(", "), parse_pattern).parse(input)
}
fn parse_file(input: &str) -> IResult<&str, Input> {
    let nl = nom::character::complete::newline;
    let patterns = nom::multi::separated_list1(nl, parse_pattern);
    nom::sequence::separated_pair(parse_towels,(nl, nl), patterns)
        .map(|(towels, patterns)| Input {towels, patterns})
        .parse(input)
}

fn count_possible_patterns(input: &Input) -> usize {
    let mut total = 0;
    for pattern in &input.patterns {

        let mut reachable: Vec<bool> = vec![false; pattern.len()+1];
        reachable[0] = true;
        for i in 0..pattern.len() {
            if reachable[i] {
                for towel in &input.towels {
                    if pattern[i..].starts_with(towel) {
                        reachable[i + towel.len()] = true;
                    }
                }
            }
        }
        if reachable[pattern.len()] {
            total += 1;
        }
    };
    return total;
}
fn count_pattern_solutions(input: &Input) -> usize {
    let mut total = 0;
    for pattern in &input.patterns {

        let mut reachable: Vec<usize> = vec![0; pattern.len()+1];
        reachable[0] += 1;
        for i in 0..pattern.len() {
            let count = reachable[i];
            if 0 < count {
                for towel in &input.towels {
                    if pattern[i..].starts_with(towel) {
                        reachable[i + towel.len()] += count;
                    }
                }
            }
        }
        total += reachable[pattern.len()];
    };
    return total;
}

#[test]
fn test_part1() {
    let input = parse_file(&read_input_file(DAY, "demo.txt")).expect("File to parse correctly").1;
    assert_eq!(count_possible_patterns(&input), 6);
}

pub fn part1() -> usize {
    let input = parse_file(&read_input_file(DAY, "full.txt")).expect("File to parse correctly").1;
    return count_possible_patterns(&input);
}

#[test]
fn test_part2() {
    let input = parse_file(&read_input_file(DAY, "demo.txt")).expect("File to parse correctly").1;
    assert_eq!(count_pattern_solutions(&input), 16);
}

pub fn part2() -> usize {
    let input = parse_file(&read_input_file(DAY, "full.txt")).expect("File to parse correctly").1;
    return count_pattern_solutions(&input);
}