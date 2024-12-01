use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;

pub struct Day1 {

}
type Input = (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>);

impl Day1 {
    fn parse_file(&self, input: &String) -> Result<Input, Box<dyn Error>> {
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        let lines = input.split("\n");
        for line in lines {
            let parts = line.split_whitespace();
            for (i, part) in parts.enumerate() {
                if i == 0 {
                    left.push(Reverse(part.parse()?));
                } else if i == 1 {
                    right.push(Reverse(part.parse()?));
                } else {
                    return Err("More than 2 values found on a single line of the input file")?
                }
            }
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