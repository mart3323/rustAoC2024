use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Eq, PartialEq, Debug)]
pub struct Pair {
    pub left: usize,
    pub right: usize,
}

peg::parser!{
  grammar input_parser() for str {
    rule number() -> usize
      = n:$(['0'..='9']+) {
            ? n.parse().or(Err("i32"))
        }

    rule pair() -> Pair
      = l:number() " "+ r:number() {
            Pair{left: l, right: r}
        }

    pub rule parse() -> Vec<Pair>
        = p:pair() ** "\n" "\n"? { p }
  }
}

/// Parses a list of whitespace-separated usize pairs
/// ```txt
/// 123    2
/// 4      31
/// 31     31
/// 3      4
/// ```
pub fn parse_input(input: &str) -> Result<Vec<Pair>, ParseError<LineCol>> {
    input_parser::parse(input)
}

#[test]
fn test_parse() {
    let s = "123    2\n\
                   3   4\n\
                   5   6\n\
    ";
    let expected = vec!(
        Pair{left: 123, right: 2},
        Pair{left: 3, right: 4},
        Pair{left: 5, right: 6},
    );
    let pairs = parse_input(s).expect("Should parse");
    assert_eq!(pairs, expected)
}