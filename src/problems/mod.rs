use crate::problems::aocday::AoCDay;

pub mod aocday;

pub mod day0;
pub mod day1;
pub mod day2;
// pub mod day3;
// pub mod day4;
// pub mod day5;
// pub mod day6;
// pub mod day7;
// pub mod day8;
// pub mod day9;
// pub mod day10;
// pub mod day11;
// pub mod day12;
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
		}
    ]
}