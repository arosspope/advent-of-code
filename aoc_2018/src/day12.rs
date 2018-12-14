//Day 12: Subterranean Sustainability
extern crate regex;

use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pot {
    Plant,
    Empty,
}

impl Pot {
    fn parse(input: &str) -> Vec<Pot> {
        input
            .chars()
            .map(|c| match c {
                '#' => Pot::Plant,
                '.' => Pot::Empty,
                _ => panic!("unknown pot state"),
            })
            .collect()
    }
}

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pot::Plant => write!(f, "#"),
            Pot::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Note {
    // The puzzle input -> Each note contains a configuration and the result in the next generation
    configuration: Vec<Pot>,
    result: Pot,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.configuration.iter().for_each(|p| {
            write!(f, "{}", p).unwrap();
        });
        write!(f, " => ")?;
        write!(f, "{}", self.result)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Garden {
    pots: Vec<Pot>,
    generation: usize,
    notes: Vec<Note>,
}

impl fmt::Display for Garden {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0: ")?;
        self.pots.iter().for_each(|p| {
            write!(f, "{}", p).unwrap();
        });
        writeln!(f, "\n")?;
        self.notes.iter().for_each(|n| {
            writeln!(f, "{}", n).unwrap();
        });
        Ok(())
    }
}

#[aoc_generator(day12)]
pub fn input_garden(input: &str) -> Garden {
    let initial = input.lines().next().unwrap().replace("initial state: ", "");
    let pots: Vec<Pot> = Pot::parse(&initial);

    let re = Regex::new(r"^(?P<from>[#.]{5}) => (?P<to>[#.])$").unwrap();

    let notes: Vec<Note> = input
        .lines()
        .skip(2)
        .map(|l| {
            let scrub = l.replace(" ", "");
            let read: Vec<&str> = scrub.split("=>").collect();
            Note {
                configuration: Pot::parse(read[0]),
                result: Pot::parse(read[1])[0].clone(),
            }
        })
        .collect();

    Garden {
        pots,
        generation: 0,
        notes,
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Garden) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "initial state: #..#.#..##......###...###\n\n\
                             ...## => #\n\
                             ..#.. => #\n\
                             .#... => #\n\
                             .#.#. => #\n\
                             .#.## => #\n\
                             .##.. => #\n\
                             .#### => #\n\
                             #.#.# => #\n\
                             #.### => #\n\
                             ##.#. => #\n\
                             ##.## => #\n\
                             ###.. => #\n\
                             ###.# => #\n\
                             ####. => #\n";

    #[test]
    fn sample1() {
        let garden = input_garden(TEST_STR);
        let expected = TEST_STR.replace("initial state", "0");
        assert_eq!(format!("{}", garden), expected);
    }
}
