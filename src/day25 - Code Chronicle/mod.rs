use crate::types::LockCompatibilityTable;
use utils::read_input_file;

mod types;
mod parse;

const DAY: &str = "day25 - Code Chronicle";

#[test]
fn test_part1() {
    let patterns = parse::parse_patterns(&read_input_file(DAY, "demo.txt")).expect("Failed to parse file");
    let mut compat_table = LockCompatibilityTable::new();
    for p in &patterns {
        if let parse::Pattern::Lock(p) = p {
            compat_table.increment_fitting(*p)
        }
    }
    let mut total = 0;
    for p in &patterns {
        if let parse::Pattern::Key(p) = p {
            total += compat_table.get_fitting_count(*p);
        }
    }
    assert_eq!(total, 3);
}

pub fn part1() -> usize {
    let patterns = parse::parse_patterns(&read_input_file(DAY, "full.txt")).expect("Failed to parse file");
    let mut compat_table = LockCompatibilityTable::new();
    for p in &patterns {
        if let parse::Pattern::Lock(p) = p {
            compat_table.increment_fitting(*p)
        }
    }
    let mut total = 0;
    for p in &patterns {
        if let parse::Pattern::Key(p) = p {
            total += compat_table.get_fitting_count(*p);
        }
    }
    return total;
}

pub fn part2() -> usize {
    todo!();
}