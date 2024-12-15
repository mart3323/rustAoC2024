use std::collections::HashSet;
use nom::Parser;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::num::TryFromIntError;
use std::str::FromStr;
use log::LevelFilter::Off;
use crate::day15::parsers::parse_input;
use crate::day15::types::{Direction, Point, Tile, Warehouse};
use crate::utils::read_input_file;

mod types;
mod parsers;

const DAY: &str = "day15";


struct Offset {
    col: isize,
    row: isize,
}
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Pos {col: isize, row: isize}
impl Pos {
    fn offset_to(&self, other: &Self) -> Offset {
        Offset{
            col: other.col - self.col,
            row: other.row - self.row,
        }
    }
    fn apply_offset(&self, offset: &Offset) -> Self {
        Self {
            col: self.col + offset.col,
            row: self.row + offset.row
        }
    }
    fn apply_offset_times(&self, offset: &Offset, times: isize) -> Self {
        Self {
            col: self.col + (times * offset.col),
            row: self.row + (times * offset.row),
        }
    }
    /// (col, row)
    fn as_usize_pair(self) -> Result<(usize, usize), TryFromIntError> {
        let col = usize::try_from(self.col)?;
        let row = usize::try_from(self.row)?;
        Ok((col, row))
    }
}
#[derive(Clone)]
struct Map2D<T> where T: Display{
    map: Vec<T>,
    width: usize,
    height: usize,
}

enum Error { OutOfBounds }
impl<T: Display> Map2D<T> {
    /// Returns the given position as a map index, ONLY IF the position is within the bounds of the map
    fn as_index(&self, pos: &Pos) -> Option<usize> {
        if 0 <= pos.col && pos.col <= self.width as isize &&
            0 <= pos.row && pos.row <= self.height as isize {
            Some((pos.col + (self.width as isize * pos.row)) as usize)
        } else {
            None
        }
    }
    fn as_pos(&self, index: usize) -> Pos {
        Pos {
            row: (index / self.width) as isize,
            col: (index % self.width) as isize,
        }
    }
    fn try_set_cell(&mut self, pos: &Pos, cell: T) -> Result<(), ()> {
        let index = self.as_index(pos);
        if let Some(index) = index {
            self.map[index] = cell;
            return Ok(())
        }
        return Err(())
    }
    fn get_cell(&self, pos: &Pos) -> Option<&T> {
        let i = self.as_index(pos)?;
        self.map.get(i)
    }
    fn set_cell(&mut self, pos: &Pos, cell: T) -> Result<(), ()> {
        let index = self.as_index(pos).ok_or(())?;
        self.map[index] = cell;
        Ok(())
    }
    fn update_cell(&mut self, pos: &Pos, update: fn(&T) -> T) -> Result<(), ()> {
        let prev = self.get_cell(pos).ok_or(())?;
        self.set_cell(pos, update(prev))
    }
    fn enumerate(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.map.iter().enumerate().map(|(i, x)| (self.as_pos(i), x))
    }
}
impl<T: Display> Display for Map2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.map[x + self.width*y])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn to_offset(dir: Direction) -> Offset {
    match dir {
        Direction::North => Offset { col: 0, row: -1},
        Direction::East => Offset { col: 1, row: 0},
        Direction::South => Offset { col: 0, row: 1},
        Direction::West => Offset { col: -1, row: 0},
    }
}
#[test]
fn test_part1() {
    let txt = read_input_file(DAY, "demo.txt");
    let (wh, moves) = parse_input(&txt).unwrap().1;
    let height = wh.get_height();
    let width = wh.width;
    let mut robot_position = Pos {col: -1, row: -1};
    let mut map = Map2D {
        height,
        width,
        map: wh.map.iter().enumerate().map(|(i, x)| {
            if *x == Tile::Robot {
                robot_position = Pos {
                    col: (i % width) as isize,
                    row: (i / width) as isize,
                }
            }
            return x;
        }).collect(),
    };
    for mv in moves {
        let offset = to_offset(mv);
        let first = robot_position.apply_offset(&offset);
        let mut end = first;
        while map.get_cell(&end).is_some_and(|&c| c == &Tile::Box) {
            end = end.apply_offset(&offset);
        }
        match map.get_cell(&end) {
            None | Some(Tile::Wall) => {
                // Blocked, do nothing
            },
            Some(Tile::Empty) => {
                // Do push
                map.set_cell(&end, &Tile::Box).expect("Unsuccessful write");
                map.set_cell(&first, &Tile::Empty).expect("Unsuccessful write");
                map.set_cell(&robot_position, &Tile::Empty).expect("Unsuccessful write");
                robot_position = first;
            }
            // Robot was removed from map before creation
            // Boxes should have gotten iterated over until we found a non box tile
            Some(Tile::Robot) | Some(Tile::Box) => panic!("This should be impossible"),
        }
    }
    map.set_cell(&robot_position, &Tile::Robot).expect("Unsuccessful write");

    let mut total = 0;
    map.enumerate()
        .filter(|(_,&t)| t == &Tile::Box)
        .for_each(|(pos, _)| {
            total += pos.col + (100 * pos.row)
        });

    assert_eq!(total, 10092);
}

