use crate::simulator::{DirpadKey, Layout, NumpadKey, Robot};
use crate::visualiser::PrintRecursive;
use itertools::Itertools;
use std::collections::HashMap;

pub mod simulator;
pub mod visualiser;

fn shortest(a: String, b: String) -> String {
    if a.chars().count() < b.chars().count() { a } else { b }
}

struct DirCache(HashMap<(DirpadKey, DirpadKey), String>);
struct DirCacheOptimized(HashMap<(DirpadKey, DirpadKey), usize>);

impl Default for DirCache {
    fn default() -> Self {
        use DirpadKey::*;
        let options = [A, Up, Down, Left, Right];
        let mut out = Self(HashMap::new());
        for start in options {
            for end in options {
                out.0.insert((start, end), match end {
                    A => String::from("A"),
                    Up => String::from("^"),
                    Right => String::from(">"),
                    Down => String::from("v"),
                    Left => String::from("<"),
                });
            }
        };
        return out;
    }
}
impl Default for DirCacheOptimized {
    fn default() -> Self {
        use DirpadKey::*;
        let options = [A, Up, Down, Left, Right];
        let mut out = Self(HashMap::new());
        for start in options {
            for end in options {
                out.0.insert((start, end), match end {
                    A => 1,
                    Up => 1,
                    Right => 1,
                    Down => 1,
                    Left => 1,
                });
            }
        };
        return out;
    }
}
impl DirCache {
    // Path to go from X to Y
    fn leg(&self, from: DirpadKey, to: DirpadKey) -> String {
        return self.0.get(&(from, to)).unwrap().to_owned()
    }
    // Path to go from A to _ times press _ time and return
    fn sequence1(&self, to: DirpadKey, press_count: usize) -> String {
        self.leg(DirpadKey::A, to) +
            &self.leg(to, to).repeat(press_count-1) +
            &self.leg(to, DirpadKey::A)
    }
    // Path to go from A to _ times press _ and _ times press _ and return
    fn sequence2(&self, to_1: DirpadKey, press_count_1: usize, to_2: DirpadKey, press_count_2: usize) -> String {
        self.leg(DirpadKey::A, to_1)
            + &self.leg(to_1, to_1).repeat(press_count_1 - 1)
            + &self.leg(to_1, to_2)
            + &self.leg(to_2, to_2).repeat(press_count_2 - 1)
            + &self.leg(to_2, DirpadKey::A)
    }
    fn sequence2_unordered(&self, to_1: DirpadKey, press_count_1: usize, to_2: DirpadKey, press_count_2: usize) -> String {
        shortest(
            self.sequence2(to_1, press_count_1, to_2, press_count_2),
            self.sequence2(to_2,press_count_2, to_1, press_count_1)
        )
    }
    fn next(&self) -> Self {
        use DirpadKey::*;
        let options = [A,Up,Down,Left,Right];
        let mut out = DirCache(HashMap::new());
        for start in options {
            for end in options {
                out.0.insert((start,end), match (start, end) {
                    // Left
                    (Down, Left)|
                    (Right,Down)|
                    (A,Up) => self.sequence1(Left, 1),
                    // Left Left
                    (Right, Left) => self.sequence1(Left, 2),
                    // Down Left (ordered)
                    (Up,Left) => self.sequence2(Down, 1, Left, 1),
                    // Down Left (unordered)
                    (A, Down) => self.sequence2_unordered(Down, 1, Left, 1),
                    // Down Left Left
                    (A, Left) => self.sequence2(Down, 1, Left, 2),
                    // Up
                    (Down, Up)|(Right,A) => self.sequence1(Up, 1),
                    // Right
                    (Left,Down)|
                    (Down,Right)|
                    (Up,A) => self.sequence1(Right, 1),
                    // Right Right
                    (Left, Right) => self.sequence1(Right, 2),
                    // Right Up (ordered)
                    (Left, Up) => self.sequence2(Right, 1, Up, 1),
                    // Right Up (unordered)
                    (Down, A) => self.sequence2_unordered(Right, 1, Up, 1),
                    // Right Right Up
                    (Left, A) => self.sequence2(Right, 2, Up, 1),
                    // Down
                    (Up,Down)|
                    (A,Right) => self.sequence1(Down, 1),
                    // Right down
                    (Up, Right) => self.sequence2_unordered(Right, 1, Down, 1),
                    // Left up
                    (Right, Up) => self.sequence2_unordered(Left, 1, Up, 1),
                    (a, b) if a == b => String::from("A"),
                    (x,y) => panic!("Match was not exhaustive, failed to cover {:?} to {:?}", &x, &y)
                });

            }
        };
        return out;
    }

