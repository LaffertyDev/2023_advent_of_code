use std::collections::VecDeque;
use crate::problems::day20::processing_module::{ProcessingModule, Pulse};
use crate::problems::shared::math::lcm;

pub struct MachineInitializer {
    processing_modules: Vec<ProcessingModule>
}

impl MachineInitializer {
    pub fn parse(contents: &str) -> Option<MachineInitializer> {
        let mut processing_modules = vec![ProcessingModule::build_button()];
        for line in contents.lines().filter(|l| !l.is_empty()) {
            let mapped = ProcessingModule::parse(line)?;
            processing_modules.push(mapped);
        }

        // now define all of the inputs
        for idx in 0..processing_modules.len() {
                for output in processing_modules[idx].get_destinations().clone() {
                    let label = processing_modules[idx].label.clone();
                    let module_that_processing_outputs_to = processing_modules.iter_mut().find(|m| m.label == output);
                    if module_that_processing_outputs_to.is_some() {
                        module_that_processing_outputs_to.unwrap().define_input(label.clone());
                }
            }
        }

        Some(MachineInitializer {
            processing_modules
        })
    }

    pub fn count_pulses(&mut self, button_presses: u64) -> u64 {
        let mut high_calls = 0;
        let mut low_calls = 0;
        for idx in 0..button_presses {
            let mut nodes_to_process: VecDeque<(String, String, Pulse)> = VecDeque::new();
            nodes_to_process.push_front(("button".to_string(), "🚀".to_string(), Pulse::High));
            while let Some(process) = nodes_to_process.pop_front() {
                let (consumer_label, emitter, signal) = process;

                if let Some(node) = self.processing_modules.iter_mut().find(|n| n.label == consumer_label) {
                    if let Some(node_output) = node.take_pulse(&emitter, signal) {
                        for destination in node.get_destinations() {
                            if node_output == Pulse::High {
                                high_calls += 1;
                            } else {
                                if destination == "rx" {
                                    println!("Found it: {}", idx + 1);
                                }
                                low_calls += 1;
                            }
                            nodes_to_process.push_back((destination.clone(), consumer_label.clone(), node_output.clone()));
                        }
                    }
                }
            }
        }

        low_calls * high_calls
    }

    pub fn get_components_that_point_to(&self, label: &str) -> Vec<String> {
        let precursor_to_rx: Vec<String> = self.processing_modules.iter().filter(|m| m.get_destinations().contains(&label.to_owned())).map(|m| m.label.clone()).collect();
        return precursor_to_rx;
    }

    pub fn get_presses_to_enable_rx(&mut self) -> u64 {
        let final_conjunction = &self.get_components_that_point_to("rx")[0];
        // all nodes that go to final component must be zero
        let components_that_feed_conjunction = self.get_components_that_point_to(final_conjunction);
        let mut components_with_press: Vec<(String, Option<usize>)> = components_that_feed_conjunction.iter().map(|c| (c.clone(), None)).collect();
        let mut presses = 0;
        loop {
            let mut nodes_to_process: VecDeque<(String, String, Pulse)> = VecDeque::new();
            nodes_to_process.push_front(("button".to_string(), "🚀".to_string(), Pulse::High));
            presses += 1;

            while let Some(process) = nodes_to_process.pop_front() {
                let (consumer_label, emitter, signal) = process;

                if components_with_press.iter().all(|(_, p)| p.is_some()) {
                    let mut result = 1;
                    for (_, p) in components_with_press {
                        result = lcm(result, p.unwrap());
                    }

                    return result as u64;
                }

                if let Some(node) = self.processing_modules.iter_mut().find(|n| n.label == consumer_label) {
                    if let Some(node_output) = node.take_pulse(&emitter, signal) {
                        if components_that_feed_conjunction.contains(&node.label) && node_output == Pulse::High {
                            let (_, p) = components_with_press.iter_mut().find(|(c, _)| c == &node.label).unwrap();
                            if let None = p {
                                *p = Some(presses);
                            }
                        }
                        for destination in node.get_destinations() {
                            nodes_to_process.push_back((destination.clone(), consumer_label.clone(), node_output.clone()));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day20::machine_initializer::MachineInitializer;

    #[test]
    fn part1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let mut machine = MachineInitializer::parse(input).unwrap();
        assert_eq!(32000000, machine.count_pulses(1000));
    }

    #[test]
    fn part1b() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        let mut machine = MachineInitializer::parse(input).unwrap();
        assert_eq!(11687500, machine.count_pulses(1000));
    }
}