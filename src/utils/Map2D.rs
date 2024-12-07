use std::str::FromStr;

struct Pos2D<I: Into<usize>> {
    x: I,
    y: I,
}
impl<I> From<(I,I)> for Pos2D<I> where I : Into<usize> {
    fn from(value: (I, I)) -> Self {
        Pos2D {
            x: value.0,
            y: value.1,
        }
    }
}


/// a 2d map of elements
/// Coordinates are X from left to right and Y from top to bottom
///
///  ```
///   0 2 4 6 8  x
///  0.........
///   .#.......  {x:1, y:1}
///  2......#..  {x:6, y:2}
///   .........
///  4.........
///   ..#......  {x:2, y:5}
///  6.........
/// 
///  y
///  ```
struct Map2D<T> {
    /// The raw map data in a 1d array for quick lookup
    map: Vec<T>,
    /// The width of the map, necessary to convert coordinates to indices
    width: usize,
    /// The height of the map, just for quick reference
    height: usize,
}
impl<T> Default for Map2D<T> {
    fn default() -> Self {
        Map2D {
            map: Vec::with_capacity(0),
            width: 0,
            height: 0,
        }
    }
}
impl<T> FromStr for Map2D<T> where T : FromStr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s.lines().next().ok_or(());
        let width = first_line?.trim().len();
        let height = s.lines().count();
        let map = s.lines().map(|l| l.trim()).collect();
        
        Ok(Map2D {map, width, height})
    }
}

impl<T> Map2D<T> {
    fn pos_to_index<I: Into<usize>>(&self, pos: Pos2D<I>) -> usize {
        let x = usize::from(pos.x);
        let y = usize::from(pos.y);
        (self.width * x) + y
    }
    fn index_to_pos<I: Into<usize>, O: From<usize>>(&self, index: I) -> Pos2D<O> {
        Pos2D {
            x: O::from(usize::from(&index) % self.width),
            y: O::from(usize::from(&index) / self.width),
        }
    }
    fn get<I: Into<usize>>(&self, pos: Pos2D<I>) -> T{
        self.map.get(self.pos_to_index(pos))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_from_string() {
        let map: Map2D<char> = Map2D::from_str("abcde\nfghij\nklmno\n").expect("Should parse correctly");
        assert_eq!(map.map, vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o']);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);
    }
    
    #[test]
    fn can_get_item_by_coord() {
        let map = Map2D::from_str("abcde\nfghij").expect("Should parse correctly");

        assert_eq!(map.get(Pos2D { x: 3usize, y: 0usize }), 'd');
        assert_eq!(map.get(Pos2D::from((1usize,1usize))), 'g');
    }
}