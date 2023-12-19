use std::cmp::Ordering;
use crate::problems::day19::part::Part;

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
}