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

fn solve(input: &Input, operators: &Vec<fn(a: &usize, b: &usize) -> usize>) -> usize {
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

pub fn solve_day7() {

    let demo_txt = read_input_file("day7", "demo.txt");
    let full_txt = read_input_file("day7", "full.txt");
    let input_demo = parse_input(&demo_txt).expect("File should parse correctly");
    let input_full = parse_input(&full_txt).expect("File should parse correctly");

    let op_add = |a:&usize, b: &usize| a+b;
    let op_mul = |a:&usize, b: &usize| a*b;
    let op_con = |a:&usize, b: &usize| (a.to_string() + &*b.to_string()).parse::<usize>().unwrap();

    let simple_op = vec!(op_add, op_mul);
    let adv_op = vec!(op_add, op_mul, op_con);
    assert_eq!(solve(&input_demo, &simple_op), 3749usize);
    println!("Demo 1 passed");
    println!("full solution is {}", solve(&input_full, &simple_op));
    println!("Demo 2 passed");
    assert_eq!(solve(&input_demo, &adv_op), 11387usize);
    println!("full solution is {}", solve(&input_full, &adv_op));
    
    // println!("{}", demo_state);
    // solve_advanced(&demo_state);
    // assert_eq!(solve_advanced(&demo_state), 6usize);
    // println!("Demo 2 passed");
    // println!("full solution is {}", solve_advanced(&full_state));
}
