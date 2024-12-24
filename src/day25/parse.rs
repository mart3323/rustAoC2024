use crate::day25::types;
use crate::day25::types::LockPattern;

#[derive(Debug, Eq, PartialEq)]
pub enum Pattern {
    Lock(types::LockPattern),
    Key(types::LockPattern)
}
fn parse_pattern(str: &str) -> Result<Pattern, String> {
    let lines: Vec<&str> = str.lines().map(|s| s.trim_end()).collect();
    if lines.len() < 7 {
        return Err("Not enough lines to parse pattern".to_string());
    } else if lines.len() > 7 {
        return Err("Too many lines to parse pattern".to_string());
    }
    let [top, l1, l2, l3, l4, l5, bottom]: [&str; 7] = [
        lines.get(0).unwrap(),
        lines.get(1).unwrap(),
        lines.get(2).unwrap(),
        lines.get(3).unwrap(),
        lines.get(4).unwrap(),
        lines.get(5).unwrap(),
        lines.get(6).unwrap(),
    ];
    match top {
        "#####" => {
            if bottom != "....." { return Err("Lock pattern must end with empty line".to_string()); };
            let mut heights = [0u8; 5];
            for (i, line) in [l1, l2, l3, l4, l5].iter().enumerate() {
                for (j, char) in line.chars().enumerate() {
                    if char == '#' {
                        heights[j] += 1;
                    }
                }
            }
            return Ok(Pattern::Lock(LockPattern::from(&(heights[0], heights[1], heights[2], heights[3], heights[4]))));
        }
        "....." => {
            if bottom != "#####" { return Err("Key pattern must end with empty line".to_string()); };
            let mut heights = [5u8; 5];
            for (i, line) in [l1, l2, l3, l4, l5].iter().enumerate() {
                for (j, char) in line.chars().enumerate() {
                    if char == '.' {
                        heights[j] -= 1;
                    }
                }
            }
            return Ok(Pattern::Key(LockPattern::from(&(heights[0], heights[1], heights[2], heights[3], heights[4]))));

        },
        _ => return Err("Pattern must start with full or empty line".to_string())
    }
}

pub fn parse_patterns(str: &str) -> Result<Vec<Pattern>, String> {
    str.split("\n\n").map(|str| parse_pattern(str)).collect()
}

#[test]
fn test_parse_patterns() {
    let p1 = parse_patterns("\
#####
.####
.####
.####
.#.#.
.#...
.....

.....
#....
#....
#...#
#.#.#
#.###
#####
");
    assert_eq!(p1, Ok(vec!(
        Pattern::Lock(LockPattern::from(&(0,5,3,4,3))),
        Pattern::Key(LockPattern::from(&(5,0,2,1,3)))
    )));
}