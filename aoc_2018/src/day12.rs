//Day 12: Subterranean Sustainability
//
// Run with `RUST_LOG=debug` to get debug statments for garden visualisation.
//
use std::collections::BTreeMap;
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

    fn has_plant(&self) -> bool {
        self == &Pot::Plant
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

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Garden {
    pots: BTreeMap<isize, Pot>,
    generation: usize,
    notes: Vec<Note>,
}

impl std::convert::AsRef<Garden> for Garden {
    fn as_ref(&self) -> &Garden {
        &self
    }
}

impl Garden {
    fn grow(&mut self) {
        self.expand(); // Make sure there is some room to grow!
        let snapshot: Vec<Pot> = self.pots.iter().map(|(_, v)| v.clone()).collect();

        for (i, (k, v)) in self
            .pots
            .iter_mut()
            .enumerate()
            .skip(2)
            .take(snapshot.len() - 4)
        {
            if let Some(rule) = self
                .notes
                .iter()
                .find(|n| n.configuration == &snapshot[i - 2..=i + 2])
            {
                *v = rule.result.clone();
            } else {
                *v = Pot::Empty;
                warn!("Gen({}) no rule found for pot {}", self.generation, k);
            }
        }

        self.generation += 1;
    }

    fn expand(&mut self) {
        // Checks if the edges of the pot line needs to be expanded
        // to allow room for growth
        let expand_f = self.pots.iter().take(5).any(|(_, p)| p == &Pot::Plant);
        let expand_b = self
            .pots
            .iter()
            .rev()
            .take(5)
            .any(|(_, p)| p == &Pot::Plant);

        if expand_f {
            let start = *self.pots.iter().next().unwrap().0;
            (1..=5).for_each(|i| {
                self.pots.insert(start - i, Pot::Empty);
            });
        }

        if expand_b {
            let end = *self.pots.iter().rev().next().unwrap().0;
            (1..=5).for_each(|i| {
                self.pots.insert(end + i, Pot::Empty);
            });
        }
    }

    fn sum_pots(&self) -> isize {
        //Add the pot numbers containing plants togher
        self.pots
            .iter()
            .filter(|(_, p)| p.has_plant())
            .map(|(k, _)| k)
            .sum()
    }
}

impl fmt::Display for Garden {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.generation)?;
        self.pots.iter().for_each(|p| {
            write!(f, "{}", p.1).unwrap();
        });
        Ok(())
    }
}

impl fmt::Debug for Garden {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}\n", self)?;
        self.notes.iter().for_each(|n| {
            //For debug display, also include the notes
            writeln!(f, "{}", n).unwrap();
        });
        Ok(())
    }
}

#[aoc_generator(day12)]
pub fn input_garden(input: &str) -> Garden {
    let initial = input.lines().next().unwrap().replace("initial state: ", "");
    let tmp = Pot::parse(&initial);
    let pots: BTreeMap<isize, Pot> = (0..tmp.len() as isize).zip(tmp.into_iter()).collect();

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
pub fn part1(input: &Garden) -> isize {
    let mut garden = input.clone();

    for _ in 0..20 {
        garden.grow();
        debug!("{}", garden);
    }

    garden.sum_pots()
}

#[aoc(day12, part2)]
pub fn part2(input: &Garden) -> usize {
    const TARGET: usize = 50_000_000_000;

    let mut garden = input.clone();
    let mut diff: usize = 0;
    let mut pattern: usize = 0;
    let mut last = 0;

    loop {
        garden.grow();

        let sum = garden.sum_pots();
        let ds = (sum - last) as usize;

        if ds == diff {
            pattern += 1;
        } else {
            pattern = 0;
        }

        if pattern > 10 {
            println!(
                "Converged to +{} per gen after {} generations",
                diff, garden.generation
            );
            break;
        }

        diff = ds;
        last = sum;
    }

    diff * (TARGET - garden.generation) + garden.sum_pots() as usize
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

    static GROW_STR: &str = "1: .....#...#....#.....#..#..#..#.....\n\
                             2: .....##..##...##....#..#..#..##....\n\
                             3: ....#.#...#..#.#....#..#..#...#.........\n\
                             4: ..........#.#..#...#.#...#..#..##..##........";

    #[test]
    fn sample1() {
        let garden = input_garden(TEST_STR);
        let expected = TEST_STR.replace("initial state", "0");
        assert_eq!(format!("{:?}", garden), expected);
    }

    #[test]
    fn growing() {
        let mut garden = input_garden(TEST_STR);
        for line in GROW_STR.lines() {
            garden.grow();
            assert_eq!(format!("{}", garden), line);
        }
    }

    #[test]
    fn sum_pots() {
        let mut garden = input_garden(TEST_STR);
        for _ in 0..20 {
            garden.grow();
        }

        assert_eq!(garden.sum_pots(), 325);
    }
}
