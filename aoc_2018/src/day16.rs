//Day 16: Chronal Classification
//
extern crate regex;

use crate::day9::regex::Regex;
use std::collections::{HashMap, VecDeque};

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
    let re = Regex::new(r"^(.*) players; last marble is worth (.*) points$").unwrap();

    let caps: Vec<&str> = re
        .captures(input)
        .unwrap()
        .iter()
        .map(|c| c.unwrap().as_str())
        .collect();

    GameInfo {
        players: caps[1].parse().unwrap(),
        last_marble: caps[2].parse().unwrap(),
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &GameInfo) -> usize {
    let mut marbles = VecDeque::with_capacity(input.last_marble);
    marbles.push_back(0);

    let mut circle = Circle { marbles };
    circle.play_game(input.players, input.last_marble)
}

#[aoc(day9, part2)]
pub fn part2(input: &GameInfo) -> usize {
    let mut marbles = VecDeque::with_capacity(input.last_marble * 100);
    marbles.push_back(0);

    let mut circle = Circle { marbles };
    circle.play_game(input.players, input.last_marble * 100)
}

#[derive(Default, PartialEq)]
pub struct Circle {
    marbles: VecDeque<usize>, // The list of marbles in the circle (front of queue being the first marble)
}

impl Circle {
    pub fn clockwise(&mut self, turns: usize) {
        for _ in 0..turns {
            let popped = self.marbles.pop_front().unwrap();
            self.marbles.push_back(popped);
        }
    }

    pub fn counter_clockwise(&mut self, turns: usize) {
        for _ in 0..turns {
            let popped = self.marbles.pop_back().unwrap();
            self.marbles.push_front(popped);
        }
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
                self.counter_clockwise(7);
                *score += self.marbles.pop_front().unwrap();
                continue; //On to the next round!
            }

            self.clockwise(2);
            self.marbles.push_front(marble);
        }

        *scores.values().max().unwrap()
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
