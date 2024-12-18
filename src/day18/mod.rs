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
        0 < pos.x && pos.x < (self.width as i32) && 0 < pos.y && pos.y < (self.height() as i32)
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

#[test]
fn test_part1() {
}

pub fn part1() -> usize {
    0
}

#[test]
fn test_part2() {
}

pub fn part2() -> usize {
    0
}