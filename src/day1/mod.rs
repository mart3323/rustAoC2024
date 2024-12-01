use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::str::FromStr;
use nom;
use nom::{IResult, Parser};

// region input
// Using nom, parse out an integer value
fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    (
        nom::character::complete::digit1.map_res(i32::from_str),
        nom::character::complete::multispace1,
        nom::character::complete::digit1.map_res(i32::from_str),
    )
        .map(|(left, _, right)| { (left, right)})
        .parse(input)
}
fn parse_file(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    nom::multi::separated_list1(nom::character::complete::line_ending, parse_line)
        .parse(input)
}
// endregion

pub struct Day1 {

}
type Input = (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>);


impl Day1 {
    fn parse_file(&self, input: &String) -> Result<Input, Box<dyn Error>> {
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        let result1 = parse_file(input);
        for (l, r) in result1.unwrap().1 {
            left.push(Reverse(l));
            right.push(Reverse(r));
        }
        Ok((left, right))
    }
}
impl crate::AocSolver<Input, Input> for Day1 {
    const PATH: &'static str = "day1";


    fn parse(&self, input: &String) -> Result<Input, Box<dyn Error>> {
        self.parse_file(input)
    }
    fn solve(&self, (left, right): Input) -> Result<String, Box<dyn Error>> {
        let mut left = left;
        let mut right = right;
        let mut total_difference = 0;
        loop {
            if let (Some(Reverse(l)), Some(Reverse(r))) = (left.pop(), right.pop()) {
                total_difference += (l-r).abs()
            } else {
                // Out of values?
                return if left.is_empty() && right.is_empty() {
                    Ok(total_difference.to_string())
                } else {
                    Err("Unable to read values, but both lists are not yet empty")?
                }
            }
        }
    }

    fn parse2(&self, input: &String) -> Result<Input, Box<dyn Error>> {
        self.parse_file(input)
    }
    fn solve2(&self, input: Input) -> Result<String, Box<dyn Error>> {
        let (mut left, mut right) = input;
        let mut difference_score = 0;
        let mut prev_value_left: Option<i32> = None;
        let mut count = 0;
        loop {
            if let Some(Reverse(value_left)) = left.pop() {
                // Workaround: Maintain count when left has a duplicate value
                if Some(value_left) == prev_value_left {
                    difference_score += value_left * count;
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
        Ok(difference_score.to_string())
    }
}