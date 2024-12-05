use crate::utils::{read_input_file, read_input_files};
use nom;
use nom::{Parser};
// needed to call map_res on parsers for some reason

pub fn solve_day3() {
    let files = read_input_files("day3");
    let demo2 = read_input_file("day3", "demo2.txt");

    assert_eq!(files.expected, solve_simple(files.demo.clone()));
    println!("Validation of part 1 passed, processing full file");
    let solution = solve_simple(files.full.clone());
    println!("Solution part 1: {}", solution);

    assert_eq!(files.expected2, solve_advanced(demo2));
    println!("Validation of part 2 passed, processing full file");
    let solution = solve_advanced(files.full);
    println!("Solution part 2: {}", solution);
}

// region parsers
#[derive(Debug, Clone)]
enum Instruction {
    Mul(u64, u64),
    Skip,
    Do,
    Dont,
}
fn parse_int(input: &str) -> nom::IResult<&str, u64> {
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

fn solve_simple(input: String) -> String {
    use nom::multi::fold_many1;

    let mut sum = fold_many1(
        parse_instruction,
        || 0,
        |sum, instruction| match instruction {
            Instruction::Mul(a, b) => sum + (a * b),
            _ => sum,
        },
    );
    sum.parse(input.as_str()).unwrap().1.to_string()
}

fn solve_advanced(input: String) -> String {
    use nom::multi::fold_many1;
    use Instruction::*;

    struct State(bool, u64);
    impl State {
        fn value(self) -> u64 {
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
    sum.parse(input.as_str()).unwrap().1.value().to_string()
}
