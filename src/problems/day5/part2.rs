use std::fs;
pub fn execute(input_path: &std::path::PathBuf) {
    let _contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    println!("Part 2");
}
