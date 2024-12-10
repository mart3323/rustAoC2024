use crate::utils::read_input_file;
use nom::AsChar;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

#[derive(Debug, Eq, PartialEq)]
enum SearchState {
    None,
    X,
    XM,
    XMA,
    SAM,
    SA,
    S,
}
impl SearchState {
    fn expect_next(self) -> (u8, SearchState, bool) {

        match self {
            SearchState::None => panic!("None should be special-cased"),
            SearchState::X => (b'M', SearchState::XM, false),
            SearchState::XM => (b'A', SearchState::XMA, false),
            SearchState::XMA => (b'S', SearchState::S, true),
            SearchState::SAM => (b'X', SearchState::X, true),
            SearchState::SA => (b'M', SearchState::SAM, false),
            SearchState::S => (b'A', SearchState::SA, false),
        }
    }
    fn process_char(self, char: u8) -> (SearchState, bool) {
        use SearchState::*;

        // Default case, see if it's the next expected char
        if self != None {
            let (expected, next, score) = self.expect_next();
            if char == expected {
                return (next, score);
            };
        }
        // If it's not, or if we haven't read any matching chars yet, read it as if it were the first
        match char {
            b'X' => (X, false),
            b'S' => (S, false),
            _ => (None, false),
        }
    }
}
fn process_chars<T>(chars: T, total: &Arc<AtomicUsize>)
    where T : Iterator<Item = char>
{
    let mut state = SearchState::None;
    let mut score;

    for c in chars {
        (state, score) = state.process_char(c as u8);
        if score {
            total.fetch_add(1, Ordering::Relaxed);
        }
    };
}

fn solve_simple(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count(); // Blank line at the end;
    let input = input.replace("\r", "").replace("\n", "");

    let found: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    thread::scope(|scope| {
        for row in 0..height {
            let left_to_right = input.chars().skip(width * row).take(width);
            scope.spawn(|| process_chars(left_to_right, &found));
            if row != 0 {
                let left_to_bottom = input.chars().skip(width * row).step_by(width + 1);
                let right_to_bottom = input.chars().skip(width * row + width - 1).step_by(width - 1);
                scope.spawn(|| process_chars(left_to_bottom, &found));
                scope.spawn(|| process_chars(right_to_bottom, &found));
            }
        }
        for col in 0..width {
            let top_to_bottom = input.chars().skip(col).step_by(width);
            scope.spawn(|| process_chars(top_to_bottom, &found));

            let top_to_right = input.chars().skip(col).step_by(width + 1).take(width - col);
            let top_to_left = input.chars().skip(col).step_by(width - 1).take(col + 1);
            scope.spawn(|| process_chars(top_to_left, &found));
            scope.spawn(|| process_chars(top_to_right, &found));
        }
    });
    return found.load(Relaxed);
}
fn solve_advanced(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count(); // Blank line at the end;
    let input = input.replace("\r", "").replace("\n", "");

    let char_at = |row: usize, col: usize| {
        input.as_bytes().get(row * width + col).unwrap().as_char()
    };
    let mut found = 0;
    for row in 1..height-1 {
        for col in 1..width-1 {
            let c = char_at(row, col);
            if c != 'A' {
                continue;
            }
            let (a,b,c,d) = (char_at(row-1, col-1),
                             char_at(row-1, col+1),
                             char_at(row+1, col+1),
                             char_at(row+1, col-1));
            const M: char = 'M';
            const S: char = 'S';
            match (a,b,c,d) {
                (M,M,S,S) | (M,S,S,M) | (S,S,M,M) | (S,M,M,S) => {
                    found += 1;
                }
                _ => {}
            }
        }
    }
    return found;
}

#[test]
fn test_solve() {
    let demo = read_input_file("day4", "demo.txt");
    let solution = solve_simple(&demo);
    assert_eq!(solution, 18);
}

#[test]
fn test_solve_advanced() {
    let demo = read_input_file("day4", "demo2.txt");
    let solution = solve_advanced(&demo);
    assert_eq!(solution, 9);
}
pub fn part1() -> usize {
    let full = read_input_file("day4", "full.txt");
    solve_simple(&full) 
}
pub fn part2() -> usize {
    let full = read_input_file("day4", "full.txt");
    solve_advanced(&full) 
}