use std::fs;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let lines = contents.lines().filter(|l| !l.is_empty());
    let mut all_digits = vec![];
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut second_digit: Option<u32> = None;
        for c in line.chars() {
            if c.is_numeric() {
                if first_digit.is_none() {
                    first_digit = c.to_digit(10);
                    second_digit = c.to_digit(10);
                } else {
                    second_digit = c.to_digit(10);
                }
            }
        }

        if first_digit.is_none() || second_digit.is_none() {
            panic!("Did not find digits in line")
        } else {
            all_digits.push(first_digit.unwrap() * 10 + second_digit.unwrap());
        }
    }

	println!("Part 1: {}", all_digits.iter().sum::<u32>());
}