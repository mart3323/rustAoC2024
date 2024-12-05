use std::iter::Skip;
use std::ops::AddAssign;
use std::str::Chars;
use std::sync::Arc;
use crate::utils::read_input_files;
use std::sync::atomic::{AtomicU32, Ordering};
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
fn process_chars(chars: Chars, total: Arc<AtomicU32>) {
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
    let width = files.demo.lines().next().unwrap().len();
    let height = files.demo.lines().count(); // Blank line at the end;

    let found : Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

    let input = files.demo.replace("\r", "").replace("\n", "");
    let input = input.as_str();
    thread::scope(|scope| {
        let vertical_offset = (width) * (height /* skip n-1 lines to get from first to last */);
        let horizontal_offset = width;
        for col in 0..width {
            let str = &input;
            let total = Arc::clone(&found);
            scope.spawn(move || {
                let mut state = SearchState::None;
                let mut score;
                println!("col {}: {}", col, str.chars().step_by(width).collect::<String>());
                for c in  str.chars().step_by(width) {
                    (state, score) = state.process_char(c as u8);
                    if score {
                        println!("Match found in col {}", col);
                        total.fetch_add(1, Ordering::Relaxed);
                    }
                };
            });
        }
        for row in 0..height {
            let str = &input[width*row..(width*(row+1))];
            let total = Arc::clone(&found);
            scope.spawn(move || {
                let mut state = SearchState::None;
                let mut score;
                println!("Row {}: {}", row, str);
                for c in str.chars() {
                    (state, score) = state.process_char(c as u8);
                    if score {
                        println!("Match found in row {}", row);
                        total.fetch_add(1, Ordering::Relaxed);
                    }
                };
            });
        }
    });
    println!("Maybe solution? {}", found.load(Ordering::Relaxed));
    // todo!();
    // assert_eq!(files.expected, solve_simple(files.demo.clone()));
    // println!("Validation of part 1 passed, processing full file");
    // let solution = solve_simple(files.full.clone());
    // println!("Solution part 1: {}", solution);
    //
    // assert_eq!(files.expected2, solve_advanced(demo2));
    // println!("Validation of part 2 passed, processing full file");
    // let solution = solve_advanced(files.full);
    // println!("Solution part 2: {}", solution);
}