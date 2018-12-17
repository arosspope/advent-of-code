//Day 13: Mine Cart Madness
//
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
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

    fn left(&self) -> Heading {
        let i = *self as usize;
        Heading::from(i.checked_sub(1).unwrap_or(3))
    }

    fn right(&self) -> Heading {
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
    fn new(direction: Heading, pos: Point) -> Cart {
        Cart {
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
                Heading::East | Heading::West => self.direction = self.direction.left(),
                Heading::North | Heading::South => self.direction = self.direction.right(),
            },
            Track::Right => match self.direction {
                Heading::East | Heading::West => self.direction = self.direction.right(),
                Heading::North | Heading::South => self.direction = self.direction.left(),
            },
            Track::Intersection => {
                match self.intersect_choice % 3 {
                    0 => self.direction = self.direction.left(),
                    1 => (), //We go forward in original direction
                    2 => self.direction = self.direction.right(),
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
        // let mut crashed = Vec::new();

        for cart in self.carts.iter_mut() {
            if cart.crashed {
                continue;
            }

            cart.advance();
            cart.turn(&self.map[cart.pos.x][cart.pos.y]); //Turn the cart if required based on the track underneath

            //Do collision checking...
        }

        let mut tmp = self.carts.clone();
        for (i, cart) in self.carts.iter().enumerate() {
            if cart.crashed {
                continue;
            }

            if let Some((x, _)) = self
                .carts
                .iter()
                .enumerate()
                .find(|(j, other)| &i != j && other.pos == cart.pos && !other.crashed)
            {
                tmp[i].crashed = true;
                tmp[x].crashed = true;
            }
        }

        self.carts = tmp;
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

    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.chars().enumerate() {
            map[x][y] = match col {
                '^' | 'v' | '<' | '>' => {
                    carts.push(Cart::new(Heading::new(col), Point { x, y }));
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
            return crash.pos.clone();
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &Mine) -> Point {
    let mut mine = input.clone();
    loop {
        mine.tick();

        if mine.carts.iter().filter(|c| !c.crashed).count() <= 1 {
            let last = mine.carts.iter().find(|c| !c.crashed).unwrap();
            return last.pos.clone();
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
| | | v
\>+</ |
  |   ^
  \<->/"#;

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
}
