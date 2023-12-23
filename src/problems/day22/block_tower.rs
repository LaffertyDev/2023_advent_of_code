#[derive(Copy, Clone, PartialEq, Eq)]
struct Block {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize
}

#[derive(Clone)]
pub struct BlockTower {
    blocks: Vec<Block>
}

impl Block {
    fn parse(line: &str) -> Block {
        let mut splitter = line.split([',', '~']);
        let x1 = splitter.next().unwrap().parse::<usize>().unwrap();
        let y1 = splitter.next().unwrap().parse::<usize>().unwrap();
        let z1 = splitter.next().unwrap().parse::<usize>().unwrap();
        let x2 = splitter.next().unwrap().parse::<usize>().unwrap();
        let y2 = splitter.next().unwrap().parse::<usize>().unwrap();
        let z2 = splitter.next().unwrap().parse::<usize>().unwrap();

        assert!(x1 <= x2);
        assert!(y1 <= y2);
        assert!(z1 <= z2);

        Block {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2
        }
    }

    fn intersects_z_independent(&self, other: &Block) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    fn does_support(&self, other: &Block) -> bool {
        if self.z2 + 1 == other.z1 {
            return self.intersects_z_independent(other);
        }

        return false;
    }
}

impl BlockTower {
    pub fn parse(contents: &str) -> BlockTower {
        // parse bricks and move as low as possible
        let mut blocks = vec![];

        for line in contents.lines().filter(|l| !l.is_empty()) {
            blocks.push(Block::parse(line));
        }

        // now move all blocks down as far as they can
        blocks.sort_by(|b1, b2| b1.z2.cmp(&b2.z2));

        BlockTower::pull_blocks_by_gravity(&mut blocks);
        BlockTower {
            blocks
        }
    }

    fn pull_blocks_by_gravity(blocks: &mut Vec<Block>) -> u64 {
        let mut blocks_that_fell = 0;
        for idx in 0..blocks.len() {
            let mut top_z_collision = 0;
            for down_idx in (0..idx).rev() {
                if blocks[idx].intersects_z_independent(&blocks[down_idx]) {
                    top_z_collision = std::cmp::max(top_z_collision, blocks[down_idx].z2);
                } // else it falls past
            }

            if blocks[idx].z1 != top_z_collision + 1 {
                blocks_that_fell += 1;
            }

            let height = blocks[idx].z2 - blocks[idx].z1;
            blocks[idx].z1 = top_z_collision + 1;
            blocks[idx].z2 = top_z_collision + 1 + height;
        }

        blocks_that_fell
    }

    fn count_blocks_supporting(&self, block: &Block) -> u64 {
        let mut supporting = 0;
        for other_block in self.blocks.iter() {
            if block != other_block && other_block.does_support(block) {
                supporting += 1;
            }
        }

        supporting
    }

    fn get_blocks_supported_by<'a>(&'a self, block: &'a Block) -> Vec<&'a Block> {
        let mut result = vec![];
        for other_block in self.blocks.iter() {
            if block.does_support(other_block) {
                result.push(other_block);
            }
        }

        result
    }

    pub fn count_bricks_that_can_disintegrate(&self) -> u64 {
        let mut result = 0;
        for block in self.blocks.iter() {
            let mut can_disintegrate = true;
            for block_supported_by_me in self.get_blocks_supported_by(block) {
                can_disintegrate &= self.count_blocks_supporting(block_supported_by_me) != 1;
            }

            if can_disintegrate {
                result += 1;
            }
        }

        result
    }

    pub fn determine_max_fall(&self) -> u64 {
        let mut fallen = 0;
        for idx in 0..self.blocks.len() {
            println!("{}/{}", idx, self.blocks.len());
            let mut cloned = self.blocks.to_vec();
            cloned.remove(idx);
            fallen += BlockTower::pull_blocks_by_gravity(&mut cloned);
        }

        fallen
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day22::block_tower::BlockTower;

    #[test]
    fn part1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        let block_tower = BlockTower::parse(input);

        assert_eq!(5, block_tower.count_bricks_that_can_disintegrate());
    }

    #[test]
    fn part2() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        let block_tower = BlockTower::parse(input);

        assert_eq!(7, block_tower.determine_max_fall());
    }
}