use std::cmp::Ordering;
use crate::problems::day19::part::{Part, PartRange};

pub enum RuleApplication {
    X(u64, Ordering),
    M(u64, Ordering),
    A(u64, Ordering),
    S(u64, Ordering)
}

impl RuleApplication {
    pub fn new(token: &str, value: u64, comparator: &str) -> Option<RuleApplication> {
        let comparator = match comparator {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => { return None; }
        };
        return match token {
            "x" => Some(RuleApplication::X(value, comparator)),
            "m" => Some(RuleApplication::M(value, comparator)),
            "a" => Some(RuleApplication::A(value, comparator)),
            "s" => Some(RuleApplication::S(value, comparator)),
            _ => None
        };
    }

    pub fn marshall_part_range(&self, previous_part: &PartRange, range: ((u64, u64), bool)) -> (PartRange, bool) {
        let ((min, max), result) = range;
        let mut part = previous_part.clone();
        match self {
            RuleApplication::X(_, _) => {
                part.x_min = min;
                part.x_max = max;
            },
            RuleApplication::M(_, _) => {
                part.m_min = min;
                part.m_max = max;
            },
            RuleApplication::A(_, _) => {
                part.a_min = min;
                part.a_max = max;
            },
            RuleApplication::S(_, _) => {
                part.s_min = min;
                part.s_max = max;
            }
        }

        return (part, result);
    }
}

pub struct Rule {
    applies: Option<RuleApplication>,
    destination: String
}

impl Rule {
    pub fn parse(def: &str) -> Option<Rule> {
        let token = &def[0..=0];
        if def.len() > 2 && (&def[1..=1] == "<" || &def[1..=1] == ">") {
            let mut split = def.split([':', def.chars().nth(1)?]);
            split.next()?; // remove token, we already have it
            let value = split.next()?.parse::<u64>().ok()?;
            let label: String = split.next()?.into();
            let comparator = &def[1..=1];
            return Some(Rule {
                applies: Some(RuleApplication::new(token, value, comparator)?),
                destination: label.into()
            });
        } else {
            // this is just a basic label
            return Some(Rule {
                applies: None,
                destination: def.into()
            });
        }
    }

    pub fn does_part_succeed(&self, part: &Part) -> bool {
        if let Some(applies) = &self.applies {
            match applies {
                RuleApplication::X(val, comp) => {
                    return &part.x.cmp(val) == comp;
                },
                RuleApplication::M(val, comp) => {
                    return &part.m.cmp(val) == comp;
                },
                RuleApplication::A(val, comp) => {
                    return &part.a.cmp(val) == comp;
                },
                RuleApplication::S(val, comp) => {
                    return &part.s.cmp(val) == comp;
                }
            }
        } else {
            return true;
        }
    }

    pub fn compare_xmas(val: u64, range_min: u64, range_max: u64, comp: &Ordering) -> Vec<((u64, u64), bool)> {
        if val < range_min {
            // entire set is greater than the comparator
            return vec![((range_min, range_max), comp == &Ordering::Greater)]
        } else if val > range_max {
            // entire set is less than the comparator
            return vec![((range_min, range_max), comp == &Ordering::Less)]
        } else {
            // we know we straddle a line
            // so part of the range is getting true, part is getting false
            if comp == &Ordering::Less {
                // < 100
                // 0..99, 100..=
                return vec![((range_min, val - 1), true), ((val, range_max), false)];
            } else {
                // > 100
                // 0.100, 101..=
                return vec![((range_min, val), false), ((val + 1, range_max), true)];
            }
        }
    }

    pub fn map_part_range(&self, part_range: &PartRange) -> Vec<(PartRange, bool)> {
        if let Some(applies) = &self.applies {
            let result = match applies {
                RuleApplication::X(val, comp) => {
                    Rule::compare_xmas(*val, part_range.x_min, part_range.x_max, comp)
                        .into_iter().map(|r| applies.marshall_part_range(part_range, r))
                        .collect()
                },
                RuleApplication::M(val, comp) => {
                    Rule::compare_xmas(*val, part_range.m_min, part_range.m_max, comp)
                        .into_iter().map(|r| applies.marshall_part_range(part_range, r))
                        .collect()
                },
                RuleApplication::A(val, comp) => {
                    Rule::compare_xmas(*val, part_range.a_min, part_range.a_max, comp)
                        .into_iter().map(|r| applies.marshall_part_range(part_range, r))
                        .collect()
                },
                RuleApplication::S(val, comp) => {
                    Rule::compare_xmas(*val, part_range.s_min, part_range.s_max, comp)
                        .into_iter().map(|r| applies.marshall_part_range(part_range, r))
                        .collect()
                }
            };

            return result;
        } else {
            return vec![(part_range.clone(), true)];
        }
    }
}

pub struct Workflow {
    label: String,
    rules: Vec<Rule>
}

impl Workflow {
    pub fn parse(line: &str) -> Option<Workflow> {
        // px{a<2006:qkq,m>2090:A,rfg}
        let mut line_parts = line.split(['{', '}']);
        let label = line_parts.next()?;
        let rules = line_parts.next()?.split(',');

        let rules = rules.map(|r| Rule::parse(r).unwrap()).collect();

        Some(Workflow {
            label: label.into(),
            rules
        })
    }

    pub fn is_start(&self) -> bool {
        return self.label == "in";
    }

    pub fn is_workflow(&self, label: &str) -> bool {
        return self.label == label;
    }

    pub fn apply_rules(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.does_part_succeed(part) {
                return rule.destination.clone();
            }
        }

        panic!("A rule did not return a destination :O :O");
    }

    pub fn apply_rules_to_range(&self, part_range: &PartRange) -> Vec<(PartRange, String)> {
        let mut part_range_results = vec![];
        let mut ranges_to_apply_to_rules: Vec<PartRange> = vec![part_range.clone()];
        for rule in &self.rules {
            let mut next_rules = vec![];
            while let Some(range_to_apply) = ranges_to_apply_to_rules.pop() {
                let sub_ranges_with_results = rule.map_part_range(&range_to_apply);
                for (sub_range, did_rule_apply) in sub_ranges_with_results {
                    if did_rule_apply {
                        part_range_results.push((sub_range, rule.destination.clone()));
                    } else {
                        // this range succeeded, so other rules will apply to it
                        next_rules.push(sub_range);
                    }
                }
            }

            ranges_to_apply_to_rules = next_rules;
        }

        if !ranges_to_apply_to_rules.is_empty() {
            panic!("There were leftover ranges not applied to rules...");
        }

        part_range_results
    }
}