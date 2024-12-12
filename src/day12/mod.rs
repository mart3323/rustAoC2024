use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use nom::character::complete::space1;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use crate::utils::read_input_file;

const DAY: &str = "day12";

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
    fn north(&self) -> Option<Pos> {
        if self.y == 0 {
            None
        } else {
            Some(Pos {x: self.x, y: self.y - 1})
        }
    }
    fn south(&self) -> Option<Pos> {
        Some(Pos {x: self.x, y: self.y + 1})
    }
    fn east(&self) -> Option<Pos> {
        Some(Pos {x: self.x + 1, y: self.y})
    }
    fn west(&self) -> Option<Pos> {
        if self.x == 0 {
            None
        } else {
            Some(Pos {x: self.x - 1, y: self.y})
        }
    }
}
impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Pos {
        Pos{x, y}
    }
}

// trait HashMapExt {
//     fn add_insert(&mut self, index: &usize, value: usize);
// }
// impl HashMapExt for HashMap<usize, usize> {
//     fn add_insert(&mut self, key: &usize, value: usize) {
//         let prev: usize = *self.get(&key).unwrap_or(&0);
//         self.insert(*key, prev + value);
//     }
// }

struct Map<T: Clone> {
    map: Vec<Vec<T>>,
    width: usize,
    height: usize,
}
impl<T: Clone> From<Vec<Vec<T>>> for Map<T> {
    fn from(v: Vec<Vec<T>>) -> Map<T> {
        let width = v.get(0).unwrap().len();
        let height = v.len();
        Map{map: v, width, height}
    }
}
impl<T: Clone> Map<T> {
    fn clone_with_default_value<X:Clone>(&self, value:X) -> Map<X> {
        Map{
            width: self.width,
            height: self.height,
            map: (0..self.height).map(|_| (0..self.width).map(|_| value.clone()).collect()).collect()
        }
    }
    fn pos_is_in_range(&self, pos: &Pos) -> bool {
        pos.y < self.height && pos.x < self.width
    }
    fn enumerate(&self) -> Vec<Pos> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| Pos{x,y})).collect()
    }
    fn neighbors(&self, pos: &Pos) -> Vec<Pos> {
        pos.get_neighbors().into_iter().filter(|pos| self.pos_is_in_range(pos)).collect()
    }
    fn get(&self, pos: &Pos) -> &T {
        &self.map[pos.y][pos.x]
    }
    fn set(&mut self, pos: &Pos, val: T) {
        self.map[pos.y][pos.x] = val;
    }
}
type Plantmap = Map<char>;


impl Plantmap {
}

fn to_fence_counts(plantmap: &Plantmap) -> Map<usize> {
    let mut counts = plantmap.clone_with_default_value(9);
    for pos in plantmap.enumerate() {
      let expected_type = plantmap.get(&pos);
      let matches = plantmap.neighbors(&pos).iter()
          .filter(|n| plantmap.get(n) == expected_type).count();
      counts.set(&pos, 4-matches)
    }
    return counts;
}
fn to_side_counts(plantmap: &Plantmap) -> Map<usize> {
    let mut counts = plantmap.clone_with_default_value(9);
    for pos in plantmap.enumerate() {
        let expected_type = plantmap.get(&pos);
        let is_same = |pos: Option<Pos>| -> bool {
            pos.is_some_and(|p| { plantmap.pos_is_in_range(&p) && plantmap.get(&p) == expected_type })
        };
        let is_start_of_fence = |side: Option<Pos>, before: Option<Pos>, before_side:Option<Pos>| -> bool {
            !is_same(side) && (!is_same(before) || is_same(before_side))
        };
        
        let count =
            usize::from(is_start_of_fence(pos.north(), pos.west(), pos.west().and_then(|p| p.north())))+
            usize::from(is_start_of_fence(pos.east(), pos.north(), pos.north().and_then(|p| p.east())))+
            usize::from(is_start_of_fence(pos.south(), pos.east(), pos.east().and_then(|p| p.south())))+
            usize::from(is_start_of_fence(pos.west(), pos.south(), pos.south().and_then(|p| p.west())));
        counts.set(&pos, count)
    }
    return counts;
}

fn to_region_sizes(plantmap: &Plantmap) -> Map<usize> {
    let mut visited = plantmap.clone_with_default_value(false);
    let mut areas = plantmap.clone_with_default_value(0);
    for pos in plantmap.enumerate() {
        if *visited.get(&pos) {
            continue;
        }
        let mut collected = HashSet::new();
        let mut front = HashSet::new();
        front.insert(pos);
        let expected_type = plantmap.get(&pos);
        while front.len() > 0 {
            front.iter().for_each(|pos| visited.set(pos, true));
            let next_front = front.iter()
                .flat_map(|p| plantmap.neighbors(p))
                .filter(|p| !*visited.get(p))
                .filter(|p| !collected.contains(p))
                .filter(|p| plantmap.get(p) == expected_type)
                .collect();
            front.into_iter().for_each(|pos| {collected.insert(pos);});
            front = next_front;
        }
        let area = collected.len();
        collected.iter().for_each(|pos| {
            areas.set(pos, area);
        });
    }
    return areas;
}

fn get_fence_cost(plantmap: &Plantmap) -> usize {
    let counts = to_fence_counts(&plantmap);
    let regions = to_region_sizes(&plantmap);

    let mut total = 0;
    for pos in counts.enumerate() {
        total += counts.get(&pos) * regions.get(&pos);
    }
    return total;
}
fn get_side_cost(plantmap: &Plantmap) -> usize {
    let counts = to_side_counts(&plantmap);
    let regions = to_region_sizes(&plantmap);

    let mut total = 0;
    for pos in counts.enumerate() {
        total += counts.get(&pos) * regions.get(&pos);
    }
    return total;
}

fn parse(demo: &str) -> Plantmap {
    Map::from(demo.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>())
}

#[test]
fn test_part1() {
    let demo = read_input_file(DAY, "demo.txt");
    let plantmap: Plantmap = parse(&demo);
    assert_eq!(get_fence_cost(&plantmap), 1930)
}


#[test]
fn test_part2() {
    let plantmap: Plantmap = parse("AAAA\nBBCD\nBBCC\nEEEC");
    assert_eq!(get_side_cost(&plantmap), 80);
    let plantmap: Plantmap = parse("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE");
    assert_eq!(get_side_cost(&plantmap), 236);
    let plantmap: Plantmap = parse("AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA");
    assert_eq!(get_side_cost(&plantmap), 368);
    let demo = read_input_file(DAY, "demo.txt");
    let plantmap:Plantmap = parse(&demo);
    assert_eq!(get_side_cost(&plantmap), 1206);
}

pub fn part1() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let plantmap: Plantmap = parse(&full);
    return get_fence_cost(&plantmap);
}
pub fn part2() -> usize {
    let full = read_input_file(DAY, "full.txt");
    let plantmap: Plantmap = parse(&full);
    return get_side_cost(&plantmap);
}