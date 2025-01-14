use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Do,
    Dont,
    Unknown(char),
    Mul(usize, usize)
}

peg::parser! {
  grammar input_lexer() for str {
    // rule _() = [' ' | '\n']*
    rule EOF() = ![_]
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule unknown() -> char = [_]
    rule token_unknown() -> Token =
        u:unknown() { Token::Unknown(u) }

    rule token_mul() -> Token
        = "mul(" a:number() "," b:number() ")" { Token::Mul(a, b) }

    rule token_do() -> Token = "do()" { Token::Do }
    rule token_dont() -> Token  = "don't()" { Token::Dont }

    pub rule parse_part1() -> Vec<Token>
        = a:(token_mul() / token_unknown())+ { a }


    pub rule parse_part2() -> Vec<Token>
        = a:(
            token_do() /
            token_dont() /
            token_mul() /
            a:[_] { Token::Unknown(a) }
        )+
  }
}

enum Source {
    Comment,
    Garbage,
    Mul(usize, usize)
}
peg::parser! {
    grammar input_parser() for [Token] {
        rule EOF() = ![_]
        
        rule comment() -> Source 
            = [Token::Dont] 
            c:([Token::Unknown(_) | Token::Mul(_,_)])* 
            ([Token::Do] / EOF()) 
        {
            Source::Comment
        }
        
        rule mul() -> Source
            = [Token::Mul(a,b)] {Source::Mul(a,b)}
        
        rule part1() -> Source
            = rule_mul() /
            _:[_]
    }
}

/// Parses a chunk of corrupted program memory, returning only the valid Mul() expressions
///
/// ```text
/// df9mul( 2 , 3) xmul(1,2)mul(3,45)))(abd
///                 ^^^^^^^^^^^^^^^^^
/// ```
pub fn parse_part1(input: &str) -> Result<Vec<Token>, ParseError<LineCol>> {
    input_parser::parse_part1(input)
}
/// Parses a chunk of corrupted program memory, returning only the valid Mul() expressions
/// that are not within a do()...don't() block
///
/// ```text
/// df9mul( 2 , 3)don't() xmul(1,2)mul(3,45)))(abd do()tghjklmul(1,2),mul(3,4)
///               xxxxxxx__________________________xxxx      ^^^^^^^^ ^^^^^^^^
/// ```
pub fn parse_part2(input: &str) -> Result<Vec<Token>, ParseError<LineCol>> {
    input_parser::parse_part2(input)
}

#[test]
fn test_parse_part1() {
    let s = "df9mul( 2 , 3) xmul(1,2)mul(3,45)))(abd";
    let expected = vec!(
        Token::Unknown('d'),
        Token::Unknown('f'),
        Token::Unknown('9'),
        Token::Unknown('m'),
        Token::Unknown('u'),
        Token::Unknown('l'),
        Token::Unknown('('),
        Token::Unknown(' '),
        Token::Unknown('2'),
        Token::Unknown(' '),
        Token::Unknown(','),
        Token::Unknown(' '),
        Token::Unknown('3'),
        Token::Unknown(')'),
        Token::Unknown(' '),
        Token::Unknown('x'),
        Token::Mul(1,2),
        Token::Mul(3,45),
        Token::Unknown(')'),
        Token::Unknown(')'),
        Token::Unknown('('),
        Token::Unknown('a'),
        Token::Unknown('b'),
        Token::Unknown('d'),
    );
    let pairs = parse_part1(s).expect("Should parse");
    assert_eq!(pairs, expected)
}

#[test]
fn test_parse_part2() {
    let s = "df9mul( 2 , 3)don't() xmul(1,2)mul(3,45)))(abd do()tghjklmul(1,2),mul(3,4)";
    let expected = vec!(
        Token::Unknown('d'),
        Token::Unknown('f'),
        Token::Unknown('9'),
        Token::Unknown('m'),
        Token::Unknown('u'),
        Token::Unknown('l'),
        Token::Unknown('('),
        Token::Unknown(' '),
        Token::Unknown('2'),
        Token::Unknown(' '),
        Token::Unknown(','),
        Token::Unknown(' '),
        Token::Unknown('3'),
        Token::Unknown(')'),
        Token::Dont,
        Token::Unknown(' '),
        Token::Unknown('x'),
        Token::Mul(1,2),
        Token::Mul(3,4),
        Token::Unknown(')'),
        Token::Unknown(')'),
        Token::Unknown('('),
        Token::Unknown('a'),
        Token::Unknown('b'),
        Token::Unknown('d'),
        Token::Unknown(' '),
        Token::Do,
        Token::Unknown('t'),
        Token::Unknown('g'),
        Token::Unknown('h'),
        Token::Unknown('j'),
        Token::Unknown('k'),
        Token::Unknown('l'),
        Token::Mul(1,2),
        Token::Unknown(','),
        Token::Mul(3,4),

    );
    let pairs = parse_part2(s).expect("Should parse");
    assert_eq!(pairs, expected)
}