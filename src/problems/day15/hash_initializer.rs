pub fn determine_hash(sequence: &str) -> usize {
    let mut current_hash = 0;
    for c in sequence.chars().filter(|c| c.is_ascii()) {
        if c == '\n' || c == '\r' {
            continue;
        }
        let char_value = c as usize; // only need 8 bits, let's us do math
        current_hash += char_value;
        current_hash *= 17;
        current_hash = current_hash % 256;
    }

    current_hash
}

pub fn determine_hash_sum(contents: &str) -> usize {
    let mut hash_result = 0;
    for sequence in contents.split(',').filter(|l| !l.is_empty()) {
        hash_result += determine_hash(sequence);
    }

    hash_result
}

#[cfg(test)]
mod tests {
    use crate::problems::day15::hash_initializer::determine_hash_sum;

    #[test]
    fn part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, determine_hash_sum(input));
    }
}