pub fn part1() -> usize {
    let txt = read_input_file(DAY, "full.txt");
    let (wh, moves) = parse_input(&txt).unwrap().1;
    let height = wh.get_height();
    let width = wh.width;
    let mut robot_position = Pos {col: -1, row: -1};
    let mut map = Map2D {
        height,
        width,
        map: wh.map.iter().enumerate().map(|(i, x)| {
            if *x == Tile::Robot {
                robot_position = Pos {
                    col: (i % width) as isize,
                    row: (i / width) as isize,
                }
            }
            return x;
        }).collect(),
    };
    for mv in moves {
        let offset = to_offset(mv);
        let first = robot_position.apply_offset(&offset);
        let mut end = first;
        while map.get_cell(&end).is_some_and(|&c| c == &Tile::Box) {
            end = end.apply_offset(&offset);
        }
        match map.get_cell(&end) {
            None | Some(Tile::Wall) => {
                // Blocked, do nothing
            },
            Some(Tile::Empty) => {
                // Do push
                map.set_cell(&end, &Tile::Box).expect("Unsuccessful write");
                map.set_cell(&first, &Tile::Empty).expect("Unsuccessful write");
                map.set_cell(&robot_position, &Tile::Empty).expect("Unsuccessful write");
                robot_position = first;
            }
            // Robot was removed from map before creation
            // Boxes should have gotten iterated over until we found a non box tile
            Some(Tile::Robot) | Some(Tile::Box) => panic!("This should be impossible"),
        }
    }
    map.set_cell(&robot_position, &Tile::Robot).expect("Unsuccessful write");

    let mut total = 0usize;
    map.enumerate()
        .filter(|(_,&t)| t == &Tile::Box)
        .for_each(|(pos, _)| {
            total += (pos.col + (100 * pos.row)) as usize
        });
    return total;
}

#[test]
fn test_part2() {
    let txt = read_input_file(DAY, "demo.txt");
    let (wh, moves) = parse_input(&txt).unwrap().1;
    let height = wh.get_height();
    let width = wh.width;
    let mut robot_position = Pos {col: -1, row: -1};
    let mut map = Map2D {
        height,
        width: width * 2,
        map: wh.map.iter().enumerate().flat_map(|(i, x)| {
            if *x == Tile::Robot {
                robot_position = Pos {
                    col: ((i % width) as isize) * 2,
                    row: (i / width) as isize,
                }
            }
            match x {
                Tile::Empty => ['.','.'],
                Tile::Box => ['[',']'],
                Tile::Wall => ['#','#'],
                Tile::Robot => ['@','.'],
            }
        }).collect(),
    };
    for mv in moves {
        let mut tiles_to_move: Vec<Vec<Pos>> = Vec::new();
        let offset = to_offset(mv);
        tiles_to_move.push(vec!(robot_position));
        let mut blocked = false;
        'push_propagation:
        loop {
            let mut next_tiles: HashSet<Pos> = HashSet::new();
            for pos in tiles_to_move.last().unwrap().iter() {
                let pos = pos.apply_offset(&offset);
                match map.get_cell(&pos) {
                    None => {
                        blocked = true;
                        break 'push_propagation;
                    }
                    Some(t) => match t {
                        '#' => {
                            blocked = true;
                            break 'push_propagation;
                        }
                        '[' => {
                            next_tiles.insert(pos);
                            if offset.row != 0 {
                                next_tiles.insert(pos.apply_offset(&Offset{col: 1, row: 0}));
                            }
                        }
                        ']' => {
                            next_tiles.insert(pos);
                            if offset.row != 0 {
                                next_tiles.insert(pos.apply_offset(&Offset{col: -1, row: 0}));
                            }
                        },
                        '.' => {}
                        '@'|_ => panic!("This should be impossible"),
                    }
                }
            }
            if next_tiles.is_empty() {
                break 'push_propagation;
            } else {
                tiles_to_move.push(next_tiles.into_iter().collect());
            }
        }
        if !blocked {
            for positions in tiles_to_move.iter().rev() {
                for pos in positions {
                    map.set_cell(&pos.apply_offset(&offset), *map.get_cell(pos).expect("Unsuccessful read")).expect("Unsuccessful write");
                    map.set_cell(&pos, '.').expect("Unsuccessful write");
                }
            }
            robot_position = robot_position.apply_offset(&offset);
        }
    }
    map.set_cell(&robot_position, '@').expect("Unsuccessful write");

    let mut total = 0;
    map.enumerate()
        .filter(|(_,&t)| t == '[')
        .for_each(|(pos, _)| {
            total += pos.col + (100 * pos.row)
        });

    assert_eq!(total, 9021);
}

