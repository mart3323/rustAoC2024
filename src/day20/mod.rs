use std::collections::HashSet;
use std::fmt::Display;
use crate::utils::read_input_file;

const DAY: &str = "day20";

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn neighbors(&self) -> [Pos; 4] {
        [
            Self { x: self.x, y: self.y - 1 },
            Self { x: self.x + 1, y: self.y },
            Self { x: self.x, y: self.y + 1 },
            Self { x: self.x - 1, y: self.y },
        ]
    }
    fn taxicab_distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
        0 <= pos.x && pos.x < (self.width as i32) && 0 <= pos.y && pos.y < (self.height() as i32)
    }
    fn get_at(&self, pos: &Pos) -> T {
        self.data[pos.y as usize * self.width + pos.x as usize]
    }
    fn set_at(&mut self, pos: &Pos, val: T) {
        self.data[pos.y as usize * self.width + pos.x as usize] = val;
    }
    fn update_at(&mut self, pos: &Pos, update: fn(prev:T) -> T) {
        let index = pos.y as usize * self.width + pos.x as usize;
        self.data[index] = update(self.data[index]);
    }
    fn filled(width: usize, height: usize, v: &T) -> Self {
        Self {
            data: Vec::from_iter((0..width*height).map(|_| *v)),
            width
        }
    }
    fn modify_all(&mut self, map: fn(v: &T) -> T) {
        self.data = self.data.iter().map(map).collect()
    }

    fn print<P: Sized+Display>(&self, t: fn(t:T) -> P) {
        for y in 0..self.height() {
            for x in 0..self.width {
                print!("{}", t(self.get_at(&Pos{x: x as i32, y: y as i32})))
            }
            println!();
        }
    }
    fn print_wide<P: Sized+Display>(&self, t: fn(t:T) -> P) {
        for y in 0..self.height() {
            for x in 0..self.width {
                print!("{:4}", t(self.get_at(&Pos{x: x as i32, y: y as i32})))
            }
            println!();
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty
}

struct Racetrack {
    map: Map<Tile>,
    start: Pos,
    end: Pos,
}

fn parse_input(str: &str) -> Racetrack {

    let mut width = 0;
    let mut start = None;
    let mut end = None;

    let mut data = Vec::new();
    for (y, line) in str.lines().enumerate() {
        width = line.len();
        data.push(line);
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => { start = Some(Pos {x: x as i32, y: y as i32})}
                'E' => { end = Some(Pos {x: x as i32, y: y as i32})}
                _ => {}
            }
        }
    }
    return Racetrack {
        start: start.expect("Expected to find a start tile"),
        end: end.expect("Expected to find an end tile"),
        map: Map {
            data: str.lines().map(|line| line.chars().map(|c| match c {
                '#' => Tile::Wall,
                _ => Tile::Empty
            })).flatten().collect(),
            width
        }
    }
}

/// Get the time-saved for every possible shortcut on the track
fn get_shortcuts(racetrack: &Racetrack) -> Vec<usize> {
    let width = racetrack.map.width;
    let height = racetrack.map.height();
    let mut normally_reachable_in = Map::filled(width, height, &None);
    let mut shortcuts = Vec::new();

    let mut time = 0;
    let mut pos = racetrack.start;
    while pos != racetrack.end {
        // Update normally-reachable-in
        normally_reachable_in.set_at(&pos, Some(time));
        // Update pos
        pos = pos.neighbors().into_iter().find(|p| {
            normally_reachable_in.get_at(&p).is_none() && racetrack.map.get_at(&p) == Tile::Empty
        }).expect("Map should have no dead ends or loops and eventually reach the end");
        // update time
        time += 1;

        for n1 in pos.neighbors() {
            for n2 in n1.neighbors() {
                if n2.taxicab_distance(&pos) == 2 && racetrack.map.contains_pos(&n2) {
                    if let Some(cut_time) = normally_reachable_in.get_at(&n2) {
                        let profit = time - (cut_time + 2);
                        if 0 < profit {
                            shortcuts.push(profit);
                        }
                    }
                }
            }
        }
    }

    return shortcuts;
}
#[test]
fn test_part_1() {
    let racetrack = parse_input(&read_input_file(DAY, "demo.txt"));


    let mut shortcuts: Vec<usize> = get_shortcuts(&racetrack);
    shortcuts.sort();

    assert_eq!(shortcuts, vec!(2,2,2,2,2,2,2,2,2,2,2,2,2,2,4,4,4,4,4,4,4,4,4,4,4,4,4,4,6,6,8,8,8,8,10,10,12,12,12,20,36,38,40,64));
}

pub fn part1() -> usize {
    let racetrack = parse_input(&read_input_file(DAY, "full.txt"));
    let mut shortcuts: Vec<usize> = get_shortcuts(&racetrack);
    let count_over_100 = shortcuts.into_iter().filter(|&saved| 100 <= saved).count();
    
    return count_over_100;
}

pub fn part2() -> usize {
    todo!()
}