    fn path_for_code(&self, code: &Vec<NumpadKey>) -> String {
        use DirpadKey::*;

        let mut out = String::new();
        let mut pos = NumpadKey::A;
        for &key in code {
            fn pos_of(k: &NumpadKey) -> (i8, i8) {
                use NumpadKey::*;
                match k {
                    Seven => (0,0), Eight => (1,0), Nine  => (2,0),
                    Four  => (0,1), Five  => (1,1), Six   => (2,1),
                    One   => (0,2), Two   => (1,2), Three => (2,2),
                                     Zero => (1,3),     A => (2,3),
                }
            }
            let start = pos_of(&pos);
            let end = pos_of(&key);
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            // Exception, do not cut the corner
            if pos == NumpadKey::Zero && dx < 0 || pos == NumpadKey::A && dx == -2 {
                assert!(dy < 0);
                out += &self.sequence2(Up, -dy as usize, Left, -dx as usize)
            } else if key == NumpadKey::Zero && dx > 0 || key == NumpadKey::A && dx == 2 {
                assert!(dy > 0);
                out += &self.sequence2(Right, dx as usize, Down, dy as usize)
            } else if dx < 0 && dy < 0 {
                out += &self.sequence2_unordered(Up, -dy as usize, Left, -dx as usize)
            } else if 0 < dx && dy < 0 {
                out += &self.sequence2_unordered(Up, -dy as usize, Right, dx as usize)
            } else if 0 < dx && 0 < dy {
                out += &self.sequence2_unordered(Down, dy as usize, Right, dx as usize)
            } else if dx < 0 && 0 < dy {
                out += &self.sequence2_unordered(Down, dy as usize, Left, -dx as usize)
            } else if dx == 0 {
                if dy > 0 {
                    out += &self.sequence1(Down, dy as usize);
                } else {
                    out += &self.sequence1(Up, -dy as usize);
                }
            } else if dy == 0 {
                if dx > 0 {
                    out += &self.sequence1(Right, dx as usize);
                } else {
                    out += &self.sequence1(Left, -dx as usize);
                }
            }
            pos = key;
        }
        return out;
    }
}
impl DirCacheOptimized {
    // Path to go from X to Y
    fn leg(&self, from: DirpadKey, to: DirpadKey) -> usize {
        return self.0.get(&(from, to)).unwrap().to_owned();
    }
    // Path to go from A to _ times press _ time and return
    fn sequence1(&self, to: DirpadKey, press_count: usize) -> usize {
        self.leg(DirpadKey::A, to) +
            &self.leg(to, to) * (press_count - 1) +
            &self.leg(to, DirpadKey::A)
    }
    // Path to go from A to _ times press _ and _ times press _ and return
    fn sequence2(&self, to_1: DirpadKey, press_count_1: usize, to_2: DirpadKey, press_count_2: usize) -> usize {
        self.leg(DirpadKey::A, to_1)
            + (&self.leg(to_1, to_1) * (press_count_1 - 1))
            + &self.leg(to_1, to_2)
            + (&self.leg(to_2, to_2) * (press_count_2 - 1))
            + &self.leg(to_2, DirpadKey::A)
    }
    fn sequence2_unordered(&self, to_1: DirpadKey, press_count_1: usize, to_2: DirpadKey, press_count_2: usize) -> usize {
        self.sequence2(to_1, press_count_1, to_2, press_count_2).min(
            self.sequence2(to_2,press_count_2, to_1, press_count_1)
        )
    }
    fn next(&self) -> Self {
        use DirpadKey::*;
        let options = [A,Up,Down,Left,Right];
        let mut out = DirCacheOptimized(HashMap::new());
        for start in options {
            for end in options {
                out.0.insert((start,end), match (start, end) {
                    // Left
                    (Down, Left)|
                    (Right,Down)|
                    (A,Up) => self.sequence1(Left, 1),
                    // Left Left
                    (Right, Left) => self.sequence1(Left, 2),
                    // Down Left (ordered)
                    (Up,Left) => self.sequence2(Down, 1, Left, 1),
                    // Down Left (unordered)
                    (A, Down) => self.sequence2_unordered(Down, 1, Left, 1),
                    // Down Left Left
                    (A, Left) => self.sequence2(Down, 1, Left, 2),
                    // Up
                    (Down, Up)|(Right,A) => self.sequence1(Up, 1),
                    // Right
                    (Left,Down)|
                    (Down,Right)|
                    (Up,A) => self.sequence1(Right, 1),
                    // Right Right
                    (Left, Right) => self.sequence1(Right, 2),
                    // Right Up (ordered)
                    (Left, Up) => self.sequence2(Right, 1, Up, 1),
                    // Right Up (unordered)
                    (Down, A) => self.sequence2_unordered(Right, 1, Up, 1),
                    // Right Right Up
                    (Left, A) => self.sequence2(Right, 2, Up, 1),
                    // Down
                    (Up,Down)|
                    (A,Right) => self.sequence1(Down, 1),
                    // Right down
                    (Up, Right) => self.sequence2_unordered(Right, 1, Down, 1),
                    // Left up
                    (Right, Up) => self.sequence2_unordered(Left, 1, Up, 1),
                    (a, b) if a == b => 1,
                    (x,y) => panic!("Match was not exhaustive, failed to cover {:?} to {:?}", &x, &y)
                });

            }
        };
        return out;
    }

