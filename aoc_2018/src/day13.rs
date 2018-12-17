//Day 13: Mine Cart Madness
//
use std::collections::HashSet;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug, Ord, PartialOrd, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
enum Heading {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Heading {
    fn new(input: char) -> Heading {
        match input {
            '^' => Heading::North,
            '>' => Heading::East,
            'v' => Heading::South,
            '<' => Heading::West,
            _ => panic!("unknown heading: {}", input),
        }
    }

    fn counter_clockwise(&self) -> Heading {
        let i = *self as usize;
        Heading::from(i.checked_sub(1).unwrap_or(3))
    }

    fn clockwise(&self) -> Heading {
        let i = *self as usize;
        Heading::from((i + 1) % 4)
    }
}

impl From<usize> for Heading {
    fn from(direction: usize) -> Self {
        match direction {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => panic!("invalid direction: {}", direction),
        }
    }
}

#[derive(Clone)]
enum Track {
    Horizontal,
    Vertical,
    Intersection,
    Left,
    Right,
    None,
}

impl Track {
    fn new(input: char) -> Track {
        match input {
            '|' | '^' | 'v' => Track::Vertical,
            '-' | '<' | '>' => Track::Horizontal,
            '\\' => Track::Right,
            '/' => Track::Left,
            '+' => Track::Intersection,
            _ => Track::None,
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Track::Vertical => '|',
                Track::Horizontal => '-',
                Track::Intersection => '+',
                Track::Left => '/',
                Track::Right => '\\',
                Track::None => ' ',
            }
        )
    }
}

#[derive(Clone)]
struct Cart {
    id: usize,
    crashed: bool,
    pos: Point,
    direction: Heading,
    intersect_choice: usize,
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.crashed {
            return write!(f, "x");
        }

        write!(
            f,
            "{}",
            match self.direction {
                Heading::North => '^',
                Heading::East => '>',
                Heading::South => 'v',
                Heading::West => '<',
            }
        )
    }
}

impl Cart {
    fn new(id: usize, direction: Heading, pos: Point) -> Cart {
        Cart {
            id,
            crashed: false,
            pos,
            direction,
            intersect_choice: 0,
        }
    }

    fn advance(&mut self) {
        match self.direction {
            Heading::North => self.pos.y -= 1,
            Heading::East => self.pos.x += 1,
            Heading::South => self.pos.y += 1,
            Heading::West => self.pos.x -= 1,
        }
    }

    fn turn(&mut self, track: &Track) {
        match track {
            Track::Left => match self.direction {
                Heading::East | Heading::West => {
                    self.direction = self.direction.counter_clockwise()
                }
                Heading::North | Heading::South => self.direction = self.direction.clockwise(),
            },
            Track::Right => match self.direction {
                Heading::East | Heading::West => self.direction = self.direction.clockwise(),
                Heading::North | Heading::South => {
                    self.direction = self.direction.counter_clockwise()
                }
            },
            Track::Intersection => {
                match self.intersect_choice % 3 {
                    0 => self.direction = self.direction.counter_clockwise(),
                    1 => (), //We go forward in original direction
                    2 => self.direction = self.direction.clockwise(),
                    _ => unreachable!(),
                }
                self.intersect_choice += 1;
            }
            _ => (), // do nothing when the track is horizontal or vertical or none
        }
    }
}

#[derive(Clone)]
pub struct Mine {
    map: Vec<Vec<Track>>,
    carts: Vec<Cart>,
}

impl Mine {
    fn tick(&mut self) {
        self.carts.sort_by_key(|c| c.pos);

        let mut recently_crashed = HashSet::new();
        let mut tmp = self.carts.clone();

        for cart in self.carts.iter_mut().filter(|c| !c.crashed) {
            if recently_crashed.contains(&cart.id) {
                continue;
            }

            cart.advance();
            cart.turn(&self.map[cart.pos.x][cart.pos.y]); //Turn the cart (if required) based on the track underneath

            //Update the tmp copy with the new information
            if let Some(copy) = tmp.iter_mut().find(|c| c.id == cart.id) {
                *copy = cart.clone();
            }

            //Determine if their is a collision with any other carts
            for other in tmp.iter().filter(|o| !o.crashed && o.id != cart.id) {
                if other.pos == cart.pos {
                    recently_crashed.insert(other.id);
                    recently_crashed.insert(cart.id);
                }
            }
        }

        //Update the carts that crashed in this tick
        for id in recently_crashed.iter() {
            if let Some(cart) = self.carts.iter_mut().find(|c| c.id == *id) {
                (*cart).crashed = true;
            }
        }
    }
}

impl fmt::Display for Mine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ysize = self.map[0].len();
        let xsize = self.map.len();

        for y in 0..ysize {
            for x in 0..xsize {
                if let Some(cart) = self.carts.iter().find(|c| c.pos == Point { x, y }) {
                    write!(f, "{}", cart)?
                } else {
                    write!(f, "{}", self.map[x][y])?
                }
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

#[aoc_generator(day13)]
pub fn input_mine(input: &str) -> Mine {
    let mut carts = Vec::new();
    let mut map = vec![
        vec![Track::None; input.lines().count()];
        input.lines().next().unwrap().chars().count()
    ];
    let mut id = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.chars().enumerate() {
            map[x][y] = match col {
                '^' | 'v' | '<' | '>' => {
                    carts.push(Cart::new(id, Heading::new(col), Point { x, y }));
                    id += 1;
                    Track::new(col)
                }
                _ => Track::new(col),
            }
        }
    }

    Mine { map, carts }
}

#[aoc(day13, part1)]
pub fn part1(input: &Mine) -> Point {
    let mut mine = input.clone();
    loop {
        mine.tick();
        if let Some(crash) = mine.carts.iter().find(|c| c.crashed) {
            return crash.pos;
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &Mine) -> Point {
    let mut mine = input.clone();

    loop {
        mine.tick();
        if mine.carts.iter().filter(|c| !c.crashed).count() == 1 {
            let last = mine.carts.iter().find(|c| !c.crashed).unwrap();
            return last.pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    static TEST_STR1: &str = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;

    #[rustfmt::skip]
    static TEST_STR2: &str = r#"/>-<\  
|   |  
| /<+-\
| v | v
\>+</ |
  |   ^
  \<->/"#;

    #[rustfmt::skip]
    static EDGE1: &str = r#"/--->>---\
^        |
\--------/"#;

    #[rustfmt::skip]
    static EDGE2: &str = r#"/<------\
v       |
v       |
\-------/"#;

    #[test]
    fn grok_input() {
        let mine = input_mine(TEST_STR1);
        assert_eq!(format!("{}", mine), format!("{}\n", TEST_STR1));
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_mine(TEST_STR1)), Point { x: 7, y: 3 });
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_mine(TEST_STR2)), Point { x: 6, y: 4 });
    }

    #[test]
    fn edge1() {
        let mut mine = input_mine(EDGE1);
        mine.tick();
        assert_eq!(mine.carts.iter().filter(|c| !c.crashed).count(), 1)
    }

    #[test]
    fn edge2() {
        let mut mine = input_mine(EDGE2);
        mine.tick();
        assert_eq!(mine.carts.iter().filter(|c| !c.crashed).count(), 1)
    }
}
