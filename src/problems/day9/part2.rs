use std::fs;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let lines = contents.lines().filter(|l| !l.is_empty());
    let mut result = 0;
    for line in lines {
        let top_entries: Vec<i64> = line.split_whitespace().map(|e| e.parse::<i64>().unwrap()).collect();
        let (left, _right) = compute_right_and_left_value_recursive(top_entries);
        result = result + left;
    }
    println!("Part 2: {}", result);
}

fn compute_right_and_left_value_recursive(parent: Vec<i64>) -> (i64, i64) {
    if parent.iter().all(|e| e == &0) {
        // base case
        return (0, 0);
    }

    let mut child_entries: Vec<i64> = vec![];
    for x in 0..(parent.len() - 1) {
        child_entries.push(parent[x + 1] - parent[x]);
    }

    let (child_left, child_right) = compute_right_and_left_value_recursive(child_entries);
    return (parent.first().unwrap() - child_left, parent.last().unwrap() + child_right);
}