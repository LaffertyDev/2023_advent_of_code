pub mod part1;
pub mod part2;

use crate::problems::aocday::AoCDay;

pub struct Day1 {

}

impl AoCDay for Day1 {
	fn part1(&self, input_path: &std::path::PathBuf) {
		part1::execute(input_path);
	}

	fn part2(&self, input_path: &std::path::PathBuf) {
		part2::execute(input_path);
	}
}