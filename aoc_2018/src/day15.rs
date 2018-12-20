//Day 15: Beverage Bandits
//
use std::collections::HashSet;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Ord, PartialOrd, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Terrain {
    Wall,
    Empty,
}

impl From<char> for Terrain {
    fn from(terrain: char) -> Self {
        match terrain {
            '#' => Terrain::Wall,
            '.' => Terrain::Empty,
            _ => panic!("invalid terrain: {}", terrain),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Race {
    Elf,
    Goblin,
}

impl From<char> for Race {
    fn from(race: char) -> Self {
        match race {
            'E' => Race::Elf,
            'G' => Race::Goblin,
            _ => panic!("invalid race: {}", race),
        }
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Race::Elf => 'E',
                Race::Goblin => 'G',
            }
        )
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Terrain::Wall => '#',
                Terrain::Empty => '.',
            }
        )
    }
}

#[derive(Clone, Copy)]
pub struct Unit {
    id: usize,
    pos: Point,
    race: Race,
    hp: isize,
}

impl Unit {
    fn new(id: usize, pos: Point, race: Race) -> Self {
        Unit {id, pos, race, hp: 200}
    }

    fn is_enemy(&self, other: Unit) -> bool {
        self.race != other.race
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.race)
    }
}

#[derive(Clone)]
pub struct Cavern {
    map: Vec<Vec<Terrain>>,
    units: Vec<Unit>,
}

impl std::convert::AsRef<Cavern> for Cavern {
    fn as_ref(&self) -> &Cavern {
        &self
    }
}

impl Cavern {
    fn is_empty(&self, pos: Point) -> bool {
        if self.units.iter().any(|u| u.pos == pos && !u.is_dead()) {
            return false;
        }

        self.map[pos.x][pos.y] == Terrain::Empty
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ysize = self.map[0].len();
        let xsize = self.map.len();

        for y in 0..ysize {
            for x in 0..xsize {
                if let Some(unit) = self.units.iter().find(|u| u.pos == Point { x, y }) {
                    write!(f, "{}", unit)?
                } else {
                    write!(f, "{}", self.map[x][y])?
                }
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

#[aoc_generator(day15)]
pub fn input_cavern(input: &str) -> Cavern {
    let mut units = Vec::new();
    let mut map = vec![
        vec![Terrain::Empty; input.lines().count()];
        input.lines().next().unwrap().chars().count()
    ];
    let mut id = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.chars().enumerate() {
            map[x][y] = match col {
                'E' | 'G' => {
                    units.push(Unit::new(id, Point { x, y }, Race::from(col)));
                    id += 1;
                    Terrain::Empty
                }
                _ => Terrain::from(col),
            }
        }
    }

    Cavern { map, units }
}

#[aoc(day15, part1)]
pub fn part1(input: &Cavern) -> usize {
    0
}

#[aoc(day15, part2)]
pub fn part2(input: &Cavern) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "#######\n\
#.G...#\n\
#...EG#\n\
#.#.#G#\n\
#..G#E#\n\
#.....#\n\
#######";

    #[test]
    fn grok_input() {
        assert_eq!(format!("{}", input_cavern(TEST_STR)), format!("{}\n",TEST_STR));
    }

    #[test]
    fn ordering() {

    }
}
