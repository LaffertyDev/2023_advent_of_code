use std::fs;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let lines = contents.split("\n").filter(|l| !l.is_empty());
    let mut all_digits = vec![];
    for line in lines {
        // find first digit in sequence
        // options
        // 1,2,3,4,5,6,7,8,9,0,one,two,three,four,five,six,seven,eight,nine
        let first_digit = find_digit(line.chars(), false);
        let second_digit = find_digit(line.chars().rev(), true);
        if first_digit.is_none() {
            panic!("Did not find front digit");
        }

        if second_digit.is_none() {
            panic!("Did not find second digit");
        }
        all_digits.push(first_digit.unwrap() * 10 + second_digit.unwrap());
    }

    println!("Part 2: {:}", all_digits.iter().sum::<u32>());
}

fn find_digit(line: impl Iterator<Item = char>, is_reversed: bool) -> Option<u32> {
    let match_options_forward: Vec<String> = vec![
        "0".into(),
        "1".into(),
        "2".into(),
        "3".into(),
        "4".into(),
        "5".into(),
        "6".into(),
        "7".into(),
        "8".into(),
        "9".into(),
        "one".into(),
        "two".into(),
        "three".into(),
        "four".into(),
        "five".into(),
        "six".into(),
        "seven".into(),
        "eight".into(),
        "nine".into()
    ];
    let match_options_backward = match_options_forward.iter().map(|l| l.chars().rev().collect::<String>()).collect();

    let matching_options = if is_reversed { match_options_backward } else { match_options_forward };

    let mut currently_considering_words = vec![];

    for c in line {
        for (match_index, match_option) in matching_options.iter().enumerate() {
            // first, is this worth considering?
            if match_option.chars().nth(0).unwrap() == c {
                // yes, this character could match a word
                currently_considering_words.push((match_index, 0usize));
            }
        }

        for (word_under_consideration, index_at_word) in currently_considering_words.iter_mut() {
            let word_under_consideration = &matching_options[*word_under_consideration];
            if word_under_consideration.chars().nth(*index_at_word).unwrap() == c {
                // this word continues to be an option or is the option
                if word_under_consideration.len() == (*index_at_word) + 1 {
                    // we have our ticket
                    return Some(digit_to_number(word_under_consideration));
                }

                (*index_at_word) += 1; // advance to the next character
            } else {
                // this word is no longer an option, mark it for deletion
                (*index_at_word) = 0
            }
        }

        currently_considering_words.retain(|(_word, index)| index > &0usize);
    }

    None
}

fn digit_to_number(digit: &str) -> u32 {
    match digit {
        "0" | "zero" | "orez" => 0,
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => panic!("Unsupported")
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day1::part2::find_digit;

    #[test]
    fn find_digit_one_one() {
        assert_eq!(1, find_digit("1".chars(), false).unwrap());
        assert_eq!(1, find_digit("1".chars(), true).unwrap());
        assert_eq!(1, find_digit("one".chars(), false).unwrap());
        assert_eq!(1, find_digit("one".chars().rev(), true).unwrap());
    }

    #[test]
    fn find_digit_twone() {
        assert_eq!(2, find_digit("twone".chars(), false).unwrap());
        assert_eq!(1, find_digit("twone".chars().rev(), true).unwrap());
    }
}