use std::fs;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    println!("Part 1: {}", parse_engine_counts(&contents));
}

fn parse_engine_counts(input: &String) -> u32 {
    let lines = input.split('\n').filter(|l| !l.is_empty()).map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut sum_total = 0;
    for (row_idx, row) in lines.iter().enumerate() {
        for (col_idx, column) in row.iter().enumerate() {
            if !column.is_numeric() && *column != '.' {
                // find numbers near this, add to the sum total
                sum_total += get_adjacent_numbers(&lines, row_idx, col_idx);
            }
        }
    }

    sum_total
}

fn get_adjacent_numbers(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    let mut numbers: u32 = 0;
    if row_idx > 0 {
        if grid[row_idx - 1][col_idx].is_numeric() {
            // we can go center-top, no need top-left or top-right because they would be adjacent
            numbers += get_number(grid, row_idx - 1, col_idx).unwrap_or(0);
        } else {
            if col_idx > 0 {
                // we can go top-left
                numbers += get_number(grid, row_idx - 1, col_idx - 1).unwrap_or(0);
            }

            if col_idx < grid[row_idx].len() - 1 {
                // we can go top-right
                numbers += get_number(grid, row_idx - 1, col_idx + 1).unwrap_or(0);
            }
        }
    }

    if row_idx < grid.len() - 1 {
        // we can go bottom
        if grid[row_idx + 1][col_idx].is_numeric() {
            // we can go center-bottom, no need bottom-left or bottom-right
            numbers += get_number(grid, row_idx + 1, col_idx).unwrap_or(0);
        } else {
            if col_idx > 0 {
                // we can go bottom-left
                numbers += get_number(grid, row_idx + 1, col_idx - 1).unwrap_or(0);
            }

            if col_idx < grid[row_idx].len() - 1 {
                // we can go bottom-right
                numbers += get_number(grid, row_idx + 1, col_idx + 1).unwrap_or(0);
            }
        }
    }

    if col_idx > 0 {
        // we can go left
        numbers += get_number(grid, row_idx, col_idx - 1).unwrap_or(0);
    }

    if col_idx < grid[row_idx].len() - 1 {
        // we can go right
        numbers += get_number(grid, row_idx, col_idx + 1).unwrap_or(0);
    }

    numbers
}

fn get_number(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<u32> {
    let mut left_most_index = col;
    let mut right_most_index = col;
    if grid[row][col].is_numeric() {
        // go left until we don't find a number
        while left_most_index > 0 && grid[row][left_most_index - 1].is_numeric() {
            left_most_index -= 1;
        }

        // go right until we don't find a number
        while grid[row].len() - 1 > right_most_index && grid[row][right_most_index + 1].is_numeric() {
            right_most_index += 1;
        }

        // sum number, return it
        let mut number = 0;
        for index in left_most_index..=right_most_index {
            // 0..3
            // 1,2,3,4
            // 1 * 10^3-0 == 1000
            // 2 * 10^3-1 == 200
            // 3 * 10^3-2 == 30
            // 4 * 10^3-3 == 4

            number += grid[row][index].to_digit(10).unwrap() * 10u32.pow((right_most_index - index) as u32);
        }

        return Some(number);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::problems::day3::part1::{get_number, parse_engine_counts};

    #[test]
    fn test_input_runs() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, parse_engine_counts(&input.to_string()));
    }

    #[test]
    fn get_number_works() {
        assert_eq!(Some(467), get_number(&vec![vec!['4', '6', '7']], 0, 2));
        assert_eq!(Some(467), get_number(&vec![vec!['.', '.', '4', '6', '7']], 0, 2));
        assert_eq!(None, get_number(&vec![vec!['.', '.', '4', '6', '7']], 0, 1));
    }
}