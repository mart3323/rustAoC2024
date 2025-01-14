use peg::error::ParseError;
use peg::str::LineCol;

pub type Level = isize;
pub type Report = Vec<Level>;
pub type Input = Vec<Report>;

peg::parser!{
  grammar input_parser() for str {
    rule number() -> isize
      = n:$(['0'..='9']+) {
            ? n.parse().or(Err("i32"))
        }
    rule report() -> Report
        = r:(number() ++ " ") { r }

    pub rule parse() -> Vec<Report>
        = r:(report() ** "\n") "\n"? { r }
  }
}

#[test]
fn test_parse() {
    let input = "1 2 3 4 51\n\
                8 4 2 6 5\n\
                8 8 8 8 8\n\
               ";
    let expect = vec!(
        vec!(1,2,3,4,51),
        vec!(8,4,2,6,5),
        vec!(8,8,8,8,8)
    );
    assert_eq!(input_parser::parse(input), Ok(expect));
}

pub fn parse_reports_file(data: &str) -> Result<Vec<Report>, ParseError<LineCol>> {
    input_parser::parse(data)
}