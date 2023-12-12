use crate::problems::day8::camel_direction::CamelDirection;
use crate::problems::day8::desert_node::DesertNode;

pub struct CamelMap {
    directions: Vec<CamelDirection>,
    nodes: Vec<DesertNode>
}

impl CamelMap {
    pub fn parse(map_data: &str) -> CamelMap {
        let mut lines = map_data.lines().filter(|l| !l.is_empty());
        let directions: Vec<CamelDirection> = lines.next().unwrap().chars().map(CamelDirection::parse).collect();
        let nodes: Vec<DesertNode> = lines.map(DesertNode::parse).collect();

        CamelMap {
            directions,
            nodes
        }
    }

    pub fn get_steps_to_zzz(&self) -> u32 {
        let mut current_node = self.nodes.iter().find(|n| n.id == ['A', 'A', 'A']).unwrap();
        let mut steps = 0;
        let mut direction_index = 0;
        while current_node.id != ['Z', 'Z', 'Z'] {
            let direction = &self.directions[direction_index];
            match direction {
                CamelDirection::Left => {
                    current_node = self.nodes.iter().find(|n| current_node.left_id == n.id).unwrap();
                },
                CamelDirection::Right => {
                    current_node = self.nodes.iter().find(|n| current_node.right_id == n.id).unwrap();
                }
            }

            direction_index = (direction_index + 1) % self.directions.len();
            steps += 1;
        }

        steps
    }

    pub fn count_steps_to_end_for_node(&self, node: &DesertNode) -> u64 {
        let mut current_node = node;
        let mut steps = 0;
        let mut direction_index = 0;
        while !current_node.is_destination_for_ghosts() {
            let direction = &self.directions[direction_index];
            match direction {
                CamelDirection::Left => {
                    current_node = self.nodes.iter().find(|n| current_node.left_id == n.id).unwrap();
                },
                CamelDirection::Right => {
                    current_node = self.nodes.iter().find(|n| current_node.right_id == n.id).unwrap();
                }
            }

            direction_index = (direction_index + 1) % self.directions.len();
            steps += 1;
        }

        steps
    }

    pub fn get_steps_to_zzz_ghost_mode(&self) -> u64 {
        // https://en.wikipedia.org/wiki/Chinese_remainder_theorem for alternative approach from reddit
        // https://en.wikipedia.org/wiki/Least_common_multiple for LCM
        // https://en.wikipedia.org/wiki/Euclidean_algorithm for GCD
        let current_nodes: Vec<&DesertNode> = self.nodes.iter().filter(|n| n.is_start_for_ghosts()).collect();
        let step_counts_for_nodes: Vec<u64> = current_nodes.iter().map(|cn| self.count_steps_to_end_for_node(cn)).collect();
        println!("{:?}", step_counts_for_nodes);
        // 277 nodes
        // upper bound is 16,617,338,012,572,747,975
        // 16897, 16343, 21883, 13019, 14681, 20221

        // 61, 59, 79, 47, 53, 73
        //

        // todo -- write algorithm that computes the LCM between a set of numbers
        // i used wolfram to do it after getting the cycle counts
        // i computed upper bound... but reddit helped point out that all of these are multiples of the direction input
            // (Which kinda feels bad, I would not have sus'd that out myself)
        // math analysis problems are boring (to non math people like me)

        return step_counts_for_nodes.iter().product::<u64>() * self.directions.len() as u64;
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day8::camel_map::CamelMap;

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let map = CamelMap::parse(input);
        assert_eq!(2, map.get_steps_to_zzz());
    }
}