use crate::utils::read_input_file;
use nom;
use nom::Parser;

// region parsers
#[derive(Debug, Clone)]
enum Instruction {
    Mul(usize, usize),
    Skip,
    Do,
    Dont,
}
fn parse_int(input: &str) -> nom::IResult<&str, usize> {
    use nom::character::complete::digit1;
    digit1.map_res(str::parse).parse(input)
}

fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    use nom::branch::alt;
    use nom::bytes::tag;
    use nom::combinator::value;
    use nom::combinator::complete;
    use Instruction::*;

    let token_do = value(Do, tag("do()"));
    let token_dont = value(Dont, tag("don't()"));
    let token_mul = (tag("mul("), parse_int, tag(","), parse_int, tag(")"))
        .map(|(_, left, _, right, _)| Mul(left, right));
    let token_invalid = value(Skip, nom::bytes::complete::take(1usize));

    let token_valid = complete(alt((token_do, token_dont, token_mul)));
    alt((token_valid, token_invalid)).parse(input)
}
// endregion

fn solve_simple(input: String) -> usize {
    use nom::multi::fold_many1;

    let mut sum = fold_many1(
        parse_instruction,
        || 0,
        |sum, instruction| match instruction {
            Instruction::Mul(a, b) => sum + (a * b),
            _ => sum,
        },
    );
    sum.parse(input.as_str()).unwrap().1
}

fn solve_advanced(input: String) -> usize {
    use nom::multi::fold_many1;
    use Instruction::*;

    struct State(bool, usize);
    impl State {
        fn value(self) -> usize {
            self.1
        }
    }

    let mut sum = fold_many1(
        parse_instruction,
        || State(true, 0),
        |state, instruction| match instruction {
            Mul(a, b) => {
                if state.0 {
                    State(true, state.1 + (a * b))
                } else {
                    state
                }
            }
            Skip => state,
            Do => State(true, state.1),
            Dont => State(false, state.1),
        },
    );
    sum.parse(input.as_str()).unwrap().1.value()
}

#[test]
fn test_solve() {
    let demo = read_input_file("day3", "demo.txt");
    assert_eq!(solve_simple(demo), 161);
}
#[test]
fn test_solve_advanced() {
    let demo = read_input_file("day3", "demo2.txt");
    assert_eq!(solve_advanced(demo), 48);
}

pub fn part1() -> usize {
    let full = read_input_file("day3", "full.txt");
    solve_simple(full)
}
pub fn part2() -> usize {
    let full = read_input_file("day3", "full.txt");
    solve_advanced(full)
}