mod test;

const FULL: &'static str = include_str!("inputs/full.txt");


pub fn solve_part1(input: &str) -> usize {
    let mut remainder = input;
    let mut total = 0;
    while let Some(index) = remainder.chars().position(|c| c == 'm') {
        let a : usize;
        let b : usize;
        // consume until m
        remainder = &remainder[index..];
        // consume mul(
        if !remainder.starts_with("mul(") {
            remainder = &remainder[1..];
            continue;
        }
        remainder = &remainder[4..];
        // consume first number
        match remainder.find(|c: char| !c.is_numeric()) {
            None => {
                remainder = "";
                continue;
            },
            Some(index) => {
                a = remainder[0..index].parse().unwrap();
                remainder = &remainder[index..];
            }
        }
        // consume comma
        match remainder.chars().next() {
            Some(',') => {remainder = &remainder[1..]}
            _ => { continue; }
        }
        // consume second number
        match remainder.find(|c: char| !c.is_numeric()) {
            None => {
                remainder = "";
                continue;
            },
            Some(index) => {
                b = remainder[0..index].parse().unwrap();
                remainder = &remainder[index..];
            }
        }
        // consume close paren
        match remainder.chars().next() {
            Some(')') => { remainder = &remainder[1..]}
            _ => { continue; }
        }
        // multiply and update total
        total += a * b
    }
    return total;
}
pub fn solve_part2(input: &str) -> usize {

    let mut remainder = input;
    let mut total = 0;
    let mut off = false;
    while let Some(index) = remainder.chars().position(|c| (!off && c == 'm') || c == 'd') {
        let a : usize;
        let b : usize;
        // Parse comment tokens
        remainder = &remainder[index..];
        if remainder.starts_with("don't()") {
            off = true;
            remainder = &remainder["don't()".len()..];
            continue;
        } else if remainder.starts_with("do()") {
            off = false;
            remainder = &remainder["do()".len()..];
            continue;
        }
        
        // consume mul(
        if !remainder.starts_with("mul(") {
            remainder = &remainder[1..];
            continue;
        }
        remainder = &remainder[4..];
        // consume first number
        match remainder.find(|c: char| !c.is_numeric()) {
            None => {
                remainder = "";
                continue;
            },
            Some(index) => {
                a = remainder[0..index].parse().unwrap();
                remainder = &remainder[index..];
            }
        }
        // consume comma
        match remainder.chars().next() {
            Some(',') => {remainder = &remainder[1..]}
            _ => { continue; }
        }
        // consume second number
        match remainder.find(|c: char| !c.is_numeric()) {
            None => {
                remainder = "";
                continue;
            },
            Some(index) => {
                b = remainder[0..index].parse().unwrap();
                remainder = &remainder[index..];
            }
        }
        // consume close paren
        match remainder.chars().next() {
            Some(')') => { remainder = &remainder[1..]}
            _ => { continue; }
        }
        // multiply and update total
        total += a * b
    }
    return total;
}

pub fn part1() -> usize {
    solve_part1(FULL)
}
pub fn part2() -> usize {
    solve_part2(FULL)
}