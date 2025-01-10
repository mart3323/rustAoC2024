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

pub fn parse_input(input: &str) -> Result<Vec<Pair>, ParseError<LineCol>> {
    input_parser::parse(input)
}