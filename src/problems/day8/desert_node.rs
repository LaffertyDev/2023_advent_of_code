pub struct DesertNode {
    pub id: [char; 3],

    pub left_id: [char; 3],
    pub right_id: [char; 3]
}

impl DesertNode {
    pub fn parse(line: &str) -> DesertNode {
        // LST = (PVJ, DPR)

        let mut id = line[0..3].chars();
        let mut left_id = line[7..10].chars();
        let mut right_id = line[12..15].chars();

        DesertNode {
            id: [id.next().unwrap(), id.next().unwrap(), id.next().unwrap()],
            left_id: [left_id.next().unwrap(), left_id.next().unwrap(), left_id.next().unwrap()],
            right_id: [right_id.next().unwrap(), right_id.next().unwrap(), right_id.next().unwrap()],
        }
    }

    pub fn is_start_for_ghosts(&self) -> bool {
        self.id[2] == 'A'
    }

    pub fn is_destination_for_ghosts(&self) -> bool {
        self.id[2] == 'Z'
    }
}