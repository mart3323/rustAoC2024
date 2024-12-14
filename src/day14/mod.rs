use std::fmt::{Display, Formatter};
use crate::utils::read_input_file;
use nom::{Complete, IResult, Parser};

const DAY: &str = "day14";

struct Vector2D {
    x: i64,
    y: i64,
}
impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
struct Robot {
    position: Vector2D,
    velocity: Vector2D,
}
impl Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Robot({}@{})", self.position, self.velocity)
    }
}
type Input = Vec<Robot>;


fn parse_number(str: &str) -> IResult<&str, i64> {
    use nom::combinator::recognize;
    use nom::combinator::opt as optional;
    use nom::character::complete::digit1 as digit1;
    use nom::character::complete::one_of;

    recognize(
    (optional(one_of("-")), digit1)
    ).map_res(str::parse).parse(str)
}
fn parse_vector(str: &str) -> IResult<&str, Vector2D> {
    use parse_number as number;
    use nom::sequence::separated_pair;
    use nom::bytes::complete::tag;

    separated_pair(number, tag(","), number)
        .map(|(x, y)| Vector2D { x, y })
        .parse(str)
}
fn parse_robot(str: &str) -> IResult<&str, Robot> {
    use nom::sequence::preceded;
    use nom::sequence::separated_pair;
    use nom::bytes::complete::tag;
    use parse_vector as vector;

    separated_pair(
        preceded(tag("p="), vector),
        tag(" "),
        preceded(tag("v="), vector)
    )
        .map(|(position, velocity)| Robot { position, velocity })
        .parse(str)
}
fn parse_input(str: &str) -> Result<Input, nom::Err<nom::error::Error<&str>>> {
    use nom::multi::separated_list1;
    use nom::character::complete::newline;
    use nom::combinator::all_consuming;
    use parse_robot as robot;

    Ok(all_consuming(separated_list1(newline, robot)).parse(str)?.1)
}
fn safety_factor_after_n_seconds(input: &Input, seconds: u64, width: i128, height: i128) -> u128 {
    use std::cmp::Ordering::{Greater, Less};
    let mut q1: u128 = 0;
    let mut q2: u128 = 0;
    let mut q3: u128 = 0;
    let mut q4: u128 = 0;
    for robot in input {
        // i64 -> i128 = safe
        // and robots are always within a 101x103 square anyway
        let start_x = robot.position.x as i128;
        let start_y = robot.position.y as i128;
        // multiplication is checked, modulus can only make the number smaller
        let mod_dx = (robot.velocity.x as i128).checked_mul(seconds as i128).expect("Overflow for dx") % width;
        let mod_dy = (robot.velocity.y as i128).checked_mul(seconds as i128).expect("Overflow for dy") % height;
        // start position is small (within 0..103), widthh and height are 101,103, no chance of overflow for i128
        let end_x = (start_x + mod_dx) % (width);
        let end_y = (start_y + mod_dy) % (height);
        
        match (end_x.cmp(&(width/2)), end_y.cmp(&(height/2))) {
            (Greater, Greater) => {q1 += 1;},
            (Greater, Less) => {q2 += 1;},
            (Less, Greater) => {q3 += 1;},
            (Less, Less) => {q4 += 1},
            _ => {
                // Robot is exactly in the middle on at least one axis, does not contribute to safety
            }
        }
    }
    return q1 * q2 * q3 * q4;
}
#[test]
fn test_part1() {
    let demo = read_input_file(DAY, "demo.txt");
    let robots = parse_input(&demo);
    match robots {
        Err(e) => panic!("Failed to parse robots: {}", e),
        Ok(robots) => {
            let safety_factor = safety_factor_after_n_seconds(&robots, 100, 11, 7);
            assert_eq!(safety_factor, 12);
        }
    }
}
#[test]
fn test_part1_wrong_submissions() {
    let full = read_input_file(DAY, "full.txt");
    let robots = parse_input(&full);
    match robots {
        Err(e) => panic!("Failed to parse robots: {}", e),
        Ok(robots) => {
            println!("{}", robots.last().unwrap());
            let found = safety_factor_after_n_seconds(&robots, 100, 101, 103);
            println!("{:?}", found);
            assert!(found < 226839600);
            assert!(110149200 < found);
        }
    }
    
}


pub fn part1() -> u128 {
    let full = read_input_file(DAY, "full.txt");
    let robots = parse_input(&full);
    match robots {
        Err(e) => panic!("Failed to parse robots: {}", e),
        Ok(robots) => {
            return safety_factor_after_n_seconds(&robots, 100, 101, 103);
        }
    }
}
pub fn part2() -> u128 {
    todo!()
}