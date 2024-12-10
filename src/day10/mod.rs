use std::collections::HashSet;
use crate::utils::read_input_file;

const DAY: &str = "day10";

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize
}
impl Pos {
    fn get_neighbors(&self) -> Vec<Pos> {
        let mut neighbors = Vec::with_capacity(4);
        neighbors.push(Pos{x: self.x, y: self.y + 1});
        neighbors.push(Pos{x: self.x + 1, y: self.y});
        if self.y > 0 {
            neighbors.push(Pos{x: self.x, y: self.y - 1});
        }
        if self.x > 0 {
            neighbors.push(Pos{x: self.x - 1, y: self.y});
        }
        return neighbors;
    }
    fn read_from(&self, map: &Vec<Vec<char>>) -> Option<char> {
        let row = map.get(self.y)?;
        let c = row.get(self.x)?;
        return Some(*c);
    }
}
impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Pos {
        Pos{x, y}
    }
}

fn score_trailheads(map: &Vec<Vec<char>>) -> usize {
    let width = map.get(0).unwrap().len();
    let height = map.len();
    let positions = (0..width).flat_map(|x|
        (0..height).map(move |y| Pos::from((x,y)))
    );
    let zero_positions = positions.filter(|v| v.read_from(&map).is_some_and(|v| v == '0'));

    let score = zero_positions.map(|pos| -> usize {
        let mut neighbors = HashSet::from([pos]);
        for i in vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            neighbors = neighbors.iter()
                .flat_map(|pos| pos.get_neighbors())
                .filter(|pos| pos.read_from(&map).is_some_and(|c| c == i))
                .collect();
        };
        return neighbors.len();
    }).sum();

    return score;
}
fn rate_trailheads(map: &Vec<Vec<char>>) -> usize {
    let width = map.get(0).unwrap().len();
    let height = map.len();
    let positions = (0..width).flat_map(|x|
        (0..height).map(move |y| Pos::from((x,y)))
    );
    let zero_positions = positions.filter(|v| v.read_from(&map).is_some_and(|v| v == '0'));

    let score = zero_positions.map(|pos| -> usize {
        let mut neighbors = vec!(pos);
        for i in vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            neighbors = neighbors.iter()
                .flat_map(|pos| pos.get_neighbors())
                .filter(|pos| pos.read_from(&map).is_some_and(|c| c == i))
                .collect();
        };
        return neighbors.len();
    }).sum();

    return score;
}
#[test]
fn test_score_trailheads() {
    let demo = read_input_file(DAY, "demo.txt");
    let lines: Vec<Vec<char>> = demo.split_whitespace().map(|line| line.chars().collect()).collect();

    let score = score_trailheads(&lines);
    assert_eq!(score, 36usize)
}

#[test]
fn test_rate_trailheads() {
    let demo = read_input_file(DAY, "demo.txt");
    let lines: Vec<Vec<char>> = demo.split_whitespace().map(|line| line.chars().collect()).collect();

    let score = rate_trailheads(&lines);
    assert_eq!(score, 81usize)
}

pub fn part1() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let lines: Vec<Vec<char>> = full.split_whitespace().map(|line| line.chars().collect()).collect();
    let score = score_trailheads(&lines);
    return score;
}
pub fn part2() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let lines: Vec<Vec<char>> = full.split_whitespace().map(|line| line.chars().collect()).collect();
    let rating = rate_trailheads(&lines);
    return rating;
}