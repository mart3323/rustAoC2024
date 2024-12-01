mod day1;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::exit;
use crate::day1::Day1;

fn main() {
    let d1 = Box::new(Day1 {});
    match d1.run() {
        Ok((res1, res2)) => {
            println!("Day1 Part 1 result: {}", res1.unwrap_or_else(|e| { e.to_string()}));
            println!("Day1 Part 2 result: {}", res2.unwrap_or_else(|e| { e.to_string()}));
        }
        Err(e) => {
            println!("Failed to process: {}", e);
        }
    }
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(&path).expect(
        format!("Reading file {}", 
            path.to_str().expect("Parsing path")
        ).as_str()
    )
}

pub trait AocSolver<Input, Input2> {
    const PATH: &'static str;
    fn parse(&self, input: &String) -> Result<Input, Box<dyn Error>>;
    fn solve(&self, input: Input) -> Result<String, Box<dyn Error>>;
    fn parse2(&self, input: &String) -> Result<Input2, Box<dyn Error>>;
    fn solve2(&self, input: Input2) -> Result<String, Box<dyn Error>>;

    fn run(&self) -> Result<(Result<String, Box<dyn Error>>, Result<String, Box<dyn Error>>), Box<dyn Error>> {
        println!("Parsing files");
        let demo = read_file(Path::new("src").join(Self::PATH).join("demo.txt").as_path());
        let expected = read_file(Path::new("src").join(Self::PATH).join("demo_solution.txt").as_path());
        let expected2 = read_file(Path::new("src").join(Self::PATH).join("demo_solution_2.txt").as_path());
        let full = read_file(Path::new("src").join(Self::PATH).join("full.txt").as_path());
        
        // Validation part 1
        {
            let parsed = self.parse(&demo)?;

            let solution = self.solve(parsed)?;
            if expected != solution {
                eprintln!("Solution does not match expected results!");
                eprintln!("Solution: {}", solution);
                eprintln!("Expected result: {}", expected);
                exit(1)
            } else {
                println!("Validation passed: {}", solution);
            }
        }
        // Validation part 2
        {
            let parsed = self.parse2(&demo)?;

            let solution = self.solve2(parsed)?;
            if expected2 != solution {
                eprintln!("Solution does not match expected results!");
                eprintln!("Solution: {}", solution);
                eprintln!("Expected result: {}", expected2);
                exit(1)
            } else {
                println!("Validation passed: {}", solution);
            }
        }
        let solution1 = {
            let parsed = self.parse(&full)?;
            self.solve(parsed)
        };
        let solution2 = {
            let parsed = self.parse2(&full)?;
            self.solve2(parsed)
        };
        Ok((solution1, solution2))
    }
}