pub fn part2() -> usize {
    let txt = read_input_file(DAY, "full.txt");
    let (wh, moves) = parse_input(&txt).unwrap().1;
    let height = wh.get_height();
    let width = wh.width;
    let mut robot_position = Pos {col: -1, row: -1};
    let mut map = Map2D {
        height,
        width: width * 2,
        map: wh.map.iter().enumerate().flat_map(|(i, x)| {
            if *x == Tile::Robot {
                robot_position = Pos {
                    col: ((i % width) as isize) * 2,
                    row: (i / width) as isize,
                }
            }
            match x {
                Tile::Empty => ['.','.'],
                Tile::Box => ['[',']'],
                Tile::Wall => ['#','#'],
                Tile::Robot => ['@','.'],
            }
        }).collect(),
    };
    for mv in moves {
        let mut tiles_to_move: Vec<Vec<Pos>> = Vec::new();
        let offset = to_offset(mv);
        tiles_to_move.push(vec!(robot_position));
        let mut blocked = false;
        'push_propagation:
        loop {
            let mut next_tiles: HashSet<Pos> = HashSet::new();
            for pos in tiles_to_move.last().unwrap().iter() {
                let pos = pos.apply_offset(&offset);
                match map.get_cell(&pos) {
                    None => {
                        blocked = true;
                        break 'push_propagation;
                    }
                    Some(t) => match t {
                        '#' => {
                            blocked = true;
                            break 'push_propagation;
                        }
                        '[' => {
                            next_tiles.insert(pos);
                            if offset.row != 0 {
                                next_tiles.insert(pos.apply_offset(&Offset{col: 1, row: 0}));
                            }
                        }
                        ']' => {
                            next_tiles.insert(pos);
                            if offset.row != 0 {
                                next_tiles.insert(pos.apply_offset(&Offset{col: -1, row: 0}));
                            }
                        },
                        '.' => {}
                        '@'|_ => panic!("This should be impossible"),
                    }
                }
            }
            if next_tiles.is_empty() {
                break 'push_propagation;
            } else {
                tiles_to_move.push(next_tiles.into_iter().collect());
            }
        }
        if !blocked {
            for positions in tiles_to_move.iter().rev() {
                for pos in positions {
                    map.set_cell(&pos.apply_offset(&offset), *map.get_cell(pos).expect("Unsuccessful read")).expect("Unsuccessful write");
                    map.set_cell(&pos, '.').expect("Unsuccessful write");
                }
            }
            robot_position = robot_position.apply_offset(&offset);
        }
    }
    map.set_cell(&robot_position, '@').expect("Unsuccessful write");

    let mut total: usize = 0;
    map.enumerate()
        .filter(|(_,&t)| t == '[')
        .for_each(|(pos, _)| {
            total += (pos.col + (100 * pos.row)) as usize
        });

    return total;
}