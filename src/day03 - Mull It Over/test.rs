#[cfg(test)]
mod test {
    use crate::{solve_part1, solve_part2};
    
    const DEMO: &'static str = include_str!("inputs/demo.txt");
    const DEMO2: &'static str = include_str!("inputs/demo2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(DEMO), 161);
    }
    #[test]
    fn test_solve_advanced() {
        assert_eq!(solve_part2(DEMO2), 48);
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