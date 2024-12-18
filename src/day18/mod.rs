use std::collections::HashSet;
use crate::utils::read_input_file;

const DAY: &str = "day18";

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
    fn filled(width: usize, height: usize, v: &T) -> Self {
        Self {
            data: Vec::from_iter((0..width*height).map(|_| *v)),
            width
        }
    }
    fn modify_all(&mut self, map: fn(v: &T) -> T) {
        self.data = self.data.iter().map(map).collect()
    }
}

/// Given the input which is a list of coordinate pairs, one on each line
/// and the size of the map
/// Output a map where each cell contains the time when that cell becomes corrupted (or usize::MAX if the cell will remain OK)
fn parse_input(str: &str, width: usize, height: usize) -> Map<usize> {
    let mut out: Map<usize> = Map::filled(width, height, &usize::MAX);
    str.lines().enumerate().for_each(|(i, line)| {
        let mut split = line.split(",");
        let x = split.next().expect("Each line should have coordinates");
        let y = split.next().expect("Each line should have a second coordinate");
        let x: usize = str::parse(x).expect("Coordinates should be valid integers");
        let y: usize = str::parse(y).expect("Coordinates should be valid integers");
        let pos = Pos { x: x as i32, y: y as i32 };
        let current = out.get_at(&pos);
        if i < current {
            out.set_at(&pos, i);
        }
    });
    return out;
}
#[test]
fn test_parse_input() {
    let map = parse_input("1,1\n1,3\n2,2",3, 4);
    assert_eq!(map.data, vec!(
        usize::MAX, usize::MAX, usize::MAX, 
        usize::MAX, 0, usize::MAX,
        usize::MAX, usize::MAX, 2,
        usize::MAX, 1, usize::MAX,
    ));
}
fn part1_flood_fill(corruption_map: &Map<usize>, start: &Pos) -> Map<usize> {
    let mut distance_map: Map<usize> = Map::filled(corruption_map.width, corruption_map.height(), &usize::MAX);
    distance_map.set_at(start, 0);
    let mut distance = 0;
    let mut position = HashSet::new();
    position.insert(*start);
    while position.len() > 0 {
        distance += 1;
        let mut new_position = HashSet::new();
        for &pos in &position {
            for neighbor in pos.neighbors() {
                if distance_map.contains_pos(&neighbor) {
                    if distance < distance_map.get_at(&neighbor) && distance < corruption_map.get_at(&neighbor){
                        distance_map.set_at(&neighbor, distance);
                        new_position.insert(neighbor);
                    }
                }
            }
        }
        position = new_position;
    }
    distance_map
}
fn part2_flood_fill(corruption_map: &Map<usize>, start: &Pos, end: &Pos) -> usize {
    let mut visibility_map: Map<bool> = Map::filled(corruption_map.width, corruption_map.height(), &false);
    
    let mut front = vec!(*start);
    let mut time = *corruption_map.data.iter().filter(|&&v| v < usize::MAX).max().unwrap_or(&0);
    loop {
        let (stuck, free): (Vec<Pos>, Vec<Pos>) = front.iter().partition(|p| corruption_map.get_at(p) < time);
        // No longer able to move because of corruption, step back in time
        if free.is_empty() {
            if time == 0 {
                for y in 0..visibility_map.height() {
                    for x in 0..visibility_map.width {
                        print!("{}", if visibility_map.get_at(&Pos{x: x as i32, y: y as i32}) { "." } else { "#" })
                    }
                    println!();
                }
                panic!("This should never happen")
            }
            time -= 1;
        } else {
            let mut new_front = stuck;
            for pos in free {
                for n in pos.neighbors() {
                    // In range
                    if visibility_map.contains_pos(&n) {
                        // Not travelled yet
                        if !visibility_map.get_at(&n) {
                            new_front.push(n);
                            visibility_map.set_at(&n, true);
                            if &n == end {
                                return time;
                            }
                        }
                    }
                }
            }
            front = new_front
        }
    }
    return time;
}
#[test]
fn test_part1() {
    let mut corruption_map = parse_input(&read_input_file(DAY, "demo.txt"), 7, 7);
    corruption_map.modify_all(|&time| if time < 12 { 0 } else { usize::MAX });
    
    let distance_map = part1_flood_fill(&corruption_map, &Pos { x: 0, y: 0 });
    assert_eq!(distance_map.get_at(&Pos{x: 6, y:6}), 22)
}

pub fn part1() -> usize {
    let mut corruption_map = parse_input(&read_input_file(DAY, "full.txt"), 71, 71);
    corruption_map.modify_all(|&time| if time < 1024 { 0 } else { usize::MAX });

    let distance_map = part1_flood_fill(&corruption_map, &Pos { x: 0, y: 0 });

    return distance_map.get_at(&Pos{x: 70, y: 70})
}

#[test]
fn test_part2() {
    let input = read_input_file(DAY, "demo.txt");
    let corruption_map = parse_input(&input, 7, 7);

    let start = &Pos { x: 0, y: 0 };
    let end = &Pos { x: 6, y: 6 };
    let max_time = part2_flood_fill(&corruption_map, start, end);
    let max_time_2 = part2_flood_fill(&corruption_map, end, start);
    assert_eq!(max_time, max_time_2);
    assert_eq!(input.lines().nth(max_time), Some("6,1"))
}

pub fn part2() -> String {
    let input = read_input_file(DAY, "full.txt");
    let corruption_map = parse_input(&input, 71, 71);

    let start = &Pos { x: 0, y: 0 };
    let end = &Pos { x: 70, y: 70 };
    let max_escapable_time = part2_flood_fill(&corruption_map, start, end);
    let max_escapable_time_2 = part2_flood_fill(&corruption_map, end, start);
    assert_eq!(max_escapable_time, max_escapable_time_2);

    input.lines().nth(max_escapable_time).expect("Resulting time to be in range").to_string()
}