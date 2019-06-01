//Day 15: Beverage Bandits
//
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Point {
    y: usize, //Ordering occurs by y
    x: usize,
}

impl Point {
    fn adjacent(&self) -> Vec<Point> {
        let mut adjacent = Vec::new();
        if self.y > 0 {
            adjacent.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x > 0 {
            adjacent.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        adjacent.push(Point {
            x: self.x + 1,
            y: self.y,
        });
        adjacent.push(Point {
            x: self.x,
            y: self.y + 1,
        });

        adjacent
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Unit {
    pos: Point,
    race: Race,
    hp: isize,
    ap: usize,
}

impl Unit {
    fn new(pos: Point, race: Race) -> Self {
        Unit {
            pos,
            race,
            hp: 200,
            ap: 3,
        }
    }

    fn enemy(&self) -> Race {
        match self.race {
            Race::Elf => Race::Goblin,
            Race::Goblin => Race::Elf,
        }
    }

    fn is_enemy(&self, other: &Unit) -> bool {
        other.race == self.enemy()
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
    units: HashMap<usize, Unit>,
    round: usize,
}

impl std::convert::AsRef<Cavern> for Cavern {
    fn as_ref(&self) -> &Cavern {
        &self
    }
}

impl Cavern {
    fn alter_timespace(&mut self, ap: usize) {
        //Update the Elves' attack power
        self.units
            .iter_mut()
            .filter(|(_, v)| v.race == Race::Elf)
            .for_each(|(_, v)| v.ap = ap)
    }

    fn elves(&self) -> usize {
        self.units
            .iter()
            .filter(|(_, v)| v.race == Race::Elf && !v.is_dead())
            .count()
    }

    fn goblins(&self) -> usize {
        self.units
            .iter()
            .filter(|(_, v)| v.race == Race::Goblin && !v.is_dead())
            .count()
    }

    fn total_hp(&self) -> usize {
        self.units
            .iter()
            .filter(|(_, v)| !v.is_dead())
            .map(|(_, v)| v.hp as usize)
            .sum()
    }

    fn attack(&mut self, ap: usize, target: usize) {
        let hp = self.units[&target].hp;
        self.units.get_mut(&target).unwrap().hp = hp - (ap as isize);
    }

    fn adjacent(&self, pos: &Point) -> Vec<Point> {
        pos.adjacent()
            .iter()
            .cloned()
            .filter(|p| p.x < self.map.len() && p.y < self.map[0].len())
            .collect()
    }

    fn attackable_target(&self, unit: usize) -> Option<usize> {
        let u = self.units[&unit];
        self.adjacent(&u.pos)
            .into_iter()
            .filter_map(|adj| {
                if let Some(target) = self
                    .units
                    .iter()
                    .find(|(_, v)| v.pos == adj && u.is_enemy(v) && !v.is_dead())
                {
                    Some(*target.0)
                } else {
                    None
                }
            })
            .min_by_key(|k| self.units[k].hp)
    }

    fn move_unit(&mut self, unit: usize, pos: Point) {
        self.units.get_mut(&unit).unwrap().pos = pos;
    }

    fn move_from(&self, from: Point, target: Race) -> Option<Point> {
        struct Node {
            position: Point,
            previous: Point,
            distance: usize,
        }

        let mut distance_max = self.map.len() * self.map[0].len();

        let mut solutions = Vec::new();
        let mut open_set = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(from);

        for adj in self.adjacent(&from) {
            let has_unit = self.units.iter().any(|(_, v)| v.pos == adj && !v.is_dead());
            if self.map[adj.x][adj.y] == Terrain::Empty && !has_unit {
                visited.insert(adj);
                open_set.push_back(Node {
                    position: adj,
                    previous: adj,
                    distance: 0,
                })
            }
        }

        while let Some(Node {
            position,
            previous,
            distance,
        }) = open_set.pop_front()
        {
            if distance > distance_max {
                break;
            }

            for adj in self.adjacent(&position) {
                if visited.contains(&adj) {
                    continue;
                }

                if let Some(occupant) = self
                    .units
                    .iter()
                    .find(|(_, v)| v.pos == adj && !v.is_dead())
                {
                    if occupant.1.race == target {
                        distance_max = distance;
                        solutions.push((position, previous));
                    }
                } else if self.map[adj.x][adj.y] == Terrain::Empty {
                    visited.insert(adj);
                    open_set.push_back(Node {
                        position: adj,
                        previous,
                        distance: distance + 1,
                    });
                }
            }
        }

        solutions
            .into_iter()
            .min_by_key(|(_, prev)| prev.y)
            .map(|(_, previous)| previous)
    }

    fn round(&mut self) {
        let mut process: Vec<usize> = self
            .units
            .iter()
            .filter(|(_, v)| !v.is_dead())
            .map(|(k, _)| k.clone())
            .collect();
        process.sort_by_key(|k| self.units[k].pos);

        for unit in process {
            if self.goblins() == 0 || self.elves() == 0 {
                return; // One side has wiped the other out - return early, round was not completed
            }

            if self.units[&unit].is_dead() {
                continue; //It was killed by another unit during a previous round
            }

            if let Some(target) = self.attackable_target(unit) {
                self.attack(self.units[&unit].ap, target); // For mordor!
            } else {
                // Search for an enemy to move towards
                //  ...
                if let Some(movement) =
                    self.move_from(self.units[&unit].pos, self.units[&unit].enemy())
                {
                    self.move_unit(unit, movement);
                    if let Some(target) = self.attackable_target(unit) {
                        self.attack(self.units[&unit].ap, target);
                    }
                }
            }
        }

        self.round += 1
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ysize = self.map[0].len();
        let xsize = self.map.len();

        for y in 0..ysize {
            for x in 0..xsize {
                if let Some((_, u)) = self
                    .units
                    .iter()
                    .find(|(_, u)| u.pos == Point { x, y } && !u.is_dead())
                {
                    write!(f, "{}", u)?
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
    let mut units = HashMap::new();
    let mut map = vec![
        vec![Terrain::Empty; input.lines().count()];
        input.lines().next().unwrap().chars().count()
    ];
    let mut id = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.chars().enumerate() {
            map[x][y] = match col {
                'E' | 'G' => {
                    units.insert(id, Unit::new(Point { x, y }, Race::from(col)));
                    id += 1;
                    Terrain::Empty
                }
                _ => Terrain::from(col),
            }
        }
    }

    Cavern {
        map,
        units,
        round: 0,
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &Cavern) -> usize {
    let mut cavern = input.clone();

    loop {
        cavern.round();

        if cavern.elves() == 0 || cavern.goblins() == 0 {
            break;
        }
    }

    cavern.round * cavern.total_hp()
}

#[aoc(day15, part2)]
pub fn part2(input: &Cavern) -> usize {
    let elven_army = input.elves();

    for ap in 4.. {
        let mut cavern = input.clone();
        cavern.alter_timespace(ap); //Give the elves the edge by altering their attack power

        'inner: loop {
            cavern.round();

            if cavern.elves() == 0 || cavern.goblins() == 0 || cavern.elves() < elven_army {
                break 'inner;
            }
        }

        if cavern.elves() < elven_army {
            continue; // The elves were not strong enough to overcome the goblins
        } else {
            return cavern.round * cavern.total_hp();
        }
    }

    unreachable!()
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

    static GAME1: &str = "#######\n\
                          #G..#E#\n\
                          #E#E.E#\n\
                          #G.##.#\n\
                          #...#E#\n\
                          #...E.#\n\
                          #######";

    static GAME2: &str = "#######\n\
                          #.G...#\n\
                          #...EG#\n\
                          #.#.#G#\n\
                          #..G#E#\n\
                          #.....#\n\
                          #######";

    static GAME3: &str = "#########\n\
                          #G......#\n\
                          #.E.#...#\n\
                          #..##..G#\n\
                          #...##..#\n\
                          #...#...#\n\
                          #.G...G.#\n\
                          #.....G.#\n\
                          #########";

    static GAME4: &str = "#######\n\
                          #E..EG#\n\
                          #.#G.E#\n\
                          #E.##E#\n\
                          #G..#.#\n\
                          #..E#.#\n\
                          #######";

    #[test]
    fn grok_input() {
        assert_eq!(
            format!("{}", input_cavern(TEST_STR)),
            format!("{}\n", TEST_STR)
        );
    }

    #[test]
    fn game1() {
        assert_eq!(part1(&input_cavern(GAME1)), 36334);
    }

    #[test]
    fn game2() {
        assert_eq!(part1(&input_cavern(GAME2)), 27730);
    }

    #[test]
    fn game3() {
        assert_eq!(part1(&input_cavern(GAME3)), 18740);
    }

    #[test]
    fn game4() {
        assert_eq!(part1(&input_cavern(GAME4)), 39514);
    }

    #[test]
    fn game5() {
        assert_eq!(part2(&input_cavern(TEST_STR)), 4988);
    }
}
