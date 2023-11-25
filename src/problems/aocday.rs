pub struct AoCDay {
	pub day: usize,
	pub part1: Box<dyn Fn(&std::path::PathBuf)>,
	pub part2: Box<dyn Fn(&std::path::PathBuf)>
}