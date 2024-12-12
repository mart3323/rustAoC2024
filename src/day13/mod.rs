use crate::utils::read_input_file;
use nom::{IResult, Parser};

const DAY: &str = "day13";

#[derive(Debug)]
struct Vector2D {
    x: usize, 
    y: usize,
}
#[derive(Debug)]
struct ClawGame {
    goal: Vector2D,
    button_a: Vector2D,
    button_b: Vector2D,
}
struct Solution {
    a_presses: usize,
    b_presses: usize,
}
impl Solution {
    fn get_cost(&self) -> usize {
        return self.a_presses*3 + self.b_presses;
    }
}

fn parse_number(str: &str) -> IResult<&str, usize> {
    nom::character::complete::digit1.map_res(str::parse::<usize>).parse(str)
}
fn parse_claw_game(str: &str) -> IResult<&str, ClawGame> {
    let tag = nom::bytes::tag;
    let number = parse_number;
    let line1 = (tag("Button A: X+"), number, tag(", Y+"), number)
        .map(|(_, x, _, y)| Vector2D{ x, y });
    let line2 = (tag("Button B: X+"), number, tag(", Y+"), number)
        .map(|(_, x, _, y)| Vector2D{ x, y });
    let line3 = (tag("Prize: X="), number, tag(", Y="), number)
        .map(|(_, x, _, y)| Vector2D{ x, y });

    let newline = nom::character::complete::newline;
    return (line1, newline, line2, newline, line3)
        .map(|(button_a,_, button_b,_, goal)| ClawGame {goal, button_a, button_b})
        .parse(str);
}
fn parse_claw_games(str: &str) -> IResult<&str, Vec<ClawGame>> {
    let newline= nom::character::complete::newline;
    return nom::multi::separated_list1((newline, newline), parse_claw_game)
        .parse(str);
}

impl ClawGame {
    fn solve(&self) -> Option<Solution> {
        let Vector2D { x: x1, y: y1 } = self.button_a;
        let Vector2D { x: x2, y: y2 } = self.button_b;
        let Vector2D { x: xt, y: yt } = self.goal;
        
        // k = (y2*xt - yt*x2) / (x1*y2 - x2*y1)
        let a_presses = (y2*xt).abs_diff(yt*x2) / (x1*y2).abs_diff(x2*y1);
        
        let moved_x = x1 * a_presses;
        if moved_x > xt {
            return None;
        }
        let b_presses = (xt - moved_x) / x2;
        
        if a_presses * x1 + b_presses * x2 == xt {
            if a_presses * y1 + b_presses * y2 == yt {
                return Some(Solution { a_presses, b_presses });
            };
        };
        return None;
    }
}
#[test]
fn test_part1() {
    let demo = read_input_file(DAY, "demo.txt");
    let games = parse_claw_games(&demo).unwrap().1;
    let mut total = 0;
    for game in games {
        if let Some(solution) = game.solve() {
            total += solution.get_cost();
        }
    }
    assert_eq!(total, 480);
}


pub fn part1() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let games = parse_claw_games(&full).unwrap().1;
    let mut total = 0;
    for game in games {
        if let Some(solution) = game.solve() {
            total += solution.get_cost();
        }
    }
    return total;
}
pub fn part2() -> usize {
    const OFFSET: usize = 10000000000000;
    
    let full = read_input_file(DAY, "full.txt");
    let mut games = parse_claw_games(&full).unwrap().1;
    let mut total = 0;
    for mut game in games {
        game.goal = Vector2D{ x: game.goal.x + OFFSET, y: game.goal.y + OFFSET };
        if let Some(solution) = game.solve() {
            total += solution.get_cost();
        }
    }
    return total;
}