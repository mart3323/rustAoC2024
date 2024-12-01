use std::error::Error;
use std::str::FromStr;

pub struct Day0 {

}
impl crate::AocSolver<u64, u64> for Day0 {
    const PATH: &'static str = "day0";

    fn parse(&self, input: &String) -> Result<u64, Box<dyn Error>> {
        u64::from_str(&input).map_err(|e| Box::new(e).into())
    }
    fn solve(&self, input: u64) -> Result<String, Box<dyn Error>> {
        Ok((input + input).to_string())
    }

    fn parse2(&self, input: &String) -> Result<u64, Box<dyn Error>> {
        self.parse(input)
    }
    fn solve2(&self, input: u64) -> Result<String, Box<dyn Error>> {
        Ok((input * input).to_string())
    }
}