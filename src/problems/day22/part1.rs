use std::fs;
use crate::problems::day22::block_tower::BlockTower;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let block_tower = BlockTower::parse(&contents);
	println!("Part 1: {}", block_tower.count_bricks_that_can_disintegrate());
}