use peg;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use utils::read_input_file;

#[derive(Eq, PartialEq, Debug)]
struct Pair {
    left: usize,
    right: usize,
}

peg::parser!{
  grammar input_parser() for str {
    rule number() -> usize
      = n:$(['0'..='9']+) {
            ? n.parse().or(Err("i32"))
        }

    rule pair() -> Pair
      = l:number() " "+ r:number() {
            Pair{left: l, right: r}
        }

    pub rule parse() -> Vec<Pair>
        = p:pair() ** "\n" "\n"? { p }
  }
}

const DAY: &'static str = "day01 - Historian Hysteria";

type Input = (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>);

fn parse_demo() -> Vec<Pair> {
    input_parser::parse(&read_input_file(DAY, "demo.txt"))
        .expect("demo.txt file to be present and valid")
}
fn parse_full() -> Vec<Pair> {
    input_parser::parse(&read_input_file(DAY, "full.txt"))
        .expect("full.txt file to be present and valid")
}
fn solve_part1(input: &Vec<Pair>) -> Result<usize, &'static str> {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    input.iter().for_each(|pair| {
        left.push(Reverse(pair.left));
        right.push(Reverse(pair.right));
    });

    let mut total_difference: usize = 0;
    loop {
        if let (Some(Reverse(l)), Some(Reverse(r))) = (left.pop(), right.pop()) {
            total_difference += l.abs_diff(r);
        } else {
            // Out of values?
            return if left.is_empty() && right.is_empty() {
                Ok(total_difference)
            } else {
                Err("Unable to read values, but both lists are not yet empty")?
            }
        }
    }

}

fn solve_part2(input: &Vec<Pair>) -> Result<usize, &'static str> {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    input.iter().for_each(|pair| {
        left.push(Reverse(pair.left));
        right.push(Reverse(pair.right));
    });

    let mut difference_score: usize = 0;
    let mut prev_value_left: Option<usize> = None;
    let mut count = 0;
    loop {
        if let Some(Reverse(value_left)) = left.pop() {
            // Workaround: Maintain count when left has a duplicate value
            if Some(value_left) == prev_value_left {
                difference_score += value_left as usize * count;
                continue;
            }
            // Otherwise, start over the count with the new value
            prev_value_left = Some(value_left);
            count = 0;
            loop {
                if let Some(Reverse(v)) = right.pop() {
                    if v > value_left {
                        right.push(Reverse(v));
                        break;
                    } else if v == value_left {
                        count += 1;
                    } else {
                        // continue
                    }
                } else {
                    break // Ran out of items in right list
                }
            }
            difference_score += value_left * count;
        } else {
            break // Ran out of items in left list
        }
    }
    Ok(difference_score)
}

#[test]
fn test_parse() {
    let s = "1    2\n\
                   3   4\n\
                   5   6\n\
    ";
    let expected = vec!(
        Pair{left: 1, right: 2},
        Pair{left: 3, right: 4},
        Pair{left: 5, right: 6},
    );
    let pairs = input_parser::parse(s).expect("Should parse");
    assert_eq!(pairs, expected)
}

#[test]
fn test_part_1() {
    assert_eq!(solve_part1(&parse_demo()), Ok(11));
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part2(&parse_demo()), Ok(31));
}

fn part1() -> Result<usize, &'static str> {
    solve_part1(&parse_full())
}
fn part2() -> Result<usize, &'static str> {
    solve_part2(&parse_full())
}
