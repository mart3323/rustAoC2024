use nom::{IResult, Parser};

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

fn parse_wire(data: &str) -> IResult<&str, (String, bool)> {
    return nom::sequence::separated_pair(
        nom::character::complete::alphanumeric1,
        nom::bytes::complete::tag(": "),
        nom::character::complete::one_of("01")
            .map(|s| match s {
                '0' => false,
                '1' => true,
                _ => panic!("This should never happen")
            })
    )
        .map(|(id, val)| (String::from(id), val))
        .parse(data);
}

fn parse_gate(data: &str) -> IResult<&str, Gate> {
    nom::sequence::separated_pair(
        (
            nom::character::complete::alphanumeric1,
            nom::branch::alt([
                nom::bytes::complete::tag(" OR "),
                nom::bytes::complete::tag(" XOR "),
                nom::bytes::complete::tag(" AND ")
            ]),
            nom::character::complete::alphanumeric1,
        ),
        nom::bytes::complete::tag(" -> "),
        nom::character::complete::alphanumeric1
    )
        .map(|((a,typ,b), out)| {
            match typ {
                " AND " => Gate::AND(String::from(a),String::from(b),String::from(out)),
                " XOR " => Gate::XOR(String::from(a),String::from(b),String::from(out)),
                " OR " => Gate::OR(String::from(a),String::from(b),String::from(out)),
                _ => panic!("This should never happen")
            }
        })
        .parse(data)
}

pub fn parse_network(data: &str) -> IResult<&str, Network> {
    let ln = nom::character::complete::line_ending;
    nom::sequence::separated_pair(
        nom::multi::separated_list1(ln, parse_wire),
        (ln, ln),
        nom::multi::separated_list1(ln, parse_gate)
    )
        .map(|(wires, gates)| {
            
            Network {
                initial_values: wires,
                gates,
            }
        })
        .parse(data)
}