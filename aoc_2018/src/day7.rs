//Day 7: The Sum of Its Parts
extern crate regex;

use day7::regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

pub fn input_steps(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    let re = Regex::new(r"^Step (.{1}?) must be finished before step (.{1}?) can begin\.$").unwrap();

    let mut instructions: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();

    for l in input.lines() {
        let caps: Vec<&str> = re
            .captures(l)
            .unwrap()
            .iter()
            .map(|c| c.unwrap().as_str())
            .collect();

        instructions
            .entry(caps[2])
            .or_insert_with(BTreeSet::new)
            .insert(caps[1]);

        instructions
            .entry(caps[1])
            .or_insert_with(BTreeSet::new);
    }

    instructions
}

pub fn complete(instruction: &str, instructions: &mut BTreeMap<&str, BTreeSet<&str>>) -> String {
    if instructions.is_empty() {
        return String::new();
    } // We've completely processed the entire instuction set

    // Complete the chosen instruction by removing it's entry, and wiping it from the instruction
    // set of other entries
    instructions.remove(instruction);
    for steps in instructions.values_mut() {
        steps.remove(instruction);
    }

    let mut order = String::new(); // The order in which instructions were completed
    let free: Vec<&str> = instructions.iter().filter(|(_, v)| v.is_empty()).map(|(&k, _)| k).collect();
    free.iter().for_each(|k| {
        // Find all the instructions that can now be completed
        order.push_str(&complete(k, instructions)); //It's recursive baby!
    });

    order.push_str(instruction);
    order
}


#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    let mut instructions = input_steps(input);

    let start: &str;
    {
        // Find the first alphabetical available step
        start = instructions.iter().find(|(_, v)| v.is_empty()).unwrap().0;
    }

    complete(start, &mut instructions).chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "Step C must be finished before step A can begin.\n\
                        Step C must be finished before step F can begin.\n\
                        Step A must be finished before step B can begin.\n\
                        Step A must be finished before step D can begin.\n\
                        Step B must be finished before step E can begin.\n\
                        Step D must be finished before step E can begin.\n\
                        Step F must be finished before step E can begin.";

    #[test]
    fn sample1() {
        assert_eq!(part1(&TEST_STR), "CABDFE");
    }
}
