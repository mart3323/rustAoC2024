use crate::utils::read_input_file;
use nom::Parser;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Cell {
    Free,
    Visited,
    Obstructed
}
impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cell::Obstructed),
            '.' => Ok(Cell::Free),
            '^' => Ok(Cell::Free),
            'X' => Ok(Cell::Visited),
            _ => Err(()),
        }
    }

}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Free => write!(f, "."),
            Cell::Visited => write!(f, "X"),
            Cell::Obstructed => write!(f, "#"),
        }
    }
}
#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
#[derive(Eq, PartialEq, Clone, Copy)]
struct Position {col: isize, row: isize, dir: Direction}
impl Position {
    fn offset(&self, dir: Direction, distance: isize) -> Self {
        let mut newpos = self.clone();
        match dir {
            Direction::North => newpos.row -= distance,
            Direction::East => newpos.col += distance,
            Direction::South => newpos.row += distance,
            Direction::West => newpos.col -= distance,
        }
        newpos
    }
    fn step(&self, dir: Direction) -> Self {
        self.offset(dir, 1)
    }
    fn step_forward(&self) -> Self {
        self.offset(self.dir, 1)
    }
    fn turn_right(&self) -> Self {
        let mut newpos = self.clone();
        newpos.dir = newpos.dir.right();
        newpos
    }
}
#[derive(Clone)]
struct State {
    map: Vec<Cell>,
    width: usize,
    height: usize,
    guard_position: Position
}

enum Error { OutOfBounds }
impl State {
    fn visit(&mut self, row: usize, col: usize) {
        self.map[row * self.width + col] = Cell::Visited;
    }
    fn is_passable(&self, row: usize, col: usize) -> bool {
        let x = *self.map.index(col + (row * self.width));
        x != Cell::Obstructed
    }
    fn is_on_map(&self, row: usize, col: usize) -> bool {
        col < self.width && row < self.height
    }
    /// Simulates one step of the basic task (first half)
    /// returns false if the guard has walked off the map and no more simulation can be performed
    fn tick_basic(&mut self) -> bool {
        let pos = self.guard_position;
        if let (Ok(col), Ok(row)) = (usize::try_from(pos.col), usize::try_from(pos.row)) {
            if self.is_on_map(row, col) {
                // OK: Still on map
                self.visit(row, col)
            } else {
                // No longer on map
                return false
            }
        } else {
            // Below 0 on either axis
            return false;
        }
        
        let fwd = pos.step_forward();
        if let (Ok(col), Ok(row)) = (usize::try_from(fwd.col), usize::try_from(fwd.row)) {
            if self.is_on_map(row, col) && !self.is_passable(row, col) {
                self.guard_position = self.guard_position.turn_right()
            } else {
                self.guard_position = fwd;
            }
        } else {
            self.guard_position = fwd;
        }
        true
    }

    fn count_visited(&self) -> usize {
        self.map.iter().filter(|x| **x == Cell::Visited).count()
    }
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.guard_position == (Position{ col: x as isize, row: y as isize, dir: Direction::North }) {
                } else {
                    write!(f, "{}", self.map[x + self.width*y])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// region input
fn parse_file(fname: &str) -> Result<State, ()> {
    let data = read_input_file("day6", fname);
    let (first, _) = data.split_once("\n").ok_or(()).expect("Map should have at least two lines to determine width");
    let width: usize = first.len();
    let guard_pos_index = data.chars().filter(|c| !c.is_whitespace()).position(|c| c == '^').expect("Map should have a guard marked by ^ character");
    Ok(State{
        map: data.chars().filter(|c| !c.is_whitespace()).map(Cell::try_from).collect::<Result<Vec<Cell>, _>>()?,
        width: width,
        height: data.len() / width,
        guard_position: Position {
            row: (guard_pos_index / width) as isize,
            col: (guard_pos_index % width) as isize,
            dir: Direction::North
        }
    })
}
// endregion


fn solve_simple(initial_state: &State) -> usize {
    let mut state = initial_state.clone();
    while state.tick_basic() {
    }
    state.count_visited()
}

pub fn solve_day6() {
    let demo_state = parse_file("demo.txt").expect("demo.txt failed to parse");
    let full_state = parse_file("full.txt").expect("full.txt failed to parse");

    assert_eq!(solve_simple(&demo_state), 41usize);
    println!("Demo 1 passed");
    println!("full solution is {}", solve_simple(&full_state));

    // assert_eq!(solve_advanced(&demo.0, &mut demo.1.clone()), 123);
    // println!("Demo 2 passed");
    // println!("full solution is {}", solve_advanced(&full.0, &mut full.1.clone()));
    
}
