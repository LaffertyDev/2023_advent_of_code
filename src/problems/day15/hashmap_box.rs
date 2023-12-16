use crate::problems::day15::hash_initializer::determine_hash;

#[derive(Clone)]
struct Lens {
    label: String,
    lens_type: u32
}

#[derive(Clone)]
pub struct HashmapBox {
    lenses: Vec<Lens>
}

impl HashmapBox {
    fn new() -> HashmapBox {
        HashmapBox {
            lenses: vec![]
        }
    }
    fn remove_lens(&mut self, label: String) {
        for index in 0..self.lenses.len() {
            if self.lenses[index].label == label {
                self.lenses.remove(index);
                return;
            }
        }
    }

    fn add_or_update_lens(&mut self, label: String, lens_type: u32) {
        for index in 0..self.lenses.len() {
            if self.lenses[index].label == label {
                self.lenses[index].lens_type = lens_type;
                return;
            }
        }

        self.lenses.push(Lens {
            label,
            lens_type
        })
    }

    fn compute_focus(&self, box_id: u32) -> u32 {
        let mut focus = 0;
        for (idx, lens) in self.lenses.iter().enumerate() {
            focus += (idx as u32 + 1) * lens.lens_type * (box_id + 1)
        }

        focus
    }

    pub fn execute(instructions: &str) -> u32 {
        let mut boxes = vec![HashmapBox::new(); 256];

        for sequence in instructions.split(',') {
            let mut lens_id = None;
            let mut operation = None;
            let mut operation_idx = 0;
            for (idx, c) in sequence.chars().enumerate() {
                match c {
                    '-' => {
                        operation = Some('-');
                        operation_idx = idx;

                    },
                    '=' => {
                        operation = Some('=');
                        operation_idx = idx;
                    },
                    data => {
                        if data.is_numeric() {
                            lens_id = data.to_digit(10);
                        }
                    }
                }
            }

            let hash = determine_hash(&sequence[0..operation_idx]);
            let label = sequence[0..operation_idx].to_string();
            let lens_box = &mut boxes[hash];

            let operation = operation.unwrap();
            if operation == '-' {
                lens_box.remove_lens(label);
            } else {
                let lens_id = lens_id.unwrap();
                lens_box.add_or_update_lens(label, lens_id);
            }
        }

        let mut result = 0;
        for (idx, lens_box) in boxes.iter().enumerate() {
            result += lens_box.compute_focus(idx as u32);
        }
        result
    }
}