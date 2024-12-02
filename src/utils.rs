use std::fs;
use std::path::Path;

fn read_file(path: &Path) -> String {
    fs::read_to_string(&path).expect(
        format!("Reading file {}",
                path.to_str().expect("Parsing path")
        ).as_str()
    )
}

pub struct InputFiles {
    pub demo: String,
    pub expected: String,
    pub expected2: String,
    pub full: String,
}
pub fn read_input_files(day: &str) -> InputFiles{
    InputFiles{
        demo: read_file(Path::new("src").join(day).join("demo.txt").as_path()),
        expected: read_file(Path::new("src").join(day).join("demo_solution.txt").as_path()),
        expected2: read_file(Path::new("src").join(day).join("demo_solution_2.txt").as_path()),
        full: read_file(Path::new("src").join(day).join("full.txt").as_path()),
    }
}