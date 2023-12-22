pub enum ProcessingModuleType {
    Button,

    FlipFlop(bool),

    // Remember a pulse from all of their connections
    // If all pulses connecting to it are HIGH, it sends a LOW pulse
    // otherwise, it sends a HIGH pulse
    // (its a NAND gate)
    Conjunction,

    // Receives a pulse, transmits it to all destinations
    Broadcast
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pulse {
    High,
    Low
}

// Processing modules is a BFS

pub struct ProcessingModule {
    pub label: String,
    processing_module_type: ProcessingModuleType,
    destinations: Vec<String>,
    inputs: Vec<(String, Pulse)>
}

impl ProcessingModule {

    pub fn build_button() -> ProcessingModule {
        ProcessingModule {
            label: "button".into(),
            processing_module_type: ProcessingModuleType::Button,
            destinations: vec!["broadcaster".into()],
            inputs: vec![]
        }
    }
    pub fn parse(line: &str) -> Option<ProcessingModule> {
        let mut split = line.split(" -> ");
        let definition: &str = split.next()?;

        let module_type: ProcessingModuleType;
        let module_label: String;
        if definition == "broadcaster" {
            module_type = ProcessingModuleType::Broadcast;
            module_label = "broadcaster".into();
        } else {
            module_type = match &definition[0..=0]  {
                "%" => ProcessingModuleType::FlipFlop(false),
                "&" => ProcessingModuleType::Conjunction,
                _ => { return None; }
            };

            module_label = definition[1..].into();
        }

        let destination_labels: &str = split.next()?;
        let destination_labels = destination_labels.split(',').map(|s| s.trim().into()).collect();
        Some(ProcessingModule {
            label: module_label,
            processing_module_type: module_type,
            destinations: destination_labels,
            inputs: vec![]
        })
    }

    pub fn define_input(&mut self, emitter: String) {
        self.inputs.push((emitter, Pulse::Low));
    }

    pub fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    pub fn take_pulse(&mut self, emitter: &str, pulse: Pulse) -> Option<Pulse> {
        match &self.processing_module_type {
            // Push a button, a LOW signal is sent to the broadcast module
            ProcessingModuleType::Button => {
                return Some(Pulse::Low);
            },
            //
            ProcessingModuleType::Broadcast => {
                // just re-emits its input
                return Some(pulse);
            },
            ProcessingModuleType::FlipFlop(currently_on) => {
                if pulse == Pulse::High {
                    // high pulses mean we don't do anything, regardless if we are on or off
                    return None;
                } else {
                    // we swap states and emit a pulse
                    let result_pulse = if *currently_on { Pulse::Low } else { Pulse::High };
                    self.processing_module_type = ProcessingModuleType::FlipFlop(!currently_on);
                    return Some(result_pulse);
                }
            },
            ProcessingModuleType::Conjunction => {
                let mut all_high = true;
                for input in self.inputs.iter_mut() {
                    let (label, prev_pulse) = input;
                    if label == emitter {
                        *prev_pulse = pulse
                    }
                    all_high &= prev_pulse == &Pulse::High;
                }

                if all_high {
                    return Some(Pulse::Low);
                } else {
                    return Some(Pulse::High);
                }
            }
        }
    }
}