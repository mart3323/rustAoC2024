use crate::utils::{read_file, read_input_file, read_input_files};
use nom;
use nom::branch::alt;
use nom::bytes::tag;
use nom::combinator::complete;
use nom::multi::fold_many1;
use nom::{IResult, Parser};

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
fn parseInt(input: &str) -> IResult<&str, u64> {
    nom::character::complete::digit1
        .map_res(str::parse)
        .parse(input)
}
// endregion

fn solve_simple(input: String) -> String {
    let values = alt((
        (tag("mul("), parseInt, tag(","), parseInt, tag(")"))
            .map(|(_, left, _, right, _)| left * right),
        nom::bytes::complete::take(1usize).map(|_| 0),
    ));
    let sum = fold_many1(values, || 0, |a, b| a + b);
    complete(sum).parse(input.as_str()).unwrap().1.to_string()
}
fn solve_advanced(input: String) -> String {
    #[derive(Debug)]
    enum Symbol {
        Mul(u64, u64),
        Skip,
        Do,
        Dont,
    }
    let values = alt((
        tag("do()").map(|_| Symbol::Do),
        tag("don't()").map(|_| Symbol::Dont),
        (tag("mul("), parseInt, tag(","), parseInt, tag(")"))
            .map(|(_, left, _, right, _)| Symbol::Mul(left, right)),
        nom::bytes::complete::take(1usize).map(|_| Symbol::Skip),
    ));
    enum State {
        On(u64),
        Off(u64),
    }
    impl State {
        fn value(self) -> u64 {
            match self {
                State::On(v) => v,
                State::Off(v) => v,
            }
        }
    }
    let sum = fold_many1(
        values,
        || State::On(0),
        |a, b| {
            match a {
                State::Off(value) => match (b) {
                    Symbol::Do => State::On(value),
                    _ => a,
                },
                State::On(value) => match(b) {
                    Symbol::Dont => State::Off(value),
                    Symbol::Mul(a,b) => State::On(value + (a*b)),
                    _ => a
                }
            }
        },
    );
    complete(sum).parse(input.as_str()).unwrap().1.value().to_string()
}
