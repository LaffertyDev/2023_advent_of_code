use crate::problems::day19::part::Part;
use crate::problems::day19::rule::Workflow;

pub struct RulesEngine {
    workflows: Vec<Workflow>,
    parts: Vec<Part>
}

impl RulesEngine {
    pub fn parse(contents: &str) -> Option<RulesEngine> {
        let mut workflows = vec![];
        let mut parts = vec![];

        let mut found_parts = false;
        for line in contents.lines() {
            if line.is_empty() {
                found_parts = true;
                continue;
            }

            if found_parts {
                let part = Part::parse(line)?;
                parts.push(part);
            } else {
                let workflow = Workflow::parse(line)?;
                workflows.push(workflow)
            }
        }

        Some(RulesEngine {
            workflows,
            parts
        })
    }

    pub fn apply_rules(&self) -> u64 {
        let mut accepted_parts = vec![];
        let mut rejected_parts = vec![];

        let starting_workflow = self.workflows.iter().find(|w| w.is_start()).unwrap();
        for part in &self.parts {
            let mut active_workflow = Some(starting_workflow);
            while let Some(workflow) = active_workflow {
                let destination = workflow.apply_rules(part);
                if destination == "R" {
                    rejected_parts.push(part);
                    active_workflow = None;
                } else if destination == "A" {
                    accepted_parts.push(part);
                    active_workflow = None;
                } else {
                    active_workflow = self.workflows.iter().find(|w| w.is_workflow(&destination))
                }
            }
        }

        accepted_parts.iter().map(|ap| ap.x + ap.m + ap.a + ap.s).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day19::rules_engine::RulesEngine;

    #[test]
    fn part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let engine = RulesEngine::parse(input).unwrap();
        assert_eq!(19114, engine.apply_rules())
    }
}