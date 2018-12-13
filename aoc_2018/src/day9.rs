//Day 7: The Sum of Its Parts
extern crate regex;

use std::fmt;
use day9::regex::Regex;
use std::collections::{HashMap};

pub struct GameInfo {
    players: usize,
    last_marble: usize,
}

impl std::convert::AsRef<GameInfo> for GameInfo {
    fn as_ref(&self) -> &GameInfo {
        &self
    }
}

#[aoc_generator(day9)]
pub fn input_gameinfo(input: &str) -> GameInfo {
    let re =
        Regex::new(r"^(.*) players; last marble is worth (.*) points$").unwrap();

    let caps: Vec<&str> = re
        .captures(input)
        .unwrap()
        .iter()
        .map(|c| c.unwrap().as_str())
        .collect();

    GameInfo{ players: caps[1].parse().unwrap(), last_marble: caps[2].parse().unwrap() }
}

#[aoc(day9, part1)]
pub fn part1(input: &GameInfo) -> usize {
    let mut marbles = Vec::with_capacity(input.last_marble);
    marbles.push(0);

    let mut circle = Circle { marbles: marbles, current: 0, length: 1 };
    circle.play_game(input.players, input.last_marble)
}

#[aoc(day9, part2)]
pub fn part2(input: &GameInfo) -> usize {
    let mut marbles = Vec::with_capacity(input.last_marble * 100);
    marbles.push(0);

    let mut circle = Circle { marbles: marbles, current: 0, length: 1 };
    circle.play_game(input.players, input.last_marble * 100)
}

#[derive(Default, PartialEq)]
pub struct Circle {
    marbles: Vec<usize>,    // The list of marbles in the circle
    current: usize,         // Index of the current marble
    length: usize,          // The artificial length of the Circle
}

impl Circle {
    pub fn clockwise(&self, i: usize, turns: usize) -> usize {
        (i + turns) % self.length
    }

    pub fn counter_clockwise(&self, i: usize, turns: usize) -> usize {
        let mut tmp = i;
        for _ in 0..turns {
            tmp = tmp.checked_sub(1).unwrap_or(self.length - 1);
        }

        tmp
    }

    pub fn play_game(&mut self, players: usize, last_marble: usize) -> usize {
        let mut scores: HashMap<usize, usize> = HashMap::new();

        let players = 1..=players;
        let marbles = 1..=last_marble;

        for (player, marble) in players.cycle().zip(marbles) {
            if marble % 23 == 0 {
                let score = scores.entry(player).or_insert(0);
                *score += marble; // Player adds the marble in play to their score

                // Remove the marble 7 positions counter clockwise, and make the next marble the current
                self.current = self.counter_clockwise(self.current, 7);

                *score += self.marbles.remove(self.current);
                self.length -= 1;
                continue; //On to the next round!
            }

            // Add the current marble dependent on some conditions
            self.length += 1;
            if self.length < 4 {
                self.current = self.clockwise(self.current, 3);
            } else if self.current == self.length - 2 {
                self.current = self.clockwise(self.current, 3);
            } else {
                self.current = self.clockwise(self.current, 2);
            }

            self.marbles.insert(self.current, marble);
        }


        *scores.values().max().unwrap()
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.length {
            if i == self.current {
                write!(f, "({}) ", self.marbles[i])?;
            } else {
                write!(f, "{} ", self.marbles[i])?;
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "9 players; last marble is worth 25 points";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_gameinfo(TEST_STR)), 32);
    }
}
