use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum SpringStatus {
    Functional,
    Damaged,
    Unknown
}

impl SpringStatus {
    pub fn parse(c: char) -> SpringStatus {
        match c {
            '#' => SpringStatus::Damaged,
            '.' => SpringStatus::Functional,
            '?' => SpringStatus::Unknown,
            _ => panic!()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SpringGroup {
    springs: Vec<SpringStatus>,
    maps: Vec<usize>
}

impl SpringGroup {
    pub fn init(springs: &[SpringStatus], maps: &[usize]) -> SpringGroup {
        SpringGroup {
            springs: springs.to_vec(),
            maps: maps.to_vec()
        }
    }

    pub fn parse(line: &str, repeat_counts: usize) -> SpringGroup {
        let mut split = line.split_whitespace();
        let mut springs: Vec<SpringStatus> = split.next().unwrap().chars().map(|c| SpringStatus::parse(c)).collect();
        let maps: Vec<usize> = split.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        if repeat_counts > 1 {
            springs.push(SpringStatus::Unknown);
        }
        let mut springs = springs.repeat(repeat_counts);
        if repeat_counts > 1 {
            springs.pop();
        }
        let maps = maps.repeat(repeat_counts);

        SpringGroup {
            springs,
            maps
        }
    }

    pub fn get_permutation_dynamic(spring_group: SpringGroup, permutation_map: &mut HashMap<SpringGroup, usize>) -> usize {
        if let Some(permutations) = permutation_map.get(&spring_group) {
            return *permutations;
        }

        if spring_group.springs.len() == 0 || spring_group.maps.len() == 0 {
            // base case
            return 0;
        }

        let mut possible_child_permutations = 0;
        let mut damaged_spring_group_index = 0;
        let mut s_index = 0;
        while s_index < spring_group.springs.len() && damaged_spring_group_index < spring_group.maps.len() {
            if spring_group.springs[s_index] == SpringStatus::Functional {
                s_index += 1;
                continue;
            }

            let group_length = spring_group.maps[damaged_spring_group_index];
            if s_index + group_length > spring_group.springs.len() {
                // the length of this group is greater than the remaining elements
                // therefor, this is not a valid permutation
                permutation_map.entry(spring_group).or_insert(possible_child_permutations);
                return possible_child_permutations;
            }

            for forward in s_index..s_index+group_length {
                match spring_group.springs[forward] {
                    SpringStatus::Functional => {
                        // the group length contains a Functional node
                        // so this branch is dead
                        permutation_map.entry(spring_group).or_insert(possible_child_permutations);
                        return possible_child_permutations;
                    },
                    SpringStatus::Damaged => {
                        continue;
                    },
                    SpringStatus::Unknown => {
                        if forward == s_index {
                            // special case, we are at a branch at the start
                            // if the start node is FUNCTIONAL, then there are still possible permutations
                            // if the start node is DAMAGED, then we simple continue our scan
                            let sub_spring_group = SpringGroup::init(&spring_group.springs[s_index + 1..], &spring_group.maps[damaged_spring_group_index..]);
                            let sub_permutations = SpringGroup::get_permutation_dynamic(sub_spring_group, permutation_map);
                            possible_child_permutations += sub_permutations;
                        } else {
                            // we found an unknown!
                            // this could be functional or non-functional
                            // functional means the current permutations cease because we can't resolve the group dynamic
                            // nonfunctional means we continue checking the group dynamics
                            // so that means there is no branching states to check here
                            // assume this is non functional
                            continue;
                        }
                    }
                }
            }

            if s_index + group_length == spring_group.springs.len() {
                // special case, we've reached the end. We're done! And we've satisfied the constraints!
                s_index += group_length + 1;
                damaged_spring_group_index += 1;
                continue;
            }

            if spring_group.springs[s_index + group_length] == SpringStatus::Damaged {
                // this is an invalid group
                // because the node after the group is not a Functional node.
                permutation_map.entry(spring_group).or_insert(possible_child_permutations);
                return possible_child_permutations;
            } else {
                // this is either Functional or an Unknown
                // if it is an unknown, it is either Functional or Damaged
                // if it is damaged, this is an invalid group target
                // so there is only one possible scenario where this is a permutation
                s_index += group_length + 1;
                damaged_spring_group_index += 1;
            }
        }

        // we have exhausted either the groups or the nodes
        // make sure if we exhausted the groups, no more Damaged nodes remain
        for remaining in s_index..spring_group.springs.len() {
            if spring_group.springs[remaining] == SpringStatus::Damaged {
                permutation_map.entry(spring_group).or_insert(possible_child_permutations);
                return possible_child_permutations;
            }
        }

        // make sure if we exhausted the nodes, that no more groups remain
        if damaged_spring_group_index < spring_group.maps.len() {
            permutation_map.entry(spring_group).or_insert(possible_child_permutations);
            return possible_child_permutations; // did not satisfy constraints
        }

        // we successfully satisfied the constraints
        permutation_map.entry(spring_group).or_insert(possible_child_permutations + 1);
        possible_child_permutations + 1
    }

    pub fn get_permutation(springs: &[SpringStatus], damaged_spring_group: &[usize]) -> usize {
        if springs.len() == 0 || damaged_spring_group.len() == 0 {
            // base case
            return 0;
        }

        // figure out how many damaged nodes are in the group
        let damaged_nodes_count = damaged_spring_group.iter().sum();
        let damaged_or_unknown_springs_count = springs.iter().filter(|s| s != &&SpringStatus::Functional).count();
        if damaged_or_unknown_springs_count < damaged_nodes_count {
            return 0; // this is an invalid path
        }

        let mut possible_child_permutations = 0;
        let mut damaged_spring_group_index = 0;
        let mut s_index = 0;
        while s_index < springs.len() && damaged_spring_group_index < damaged_spring_group.len() {
            if springs[s_index] == SpringStatus::Functional {
                s_index += 1;
                continue;
            }

            let group_length = damaged_spring_group[damaged_spring_group_index];
            if s_index + group_length > springs.len() {
                // the length of this group is greater than the remaining elements
                // therefor, this is not a valid permutation
                return possible_child_permutations;
            }

            for forward in s_index..s_index+group_length {
                match springs[forward] {
                    SpringStatus::Functional => {
                        // the group length contains a Functional node
                        // so this branch is dead
                        return possible_child_permutations;
                    },
                    SpringStatus::Damaged => {
                        continue;
                    },
                    SpringStatus::Unknown => {
                        if forward == s_index {
                            // special case, we are at a branch at the start
                            // if the start node is FUNCTIONAL, then there are still possible permutations
                            // if the start node is DAMAGED, then we simple continue our scan
                            let sub_permutations = SpringGroup::get_permutation(&springs[s_index + 1..], &damaged_spring_group[damaged_spring_group_index..]);
                            possible_child_permutations += sub_permutations;
                        } else {
                            // we found an unknown!
                            // this could be functional or non-functional
                            // functional means the current permutations cease because we can't resolve the group dynamic
                            // nonfunctional means we continue checking the group dynamics
                            // so that means there is no branching states to check here
                            // assume this is non functional
                            continue;
                        }
                    }
                }
            }

            if s_index + group_length == springs.len() {
                // special case, we've reached the end. We're done! And we've satisfied the constraints!
                s_index += group_length + 1;
                damaged_spring_group_index += 1;
                continue;
            }

            if springs[s_index + group_length] == SpringStatus::Damaged {
                // this is an invalid group
                // because the node after the group is not a Functional node.
                return possible_child_permutations;
            } else {
                // this is either Functional or an Unknown
                // if it is an unknown, it is either Functional or Damaged
                // if it is damaged, this is an invalid group target
                // so there is only one possible scenario where this is a permutation
                s_index += group_length + 1;
                damaged_spring_group_index += 1;
            }
        }

        // we have exhausted either the groups or the nodes
        // make sure if we exhausted the groups, no more Damaged nodes remain
        for remaining in s_index..springs.len() {
            if springs[remaining] == SpringStatus::Damaged {
                return possible_child_permutations;
            }
        }

        // make sure if we exhausted the nodes, that no more groups remain
        if damaged_spring_group_index < damaged_spring_group.len() {
            return possible_child_permutations; // did not satisfy constraints
        }

        // we successfully satisfied the constraints
        possible_child_permutations + 1
    }

    pub fn get_permutations(&self) -> usize {
        SpringGroup::get_permutation_dynamic(self.clone(), &mut HashMap::new())
    }
}

pub struct SpringCollection {
    spring_groups: Vec<SpringGroup>
}

impl SpringCollection {
    pub fn parse(contents: &str, repeat_counts: usize) -> SpringCollection {
        SpringCollection {
            spring_groups: contents.lines().filter(|l| !l.is_empty()).map(|l| SpringGroup::parse(l, repeat_counts)).collect()
        }
    }

    pub fn get_possible_permutations(&self) -> usize {
        self.spring_groups.iter().map(|sg| sg.get_permutations()).sum()
    }
}


#[cfg(test)]
mod tests {
    use crate::problems::day12::springy::SpringCollection;

    #[test]
    fn part1_test() {
        assert_eq!(1, SpringCollection::parse("???.### 1,1,3", 1).get_possible_permutations());
        assert_eq!(4, SpringCollection::parse(".??..??...?##. 1,1,3", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?#?#?#?#?#?#?#? 1,3,1,6", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("????.#...#... 4,1,1", 1).get_possible_permutations());
        assert_eq!(4, SpringCollection::parse("????.######..#####. 1,6,5", 1).get_possible_permutations());
        assert_eq!(10, SpringCollection::parse("?###???????? 3,2,1", 1).get_possible_permutations());

        let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!(21, SpringCollection::parse(&input, 1).get_possible_permutations());
    }

    #[test]
    fn part1_basic_series() {
        assert_eq!(1, SpringCollection::parse("# 1", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("## 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("### 3", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("#### 4", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("##### 5", 1).get_possible_permutations());
    }

    #[test]
    fn part1_base_cases() {
        assert_eq!(0, SpringCollection::parse(". 1", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("? 1", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("# 1", 1).get_possible_permutations());
    }

    #[test]
    fn part1_two() {
        assert_eq!(0, SpringCollection::parse(".. 1", 1).get_possible_permutations());
        // #.
        // .#
        assert_eq!(2, SpringCollection::parse("?? 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("## 1", 1).get_possible_permutations());
    }

    #[test]
    fn part1_three() {
        assert_eq!(0, SpringCollection::parse("... 1", 1).get_possible_permutations());
        // #..
        // .#.
        // ..#
        assert_eq!(3, SpringCollection::parse("??? 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("### 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("#.# 1", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?.# 1", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("#.? 1", 1).get_possible_permutations());
    }

    #[test]
    fn part1_test2() {
        assert_eq!(1, SpringCollection::parse("# 1", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("#.# 1,1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse(". 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("# 2", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("## 3", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("### 4", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("#. 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse(".# 2", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("#.# 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("? 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("? 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?? 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("#? 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?# 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?## 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("##? 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse(".##? 2", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("#?# 2", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse(".? 2", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("?. 2", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?. 1", 1).get_possible_permutations());
        assert_eq!(2, SpringCollection::parse("??. 1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("? 1,1,1", 1).get_possible_permutations());
        assert_eq!(0, SpringCollection::parse("??? 1,1,1", 1).get_possible_permutations());
    }

    #[test]
    fn part2_copy() {
        //assert_eq!(1, SpringCollection::parse("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", 1).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("???.### 1,1,3", 5).get_possible_permutations());
        assert_eq!(16384, SpringCollection::parse(".??..??...?##. 1,1,3", 5).get_possible_permutations());
        assert_eq!(1, SpringCollection::parse("?#?#?#?#?#?#?#? 1,3,1,6", 5).get_possible_permutations());
        assert_eq!(16, SpringCollection::parse("????.#...#... 4,1,1", 5).get_possible_permutations());
        assert_eq!(2500, SpringCollection::parse("????.######..#####. 1,6,5", 5).get_possible_permutations());
        assert_eq!(506250, SpringCollection::parse("?###???????? 3,2,1", 5).get_possible_permutations());

        let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!(525152, SpringCollection::parse(&input, 5).get_possible_permutations());
    }
}