    fn path_for_code(&self, code: &Vec<NumpadKey>) -> usize {
        use DirpadKey::*;

        let mut out = 0;
        let mut pos = NumpadKey::A;
        for &key in code {
            fn pos_of(k: &NumpadKey) -> (i8, i8) {
                use NumpadKey::*;
                match k {
                    Seven => (0,0), Eight => (1,0), Nine  => (2,0),
                    Four  => (0,1), Five  => (1,1), Six   => (2,1),
                    One   => (0,2), Two   => (1,2), Three => (2,2),
                                     Zero => (1,3),     A => (2,3),
                }
            }
            let start = pos_of(&pos);
            let end = pos_of(&key);
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            // Exception, do not cut the corner
            if pos == NumpadKey::Zero && dx < 0 || pos == NumpadKey::A && dx == -2 {
                assert!(dy < 0);
                out += &self.sequence2(Up, -dy as usize, Left, -dx as usize)
            } else if key == NumpadKey::Zero && dx > 0 || key == NumpadKey::A && dx == 2 {
                assert!(dy > 0);
                out += &self.sequence2(Right, dx as usize, Down, dy as usize)
            } else if dx < 0 && dy < 0 {
                out += &self.sequence2_unordered(Up, -dy as usize, Left, -dx as usize)
            } else if 0 < dx && dy < 0 {
                out += &self.sequence2_unordered(Up, -dy as usize, Right, dx as usize)
            } else if 0 < dx && 0 < dy {
                out += &self.sequence2_unordered(Down, dy as usize, Right, dx as usize)
            } else if dx < 0 && 0 < dy {
                out += &self.sequence2_unordered(Down, dy as usize, Left, -dx as usize)
            } else if dx == 0 {
                if dy > 0 {
                    out += &self.sequence1(Down, dy as usize);
                } else {
                    out += &self.sequence1(Up, -dy as usize);
                }
            } else if dy == 0 {
                if dx > 0 {
                    out += &self.sequence1(Right, dx as usize);
                } else {
                    out += &self.sequence1(Left, -dx as usize);
                }
            }
            pos = key;
        }
        return out;
    }
}

