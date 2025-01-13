#[cfg(test)]
mod test {
    use crate::{parse, solve_part1, solve_part2};
    use crate::parse::Pair;

    const DEMO: &'static str = include_str!("inputs/demo.txt");

    fn parse_demo() -> Vec<Pair> {
        parse::parse_input(DEMO).expect("Input to parse")
    }
    #[test]
    fn test_demo_input_parses() {
        parse_demo();
    }
    #[test]
    fn test_part_1() {
        let input = parse_demo();
        let solution = solve_part1(&input);
        assert_eq!(solution, 11);
    }

    #[test]
    fn test_part_2() {
        let input = parse_demo();
        let solution = solve_part2(&input);
        assert_eq!(solution, 31);
    }
}

#[cfg(test)]
mod test_regression {
    #[test]
    fn test_part_1() {
        assert_eq!(crate::part1(), 1765812);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(crate::part2(), 20520794);
    }
}