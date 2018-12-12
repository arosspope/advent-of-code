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

pub fn reduce(key: &str, instructions: &mut BTreeMap<&str, BTreeSet<&str>>) -> String {
    if instructions.is_empty() {
        return String::new();
    }

    let mut code = String::new();
    instructions.remove(key);

    for steps in instructions.values_mut() {
        steps.remove(key);
    }

    let free: Vec<&str> = instructions.iter().filter(|(_, v)| v.is_empty()).map(|(&k, _)| k).collect();
    free.iter().for_each(|k| {
        code.push_str(&reduce(k, instructions));
    });


    code.push_str(key);
    code
}


#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    let mut instructions = input_steps(input);

    let start: &str;
    {
        // Find the first alphabetical available step
        start = instructions.iter().find(|(_, v)| v.is_empty()).unwrap().0;
    }

    reduce(start, &mut instructions).chars().rev().collect()
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