// Ours: <vA<AA>>^AvAA^<A>A v<<A>>^AvA^A v<<A>>^AA<vA>A^A<A>A v<<A>A^>AAA<Av>A^A
// Ref:  <vA<AA>>^AvAA<^A>A <v<A>>^AvA^A <vA>^A<v<A>^A>AAvA^A <v<A>A>^AAAvA<^A>A

const DEMO_INPUT: [&str; 5] = ["029A","980A","179A","456A","379A"];
const FULL_INPUT: [&str; 5] = ["341A","803A","149A","683A","208A"];

fn parse_code(code: &str) -> Option<Vec<NumpadKey>> {
    code.chars().map(|c| match c {
        'A' => Some(NumpadKey::A),
        '0' => Some(NumpadKey::Zero),
        '1' => Some(NumpadKey::One),
        '2' => Some(NumpadKey::Two),
        '3' => Some(NumpadKey::Three),
        '4' => Some(NumpadKey::Four),
        '5' => Some(NumpadKey::Five),
        '6' => Some(NumpadKey::Six),
        '7' => Some(NumpadKey::Seven),
        '8' => Some(NumpadKey::Eight),
        '9' => Some(NumpadKey::Nine),
        _ => None
    }).collect()
}

fn solve_part_1(depth: usize, codes: Vec<&str>) -> usize {
    let mut solver = DirCache::default();
    for _ in 1..depth {
        solver = solver.next();
    };
    let mut total_complexity = 0;
    for code in codes {
        let value: usize = code.trim_start_matches("0").trim_end_matches("A").parse().expect("Code to be valid");
        let code = parse_code(&code).expect("Code to be valid");
        let length: usize = solver.path_for_code(&code).chars().count();
        total_complexity += value * length;
    };
    return total_complexity
}

#[test]
fn test_part_1() {
    let expect = [
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
    ];
    let solver_3 = DirCache::default().next().next();
    for i in 0..5 {
        assert_eq!(solver_3.path_for_code(&parse_code(DEMO_INPUT[i]).expect("Code to parse")).chars().count(), expect[i].chars().count());
    }
    let solver = DirCache::default();
    let path = solver.next().path_for_code(&parse_code("980A").expect("Code to be valid"));
    println!("980A: {:?}", path);
    assert_eq!(solve_part_1(3, DEMO_INPUT.to_vec()), 126384);
    // Incorrect submission
    let result = solve_part_1(3, FULL_INPUT.to_vec());
    println!("full compl: {}", result);
    assert!(156544 < result);
}
#[test]
fn test_solvers_are_equivalent() {
    let solver3 = DirCache::default().next().next();
    let osolver3 = DirCacheOptimized::default().next().next();
    for i in DEMO_INPUT {
        let code = parse_code(i).expect("Code to be valid");
        assert_eq!(
            solver3.path_for_code(&code).chars().count(),
            osolver3.path_for_code(&code)
        )
    }
}
pub fn part1() -> usize {
    let mut total_complexity = 0;
    let solver_3 = DirCache::default().next().next();
    for code in FULL_INPUT {
        let value: usize = code.trim_start_matches("0").trim_end_matches("A").parse().expect("Code to be valid");
        let code = parse_code(code).expect("Code to be valid");
        let length: usize = solver_3.path_for_code(&code).chars().count();
        total_complexity += value * length;
    }
    return total_complexity
}
pub fn part2() -> usize {
    let mut total_complexity = 0;
    let mut solver = DirCacheOptimized::default();
    for i in 0..25 {
        solver = solver.next();
    }
    let solver_26 = solver;
    for code in FULL_INPUT {
        let value: usize = code.trim_start_matches("0").trim_end_matches("A").parse().expect("Code to be valid");
        let code = parse_code(code).expect("Code to be valid");
        let length: usize = solver_26.path_for_code(&code);
        total_complexity += value * length;
    }
    return total_complexity
}

