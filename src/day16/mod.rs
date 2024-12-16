use std::collections::HashSet;
use nom::combinator::map;
use nom::Parser;
use crate::utils::read_input_file;
use pathfinding::prelude::{astar, astar_bag};

const DAY: &str = "day16";

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
}
impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Dir {
    North, East, South, West,
}
impl Dir {
    fn right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
    fn left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn step(&self, dir: &Dir) -> Self {
        match dir {
            Dir::North => Self { x: self.x, y: self.y - 1 },
            Dir::East => Self { x: self.x + 1, y: self.y },
            Dir::South => Self { x: self.x, y: self.y + 1 },
            Dir::West => Self { x: self.x - 1, y: self.y },
        }
    }
}
#[derive(Debug)]
struct Map<T: Copy> {
    data: Vec<T>,
    width: usize,
}
impl<T: Copy> Map<T> {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn contains_pos(&self, pos: &Pos) -> bool {
        0 < pos.x && pos.x < (self.width as i32) && 0 < pos.y && pos.y < (self.height() as i32)
    }
    fn get_at(&self, pos: &Pos) -> T {
        self.data[pos.y as usize * self.width + pos.x as usize]
    }
    fn set_at(&mut self, pos: &Pos, val: T) {
        self.data[pos.y as usize * self.width + pos.x as usize] = val;
    }
}

struct Maze {
    start: Pos,
    end: Pos,
    map: Map<Tile>
}

fn parse(str: &str) -> Result<Maze, &str> {
    let lines = str.lines();
    let map2d: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let width = map2d.first().expect("Map should have at least one line").len();
    let height = map2d.len();

    let mut start = None;
    let mut end = None;

    let mut data: Vec<Tile> = Vec::with_capacity(width*height);
    for (y, row) in map2d.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                if let Some(_) = start {
                    return Err("Multiple starts found on map")
                }
                start = Some(Pos { x: x as i32, y: y as i32 });
            } else if ch == 'E' {
                if let Some(_) = end {
                    return Err("Multiple ends found on map")
                }
                end = Some(Pos { x: x as i32, y: y as i32 });
            }
            match ch {
                'S'|'E'|'.' => data.push(Tile::Empty),
                '#' => data.push(Tile::Wall),
                _ => return Err("Unknown character in map")
            }
        }
    }
    if let Some(start) = start {
        if let Some(end) = end {
            Ok(Maze { start, end, map: Map { data, width } })
        } else {
            Err("No end found on map")
        }
    } else {
        Err("No start found on map")
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Reindeer {
    position: Pos,
    direction: Dir,
}
impl Reindeer {
    const AVAILABLE_MOVES: [Cmd;3] = [Cmd::Fwd, Cmd::TurnRight, Cmd::TurnLeft];
    fn apply(&self, cmd: &Cmd) -> Reindeer {
        match cmd {
            Cmd::Fwd => Reindeer {
                position: self.position.step(&self.direction),
                direction: self.direction,
            },
            Cmd::TurnRight => Reindeer {
                position: self.position,
                direction: self.direction.right(),
            },
            Cmd::TurnLeft => Reindeer {
                position: self.position,
                direction: self.direction.left(),
            }
        }
    }
}

enum Cmd { Fwd, TurnRight, TurnLeft }
type Path = Vec<Cmd>;
trait Cost {
    fn cost(&self) -> usize;
}
impl Cost for Cmd {
    fn cost(&self) -> usize {
        match self {
            Cmd::Fwd => 1,
            Cmd::TurnRight | Cmd::TurnLeft => 1000,
        }
    }
}
impl Cost for Path {
    fn cost(&self) -> usize {
        self.iter().map(Cmd::cost).sum()
    }
}

fn get_available_moves(maze: &Map<Tile>, pos: Pos, dir: Dir) -> Vec<Cmd> {
    Reindeer::AVAILABLE_MOVES.into_iter().filter(|mv| {
        match mv {
            Cmd::TurnRight| Cmd::TurnLeft => true,
            Cmd::Fwd => maze.get_at(&pos.step(&dir)) == Tile::Empty,
        }
    }).collect()
}

fn min_bound_cost(from: Pos, to:Pos) -> usize {
    let mut total = 0;
    let x_diff = from.x.abs_diff(to.x) as usize;
    let y_diff = from.y.abs_diff(to.y) as usize;
    total += (x_diff + y_diff) * Cmd::Fwd.cost();
    if x_diff > 0 && y_diff > 0 {
        total += Cmd::TurnRight.cost();
    }
    return total;
}

fn solve_simple(maze: &Maze) -> Result<usize, &str> {
    let result = astar(
        &Reindeer { position: maze.start, direction: Dir::East },
        |&r| -> Vec<(Reindeer, usize)> {
            get_available_moves(&maze.map, r.position, r.direction).iter().map(|m| (r.apply(m), m.cost())).collect()
        },
        |&r| min_bound_cost(r.position, maze.end),
        |state: &Reindeer| state.position == maze.end,
    );

    if let Some(result) = result {
        Ok(result.1)
    } else {
        Err("No solution found")
    }
}
fn solve_advanced(maze: &Maze) -> Result<usize, &str> {
    let result = astar_bag(
        &Reindeer { position: maze.start, direction: Dir::East },
        |&r| -> Vec<(Reindeer, usize)> {
            get_available_moves(&maze.map, r.position, r.direction).iter().map(|m| (r.apply(m), m.cost())).collect()
        },
        |&r| min_bound_cost(r.position, maze.end),
        |state: &Reindeer| state.position == maze.end,
    );

    if let Some(result) = result {
        let paths = result.0;
        let positions: HashSet<Pos> = paths.flat_map(|path| path.iter().map(|n: &Reindeer| n.position).collect::<Vec<Pos>>()).collect();
        Ok(positions.len())
    } else {
        Err("No solution found")
    }
}

#[test]
fn test_part1() {
    let demo1 = parse(&read_input_file(DAY, "demo1.txt")).expect("Failed to parse demo1.txt");
    let demo2 = parse(&read_input_file(DAY, "demo2.txt")).expect("Failed to parse demo2.txt");

    let cost1 = solve_simple(&demo1).expect("Problem to be solvable");
    let cost2 = solve_simple(&demo2).expect("Problem to be solvable");
    assert_eq!(cost1, 7036);
    assert_eq!(cost2, 11048);
}

pub fn part1() -> usize {
    let full = parse(&read_input_file(DAY, "full.txt")).expect("Failed to parse full.txt");

    let cost = solve_simple(&full).expect("Problem to be solvable");
    return cost;
}

#[test]
fn test_part2() {
    let demo1 = parse(&read_input_file(DAY, "demo1.txt")).expect("Failed to parse demo1.txt");
    let demo2 = parse(&read_input_file(DAY, "demo2.txt")).expect("Failed to parse demo2.txt");

    let cost1 = solve_advanced(&demo1).expect("Problem to be solvable");
    let cost2 = solve_advanced(&demo2).expect("Problem to be solvable");
    assert_eq!(cost1, 45);
    assert_eq!(cost2, 64);
}

pub fn part2() -> usize {
    let full = parse(&read_input_file(DAY, "full.txt")).expect("Failed to parse full.txt");

    let cost = solve_advanced(&full).expect("Problem to be solvable");
    return cost;
}