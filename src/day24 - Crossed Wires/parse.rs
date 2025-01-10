use std::ops::Not;
use itertools::Itertools;
use peg::error::ParseError;
use peg::str::LineCol;

pub enum Gate {
    XOR(String, String, String),
    AND(String, String, String),
    OR(String, String, String)
}
impl Gate {
    pub fn inputs(&self) -> [&str; 2] {
        match &self {
            Gate::XOR(a, b, _) => [a,b],
            Gate::AND(a, b, _) => [a,b],
            Gate::OR(a, b, _) => [a,b],
        }
    }
    pub fn output(&self) -> &str {
        match &self {
            Gate::XOR(_, _, out) => out,
            Gate::AND(_, _, out) => out,
            Gate::OR(_, _, out) => out,
        }
    }
    pub fn process(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::XOR(_, _, _) => a ^ b,
            Gate::AND(_, _, _) => a && b,
            Gate::OR(_, _, _) => a || b
        }
    }
}
pub struct Network {
    pub initial_values: Vec<(String, bool)>,
    pub gates: Vec<Gate>
}

peg::parser! {
    pub grammar parser() for str {
        rule identifier() -> String
            = c:([c if c.is_ascii() && !c.is_ascii_whitespace() ]*<3>)
            { c.into_iter().join("") }

        rule gate() -> Gate
            = a:identifier() " " "XOR" " " b:identifier() " -> " out:identifier() { Gate::XOR(a,b,out) }
            / a:identifier() " " "AND" " " b:identifier() " -> " out:identifier() { Gate::AND(a,b,out) }
            / a:identifier() " " "OR"  " " b:identifier() " -> " out:identifier() { Gate::OR(a,b,out) }

        rule wire() -> (String, bool)
            = n:identifier() ": 0" { (n, false) }
            / n:identifier() ": 1" { (n, true) }

        rule network() -> Network
            = w:(w:wire() "\n" { w })+
                "\n"
                g:(g:gate() "\n" { g })+ { Network {initial_values: w, gates: g}}
        
        pub rule parse() -> Network = network()
    }
}

pub fn parse_input(input: &str) -> Result<Network, ParseError<LineCol>> {
    parser::parse(input)
}