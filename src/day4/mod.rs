use std::iter::Skip;
use std::ops::AddAssign;
use std::str::Chars;
use std::sync::Arc;
use crate::utils::{read_input_files, InputFiles};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use nom::Input;

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
pub fn solve_day4() {
    let files = read_input_files("day4");

    assert_eq!(solve_simple(&files.demo), 18);
    println!("Validation of part 1 passed, processing full file");
    let part1 = solve_simple(&files.full);
    println!("Solution part 1: {}", part1);
    //
    // assert_eq!(files.expected2, solve_advanced(demo2));
    // println!("Validation of part 2 passed, processing full file");
    // let solution = solve_advanced(files.full);
    // println!("Solution part 2: {}", solution);
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