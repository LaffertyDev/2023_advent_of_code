pub struct AoCDay {
	pub day: u8,
	pub part1: Box<dyn Fn(&std::path::PathBuf)>,
	pub part2: Box<dyn Fn(&std::path::PathBuf)>
}