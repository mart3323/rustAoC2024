use crate::utils::read_input_files;
use nom::multi::{many1, separated_list1};
use nom::Parser;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

// region input
struct PairOrderingRules(HashMap<u32, HashSet<u32>>);

#[derive(Debug)]
#[derive(Clone)]
struct Update(Vec<u32>);

fn parse_int(input: &str) -> nom::IResult<&str, u32> {
    use nom::character::complete::digit1;
    digit1.map_res(str::parse)
        .parse(input)
}
fn parse_page_ordering_rule(input: &str) -> nom::IResult<&str, (u32, u32)> {
    use nom::bytes::complete::tag;

    (parse_int,tag("|"),parse_int)
        .map(| (before, _, after)| (before, after))
        .parse(input)
}
fn parse_update(input: &str) -> nom::IResult<&str, Update> {
    use nom::bytes::complete::tag;

    separated_list1(tag(","), parse_int)
        .map(|items| Update(items))
        .parse(input)
}
fn parse_file(input: &str) -> nom::IResult<&str, (PairOrderingRules, Vec<Update>)> {
    use nom::character::complete::line_ending;
    use nom::sequence::separated_pair;

    separated_pair(
        separated_list1(line_ending, parse_page_ordering_rule)
            .map(|items| PairOrderingRules(
                items.into_iter()
                    .fold(HashMap::new(),|mut hs, (before, after)| {
                        if let Some(befores) = hs.get_mut(&after) {
                            befores.insert(before);
                        } else {
                            hs.insert(after, HashSet::from([before]));
                        }
                        return hs;
                    })
            )),
        (line_ending, line_ending),
        separated_list1(line_ending, parse_update),
    )
        .parse(input)
}
// endregion

impl Update {
    fn get_middle_page(&self) -> &u32 {
        self.0.get(self.0.len() / 2)
            .expect("Expected vector of size N to have an item at index N/2")
    }
    fn validate(&self, rules: &PairOrderingRules) -> bool {
        let mut forbidden = HashSet::with_capacity(rules.0.len());
        for page_number in self.0.iter() {
            if forbidden.contains(page_number) {
                return false;
            }
            if let Some(befores) = rules.0.get(&page_number) {
                for before in befores {
                    forbidden.insert(before);
                }
            }
        };
        return true;
    }
    fn fix(&mut self, rules: &PairOrderingRules) {
        loop {
            let mut invalid_index = None;
            let mut forbidden = HashSet::with_capacity(rules.0.len());
            // Find invalid index
            for (i, page_number) in self.0.iter().enumerate() {
                if let Some(befores) = rules.0.get(&page_number) {
                    for before in befores {
                        forbidden.insert(before);
                    }
                }
                if forbidden.contains(page_number) {
                    invalid_index = Some(i);
                    break;
                }
            }
            if invalid_index == None {
                return // Entire list passed validation
            }
            let page_number = self.0.remove(invalid_index.unwrap());
            let mut insertion_index = None;
            for (i, nr) in self.0.iter().enumerate() {
                if let Some(befores) = rules.0.get(nr) {
                    if befores.contains(&page_number) {
                        insertion_index = Some(i);
                        break;
                    }
                }
            }
            if let Some(index) = insertion_index {
                self.0.insert(index, page_number);
            } else {
                panic!("Number was invalid before, but now passed the entire list without conflict. This should not happen")
            }
        };
    }
}
fn solve_simple(rules: &PairOrderingRules, updates: &Vec<Update>) -> u32 {
    let mut total = 0;
    for upd in updates {
        if upd.validate(rules) {
            let middle_page = upd.get_middle_page();
            total += middle_page;
        }
    }
    return total;
}
fn solve_advanced(rules: &PairOrderingRules, updates: &mut Vec<Update>) -> u32 {
    let mut total = 0;
    for upd in updates {
        if !upd.validate(rules) {
            upd.fix(rules);
            total += upd.get_middle_page();
        }
    }
    return total;
}

pub fn solve_day5() {
    let files = read_input_files("day5");
    let demo = parse_file(&files.demo).expect("Demo file should parse").1;
    let full = parse_file(&files.full).expect("Full file should parse").1;
    let demo_expected: u32 = str::parse(&files.expected).expect("Solution should be a valid number");

    assert_eq!(solve_simple(&demo.0, &demo.1), 143);
    println!("Demo 1 passed");
    println!("full solution is {}", solve_simple(&full.0, &full.1));

    assert_eq!(solve_advanced(&demo.0, &mut demo.1.clone()), 123);
    println!("Demo 2 passed");
    println!("full solution is {}", solve_advanced(&full.0, &mut full.1.clone()));
    
}
