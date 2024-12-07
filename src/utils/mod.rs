use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> String {
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
pub fn read_input_file(day: &str, name: &str) -> String {
    read_file(Path::new("src").join(day).join(name).as_path())
}
pub fn read_input_files(day: &str) -> InputFiles{
    InputFiles{
        demo: read_input_file(day, "demo.txt"),
        expected: read_input_file(day, "demo_solution.txt"),
        expected2: read_input_file(day, "demo_solution_2.txt"),
        full: read_input_file(day, "full.txt"),
    }
}

/** https://stackoverflow.com/questions/13322479/how-to-benchmark-programs-in-rust */
pub struct Timer(&'static str, std::time::SystemTime);
impl Drop for Timer {
    fn drop(&mut self) {
        println!(
            "{} finished in {:?} ms",
            self.0,
            self.1.elapsed().unwrap_or_default()
        );
    }
}

impl Timer {
    pub fn start(op: &'static str) -> Timer {
        let now = std::time::SystemTime::now();

        Timer(op, now)
    }
}