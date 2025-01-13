#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Pos2d {
    pub x: isize,
    pub y: isize,
}
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Vec2d {
    dx: isize,
    dy: isize,
}
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Dir2dCardinal {
    North,
    East,
    South,
    West,
}
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Dir2d {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

pub trait Neighbors {
    fn neighbors_cardinal(&self) -> impl IntoIterator<Item=Self>;
    fn neighbors_all(&self) -> impl IntoIterator<Item=Self>;
}
pub trait Vector<Coordinate> {
    fn scale(&self, scale: isize) -> Self;
    fn apply(&self, pos: &Coordinate) -> Coordinate;
}

impl Neighbors for Pos2d {
    fn neighbors_cardinal(&self) -> [Pos2d; 4] {
        [
            Pos2d { y: self.y - 1, ..*self },
            Pos2d { y: self.y + 1, ..*self },
            Pos2d { x: self.x - 1, ..*self },
            Pos2d { x: self.x + 1, ..*self },
        ]
    }
    fn neighbors_all(&self) -> [Pos2d; 8] {
        let mut out: [Pos2d; 8] = [self.clone(),self.clone(),self.clone(),self.clone(),self.clone(),self.clone(),self.clone(),self.clone()];
        let mut i = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx != 0 || dy != 0 {
                    out[i].x += dx;
                    out[i].y += dy;
                    i += 1;
                }
            }
        }
        out
    }
}
impl Vector<Pos2d> for Vec2d {
    fn scale(&self, scale: isize) -> Self {
        Vec2d {dx: self.dx * scale, dy: self.dy * scale}
    }

    fn apply(&self, pos: &Pos2d) -> Pos2d {
        Pos2d {x: self.dx + pos.x, y: self.dy + pos.y}
    }
}
impl From<Dir2dCardinal> for Vec2d {
    fn from(value: Dir2dCardinal) -> Self {
        use Dir2dCardinal::*;
        match value {
            North => Vec2d {dx: 0, dy: -1}, 
            East => Vec2d {dx: 1, dy: 0},
            South => Vec2d {dx: 0, dy: 1},
            West => Vec2d {dx: -1, dy: 0},
        }
    }
}
impl Dir2dCardinal {
    fn turn_right(&self) -> Self {
        use Dir2dCardinal::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(&self) -> Self {
        use Dir2dCardinal::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn reverse(&self) -> Self {
        use Dir2dCardinal::*;
        match self {
            North => South,
            South => North,
            West=> East,
            East => West,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pos = Pos2d { x: 3, y: 4 };
        let dir = Dir2dCardinal::North.turn_right().reverse();
        let pos = Vec2d::from(dir).scale(-1).scale(2).apply(&pos);
        
        assert_eq!(pos, Pos2d { x: 5, y: 4 });
    }
}
