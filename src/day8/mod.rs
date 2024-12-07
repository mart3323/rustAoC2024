use crate::utils::read_input_file;
use nom::Parser;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;

type Frequency = char;
type Antenna = Frequency;
type Strength = usize;

#[derive(Eq, PartialEq, Clone, Copy)]
struct Cell {
    antenna: Option<Antenna>,
}
impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell{ antenna: None }),
            _ => Ok(Cell{ antenna: Some(c) }),
        }
    }

}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell{ antenna: Some(antenna) } => write!(f, "{}", antenna),
            Cell{ antenna: None } => write!(f, "."),
        }
    }
}
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

// region input
fn parse_file(fname: &str) -> Result<Map2D<Cell>, ()> {
    let data = read_input_file("day8", fname);
    let (first, _) = data.split_once("\n").ok_or(()).expect("Map should have at least two lines to determine width");
    let width: usize = first.len();
    Ok(Map2D {
        map: data.chars().filter(|c| !c.is_whitespace()).map(Cell::try_from).collect::<Result<Vec<Cell>, _>>()?,
        width: width,
        height: data.len() / width,
    })
}
// endregion

type AntennaMap = Map2D<Cell>;
type AntinodesMap = Map2D<usize>;

fn solve_simple(initial_state: &AntennaMap) -> usize {
    let mut antenna_locations_by_frequency: HashMap<Frequency, Vec<Pos>> = HashMap::new();
    // Find and group up antennas by frequency
    initial_state.enumerate().for_each(|(pos, cell)| {
         if let Some(antenna) = cell.antenna {
             if antenna_locations_by_frequency.contains_key(&antenna) {
                 antenna_locations_by_frequency.get_mut(&antenna).unwrap().push(pos)
             } else {
                 antenna_locations_by_frequency.insert(antenna, vec![pos]);
             }
         }
    });

    let mut antinodes_map = AntinodesMap {
        map: Vec::from_iter(initial_state.map.iter().map(|_| 0)),
        width: initial_state.width,
        height: initial_state.height,
    };
    for (_, positions) in antenna_locations_by_frequency.iter() {
        positions.iter().for_each(|pos| {
            positions.iter().for_each(|pos2| {
                if pos == pos2 {
                    return
                }
                let antinode = pos2.apply_offset(&pos.offset_to(&pos2));
                match antinodes_map.update_cell(&antinode, |c| c+1) {
                    Ok(_) => {} // Registered new antinode
                    Err(_) => {} // Antinode is off the map so we don't care
                }
            })
        })
    };

    let unique_antinode_locations = antinodes_map.enumerate().map(|(p,v)| *v)
        .filter(|v| v > &0usize)
        .count();

    unique_antinode_locations
}
fn solve_advanced(initial_state: &AntennaMap) -> usize {
    0
}

// Part 1: 366 too high
pub fn solve_day8() {
    let demo_input = parse_file("demo.txt").expect("demo.txt failed to parse");
    let full_input = parse_file("full.txt").expect("full.txt failed to parse");

    assert_eq!(solve_simple(&demo_input), 14usize);
    println!("Demo 1 passed");
    let full_solution_1 = solve_simple(&full_input);
    assert!(full_solution_1 < 366); // Failed submission 1
    println!("full solution is {}", full_solution_1);
    
    solve_advanced(&demo_input);
    assert_eq!(solve_advanced(&demo_input), 0usize);
    println!("Demo 2 passed");
    println!("full solution is {}", solve_advanced(&full_input));
    // assert_eq!(solve_advanced(&demo.0, &mut demo.1.clone()), 123);
    // println!("Demo 2 passed");
    // println!("full solution is {}", solve_advanced(&full.0, &mut full.1.clone()));
    
}
