use crate::day6::Cell::Obstructed;
use crate::utils::read_input_file;
use nom::Parser;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;
use std::ops::Index;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::thread;

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
            Cell::Free => write!(f, "·"),
            Cell::Visited => write!(f, "⨯"),
            Cell::Obstructed => write!(f, "█"),
        }
    }
}
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
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
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
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
    /// (col, row)
    fn as_usize_pair(self) -> Result<(usize, usize), TryFromIntError> {
        let col = usize::try_from(self.col)?;
        let row = usize::try_from(self.row)?;
        Ok((col, row))
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
    fn is_free(&self, row: usize, col: usize) -> bool {
        self.map[row * self.width + col] == Cell::Free
    }
    fn is_passable(&self, row: usize, col: usize) -> bool {
        let x = *self.map.index(col + (row * self.width));
        x != Cell::Obstructed
    }
    fn is_on_map(&self, row: usize, col: usize) -> bool {
        col < self.width && row < self.height
    }
    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.map[row * self.width + col] = cell;
        // ^ Shouldn't this be illegal? am i not mutating an immutable reference?
    }
    /// Simulates one step of the basic task (first half)
    /// returns false if the guard has walked off the map and no more simulation can be performed
    fn tick_basic(&mut self) -> bool {
        let pos = self.guard_position;
        if let Ok((col, row)) = pos.as_usize_pair() {
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
        if let Ok((col, row)) = fwd.as_usize_pair() {
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
                if self.guard_position.col == x as isize && self.guard_position.row == y as isize {
                    match self.guard_position.dir {
                        Direction::North => write!(f, "{}", "⮝")?,
                        Direction::East => write!(f, "{}", "⮞")?,
                        Direction::South => write!(f, "{}", "⮟")?,
                        Direction::West => write!(f, "{}", "⮜")?,
                    }
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
fn solve_advanced(initial_state: &State) -> usize {
    let initial_pos = initial_state.guard_position;
    let mut state = initial_state.clone();
    let mut candidate_positions: Vec<Position> = Vec::new();
    loop {
        let next = state.guard_position.step_forward();
        if let Ok((col, row)) = next.as_usize_pair() {
            if state.is_on_map(row, col) && state.is_free(row, col) {
                if initial_pos.as_usize_pair() != next.as_usize_pair() {
                    candidate_positions.push(state.guard_position);
                }
                // Test for obstacle at row-col
            }
        }
        let running = state.tick_basic();
        if !running {
            break;
        }
    }
    let total = Arc::new(AtomicUsize::new(0));
    thread::scope(|scope| {
        for start in candidate_positions {
            let total = total.clone();
            let mut state = initial_state.clone();
            scope.spawn(move || {
                if let Ok((col, row)) = start.step_forward().as_usize_pair() {
                    state.set_cell(row, col, Obstructed);
                    state.guard_position = initial_pos;
                }
                let mut collisions: HashSet<Position> = HashSet::new();
                loop {
                    let next = state.guard_position.step_forward();
                    if let Ok((col, row)) = next.as_usize_pair() {
                        if state.is_on_map(row, col) && !state.is_passable(row, col) {
                            if !collisions.insert(state.guard_position) {
                                // Loop detected
                                total.fetch_add(1, Relaxed);
                                return;
                            }
                        }
                    }
                    let running = state.tick_basic();
                    if !running {
                        // Ran out of simulation
                        return
                    }
                }
            });
        }
    });
    let total = total.load(Relaxed);
    return total;
}


#[test]
fn test_solve_simple() {
    let state = parse_file("demo.txt").expect("Demo file should parse");
    assert_eq!(solve_simple(&state), 41);
}
#[test]
fn test_solve_advanced() {
    let state = parse_file("demo.txt").expect("Demo file should parse");
    assert_eq!(solve_advanced(&state), 6);
}

pub fn part1() -> usize {
    let state = parse_file("full.txt").expect("Demo file should parse");
    solve_simple(&state)
}
pub fn part2() -> usize {
    let state = parse_file("full.txt").expect("Demo file should parse");
    solve_advanced(&state)
}