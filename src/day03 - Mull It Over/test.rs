#[cfg(test)]
mod test {
    use crate::{parse, solve};
    use crate::parse::Mul;

    const DEMO: &'static str = include_str!("inputs/demo.txt");
    const DEMO2: &'static str = include_str!("inputs/demo2.txt");

    fn parse_demo() -> Vec<Mul> {
        parse::parse_part1(DEMO).expect("Demo input should parse")
    }
    fn parse_demo2() -> Vec<Mul> {
        parse::parse_part1(DEMO2).expect("Demo input should parse")
    }
    #[test]
    fn demo_inputs_parse() {
        parse_demo();
        parse_demo2();
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve(parse_demo()), 161);
    }
    #[test]
    fn test_solve_advanced() {
        assert_eq!(solve(parse_demo2()), 48);
    }
}
#[cfg(test)]
mod test_regression {
    #[test]
    fn test_part_1() {
        assert_eq!(crate::part1(), 173731097);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(crate::part2(), 93729253);
    }
}