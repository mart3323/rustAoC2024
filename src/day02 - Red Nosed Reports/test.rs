#[cfg(test)]
mod test {
    use crate::{parse, solve, solve2};

    const DEMO: &'static str = include_str!("inputs/demo.txt");

    fn parse_demo() -> Vec<parse::Report> {
        parse::parse_reports_file(DEMO).expect("Input to parse")
    }
    #[test]
    fn demo_input_parses() {
        parse_demo();
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse_demo()), 2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve2(&parse_demo()), 4)
    }
}

#[cfg(test)]
mod test_regression {
    #[test]
    fn test_part_1() {
        assert_eq!(crate::part1(), 202);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(crate::part2(), 271);
    }
}