use crate::problems::aocday::AoCDay;

pub mod aocday;

mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub fn get_days() -> Vec<aocday::AoCDay> {
	vec![
		AoCDay {
			day: 0,
			part1: Box::new(day0::part1::execute),
			part2: Box::new(day0::part2::execute)
		},
		AoCDay {
			day: 1,
			part1: Box::new(day1::part1::execute),
			part2: Box::new(day1::part2::execute)
		},
		AoCDay {
			day: 2,
			part1: Box::new(day2::part1::execute),
			part2: Box::new(day2::part2::execute)
		},
		AoCDay {
			day: 3,
			part1: Box::new(day3::part1::execute),
			part2: Box::new(day3::part2::execute)
		},
		AoCDay {
			day: 4,
			part1: Box::new(day4::part1::execute),
			part2: Box::new(day4::part2::execute)
		},
		AoCDay {
			day: 5,
			part1: Box::new(day5::part1::execute),
			part2: Box::new(day5::part2::execute)
		},
		AoCDay {
			day: 6,
			part1: Box::new(day6::part1::execute),
			part2: Box::new(day6::part2::execute)
		},
		AoCDay {
			day: 7,
			part1: Box::new(day7::part1::execute),
			part2: Box::new(day7::part2::execute)
		},
		AoCDay {
			day: 8,
			part1: Box::new(day8::part1::execute),
			part2: Box::new(day8::part2::execute)
		},
		AoCDay {
			day: 9,
			part1: Box::new(day9::part1::execute),
			part2: Box::new(day9::part2::execute)
		},
		AoCDay {
			day: 10,
			part1: Box::new(day10::part1::execute),
			part2: Box::new(day10::part2::execute)
		},
		AoCDay {
			day: 11,
			part1: Box::new(day11::part1::execute),
			part2: Box::new(day11::part2::execute)
		},
		AoCDay {
			day: 12,
			part1: Box::new(day12::part1::execute),
			part2: Box::new(day12::part2::execute)
		},
    ]
}