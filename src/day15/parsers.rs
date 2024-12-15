use nom::Parser;
use nom::IResult;
use crate::day15::types;

/// `@`, `#`, `O`, `.` 
fn parse_tile(str: &str) -> IResult<&str, types::Tile> {
    nom::character::complete::one_of("#O.@")
        .map_res(|c| types::Tile::try_from(c))
        .parse(str)
}

/// NB: Vertical line of > not part of map. Rustdoc doesn't allow starting a line with ##, even in
/// a code block
/// ```text
/// > #########
/// > #.....O.#
/// > ##..@...#
/// > #...OOOO#
/// > #O....O.#
/// > #.....O.#
/// > #########
/// ```
fn parse_map(str: &str) -> IResult<&str, types::Warehouse> {
    let line_ending = nom::character::complete::line_ending;
    let separated_list1 = nom::multi::separated_list1;
    let many1 = nom::multi::many1;
    
    let tile = parse_tile;
    let row = many1(tile);
    let map = separated_list1(line_ending, row);
    
    return map.map(types::Warehouse::from_2d_vec).parse(str);
}

/// `^` `>` `v` `<` 
fn parse_dir(str: &str) -> IResult<&str, types::Direction> {
    nom::character::complete::one_of("^>v<")
        .map_res(types::Direction::try_from)
        .parse(str)
}
/// ```text
/// ^<<<^^>>vvvvvvv>^>^
/// <<^vvv>^>>vv^>^^vv<
/// ```
fn parse_instructions(str: &str) -> IResult<&str, Vec<types::Direction>> {
    let line_ending = nom::character::complete::line_ending;
    let separated_list1 = nom::multi::separated_list1;
    let many1 = nom::multi::many1;
    let dir = parse_dir;
    
    separated_list1(line_ending, many1(dir))
        .map(|dirs| dirs.into_iter().flatten().collect())
        .parse(str)
}
/// NB: Vertical line of > not part of map. Rustdoc doesn't allow starting a line with ##, even in
/// a code block
/// ```text
/// > #########
/// > #.....O.#
/// > ##..@...#
/// > #...OOOO#
/// > #O....O.#
/// > #.....O.#
/// > #########
/// >
/// > ^<<<^^>>vvvvvvv>^>^
/// > <<^vvv>^>>vv^>^^vv<
/// ```
pub fn parse_input(str: &str) -> IResult<&str, (types::Warehouse, types::Instructions)> {
    let separated_pair = nom::sequence::separated_pair;
    let line_ending = nom::character::complete::line_ending;
    let map = parse_map;
    let instructions = parse_instructions;
    
    separated_pair(map, (line_ending, line_ending), instructions)
        .map(|(map, instructions)| (map, instructions))
        .parse(str)
}

#[test]
fn test_parse_input() {
    let (map, instructions) = parse_input("#########
#.....O.#
##..@...#
#...OOOO#
#O....O.#
#.....O.#
#########

^<<<^^>>vvvvvvv>^>^
<<^vvv>^>>vv^>^^vv<").expect("Should parse successfully").1;
    assert_eq!(*instructions.first().unwrap(), types::Direction::North);;
    assert_eq!(*instructions.last().unwrap(), types::Direction::West);
    assert_eq!(instructions.len(), 38);
    
    assert_eq!(*map.get_at(&types::Point::new(2, 2)).expect("2,2 should be on the map"), types::Tile::Empty);
    assert_eq!(*map.get_at(&types::Point::new(1, 2)).expect("1,3 should be on the map"), types::Tile::Wall);
    assert_eq!(*map.get_at(&types::Point::new(4, 2)).expect("5,2 should be on the map"), types::Tile::Robot);
    assert_eq!(*map.get_at(&types::Point::new(6, 4)).expect("6,4 should be on the map"), types::Tile::Box);
    assert_eq!(*map.get_at(&types::Point::new(8, 6)).expect("9,7 should be on the map"), types::Tile::Wall);
    assert!(map.get_at(&types::Point::new(8,7)).is_none());
    assert!(map.get_at(&types::Point::new(9,6)).is_none());
    assert!(map.get_at(&types::Point::new(-1,0)).is_none());
    assert!(map.get_at(&types::Point::new(0,-1)).is_none());
}