fn notes() {
    // Note: If the previous and current numpad are both on the A key, the cheapest path to press X key is known
    // Note: The sequence to press a key on the current numpad always ends with all previous numpads returning to the A key
    //       because that is the only key that propagates through the chain
    //         HOWEVER: for any one press on the current numpad, the previous numpad may have to make multiple presses
    //                  without resetting in the meantime
    //                  For example, consider the following situation where we want to return to numpad A

    // TODO: Theory
    //       Assumption: A robot will never need to move in two opposite directions between A presses
    //                   because if it does, it would be more optimal to skip both the movements
    //       Therefore: we only need the costs for movements that don't contain opposites (DDD, DDR, but not DRU)
    //       Plan:
    //         For each layer of keypad, starting from the first, precalculate the cost of pressing each of the following combinations
    //         L, R, U, D, UR, DR, DL, UL (for diagonals, test both orders)
    //           When doing DL to ←, only accept order DL, otherwise use DL or LD, whichever is shorter
    //           When doing UR from ←, only accept oder RU, otherwise use UR or RU, whichever is shorter
    //         Cache these results, For the next layer's calculations, simplify the path to two directions, use the cached value, and add back +1 for each omitted duplicate
    //
    // TODO: Test
    //             L  R  U  D  UR DR DL UL
    //  Numpad 0:  1  1  1  1  2  2  2  2    (path + return path + len + duplicates)
    //  Numpad 1:  7  3  3  5  6  7
    {
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │A│  │ │A││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │ │ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //  After some moves
        //                       ┌─┬─┬─┐ Movement cost: nr of spaces + 6 if contains left + 4 if contains down or up right + 2 if contains right or up
        //                       │ │ │ │ (with return trip)
        //                       ├─┼─┼─┤ Movement cost 2: nr of spaces + 6+4 if contains left and/or down + 6+2 if contains down or up + 4+2 if contains right
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤ Cost   A    LD/LU          RD           RU        U         R
        //    │ │A│  │ │ │  │ │A││ │ │9│        A    nA*6+nLD+nRU   nA*
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │↓│ ││ │ │ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │↓│ ││ │ │→│  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //  Note: Here numpad 2 will not return to A, because numpad 3 is still in the middle of its movement sequence
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │←│ │ ││ │↓│ ││ │ │→│  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │←│ │ ││←│ │ ││ │ │→│  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │ │
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │←│ │ ││←│ │ ││ │↓│ │  │ │A│
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        // .                     ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │↓│ ││←│ │ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │→││ │↓│ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │→││ │ │→││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │→││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │↑│ │  │ │ │  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │→││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │↑│ │  │ │A│  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │A│  │ │ ││ │ │9│
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │↓│ │  │ │ │
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
        //                       ┌─┬─┬─┐
        //                       │ │ │ │
        //                       ├─┼─┼─┤
        //                       │ │ │ │
        //    ┌─┬─┐  ┌─┬─┐  ┌─┬─┐├─┼─┼─┤
        //    │ │A│  │ │A│  │ │ ││ │ │ │
        //  ┌─┼─┼─┤┌─┼─┼─┤┌─┼─┼─┤└─┼─┼─┤
        //  │ │ │ ││ │ │ ││ │↓│ │  │ │A│
        //  └─┴─┴─┘└─┴─┴─┘└─┴─┴─┘  └─┴─┘
    }
}