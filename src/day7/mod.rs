use std::collections::HashSet;
use crate::utils::read_input_file;
use std::fmt::{Debug, Display, Formatter};
use nom::{IResult, Parser};

type Input = Vec<EquationLine>;
#[derive(Debug)]
struct EquationLine {
    expected: usize,
    equation: Equation
}
impl From<(usize, Equation)> for EquationLine {
    fn from((expected, equation): (usize, Equation)) -> Self {
        Self { expected, equation }
    }
}
impl Display for EquationLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line(exp: {}, eq:{})", self.expected, self.equation)
    }
}
#[derive(Debug)]
struct Equation {
    values: Vec<usize>,
}
impl From<Vec<usize>> for Equation {
    fn from(values: Vec<usize>) -> Self {
        Equation { values }
    }
}
impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Eq({:?})", self.values)
    }
}

fn solve(input: &Input, operators: &[Op]) -> usize {
    let mut total = 0;
    for line in input {
        let mut potential_values = HashSet::from([0]);
        for value in line.equation.values.iter() {
            potential_values = potential_values.iter().flat_map(|v|
                operators.iter().map(|op| op(v, value))
            ).filter(|v| *v <= line.expected).collect();
        }
        if potential_values.contains(&line.expected) {
            total += line.expected;
        }
    }
    return total;
}

// region input

fn parse_input(data: &str) -> Result<Vec<EquationLine>, ()> {
    use nom::bytes::tag;
    use nom::character::complete::digit1;
    use nom::character::complete::space1;
    use nom::character::complete::newline;
    use nom::sequence::separated_pair;
    use nom::multi::separated_list1;
    enum ParseFileError {
        ParseError,
    }

    fn number(s: &str) -> IResult<&str, usize> {
        digit1.map_res(str::parse).parse(s)
    }
    fn equation(s: &str) -> IResult<&str, Equation> {
        separated_list1(space1, number).map(Equation::from).parse(s)
    }
    fn equation_line(s: &str) -> IResult<&str, EquationLine> {
        separated_pair(number, tag(": "), equation).map(EquationLine::from).parse(s)
    }

    let (remainder, parsed) = separated_list1(newline, equation_line).parse(data).expect("File should parse correctly");
    match remainder.len() {
        0 => Ok(parsed),
        _ => Err(()),
    }
}
// endregion


type Op = fn(a: &usize, b: &usize) -> usize;
const OP_ADD: Op = |a:&usize, b: &usize| a+b;
const OP_MUL: Op = |a:&usize, b: &usize| a*b;
const OP_CON: Op = |a:&usize, b: &usize| (a.to_string() + &*b.to_string()).parse::<usize>().unwrap();

const SIMPLE_OP: [Op; 2] = [OP_ADD, OP_MUL];
const ADV_OP: [Op; 3] = [OP_ADD, OP_MUL, OP_CON];

#[test]
fn test_solve_simple() {
    let demo = read_input_file("day7", "demo.txt");
    let state = parse_input(&demo).expect("Demo file should parse");
    assert_eq!(solve(&state, &SIMPLE_OP), 3749);
}
#[test]
fn test_solve_advanced() {
    let demo = read_input_file("day7", "demo.txt");
    let state = parse_input(&demo).expect("Demo file should parse");
    assert_eq!(solve(&state, &ADV_OP), 11387);
}

pub fn part1() -> usize {
    let full = read_input_file("day7", "full.txt");
    let state = parse_input(&full).expect("Demo file should parse");
    solve(&state, &SIMPLE_OP)
}
pub fn part2() -> usize {
    let full = read_input_file("day7", "full.txt");
    let state = parse_input(&full).expect("Demo file should parse");
    solve(&state, &ADV_OP)
}