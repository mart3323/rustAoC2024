use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::North => String::from("^"),
            Direction::East => String::from(">"),
            Direction::South => String::from("v"),
            Direction::West => String::from("<"),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Box,
    Wall,
    Robot,
}
impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Empty),
            'O' => Ok(Tile::Box),
            '#' => Ok(Tile::Wall),
            '@' => Ok(Tile::Robot),
            _ => Err(()),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Empty => String::from("."),
            Tile::Box => String::from("O"),
            Tile::Wall => String::from("#"),
            Tile::Robot => String::from("@"),
        })
    }
}

pub type Instructions = Vec<Direction>;

#[derive(Debug, Eq, PartialEq)]
pub struct Warehouse {
    pub map: Vec<Tile>,
    pub width: usize,
}

impl Warehouse {
    pub fn from_2d_vec(input: Vec<Vec<Tile>>) -> Warehouse{
        if let Some(first) = input.iter().next() {
            Warehouse {
                width: first.len(),
                map: input.into_iter().flatten().collect(),
            }
        } else {
            Warehouse { map: Vec::new(), width: 0 }
        }
    }
    pub fn get_height(&self) -> usize {
        return self.map.len() / self.width;
    }

    pub fn get_at(&self, point: &Point) -> Option<&Tile> {
        let Point { x, y } = point;
        if *x < 0 { return None }
        if *x >= self.width as isize { return None }
        if *y < 0 { return None }
        if *y >= self.get_height() as isize { return None }
        return Some(&self.map[*y as usize * self.width + *x as usize]);
    }

    fn set_at(&mut self, point: &Point, tile: Tile) {
        let Point { x, y } = point;
        if *x < 0 { return }
        if *x >= self.width as isize { return }
        if *y < 0 { return }
        if *y >= self.get_height() as isize { return }
    }
}