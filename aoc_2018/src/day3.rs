//Day 3: No Matter How You Slice It
extern crate regex;

use std::collections::HashMap;
use day3::regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

// For a specific claim, find the coordinate positions it occupies
//
fn resolve_occupancy(claim: &Claim) -> Vec<(usize, usize)> {
    let mut occupancy = Vec::new();
    for x in claim.x..(claim.x + claim.w) {
        for y in claim.y..(claim.y + claim.h) {
            occupancy.push((x, y));
        }
    }
    occupancy
}

// Build a hashmap listing the number of overlaps at a specific position
//
fn build_fabric_grid(claims: &[Claim]) -> HashMap<(usize, usize), usize> {
    let mut grid = HashMap::new();

    for claim in claims {
        for (x, y) in resolve_occupancy(claim) {
            let position = grid.entry((x, y)).or_insert(0);
            *position += 1;
        }
    }

    grid
}

#[aoc_generator(day3)]
pub fn input_claims(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    input
        .lines()
        .map(|l| {
            let caps: Vec<&str> = re
                .captures(l)
                .unwrap()
                .iter()
                .map(|c| c.unwrap().as_str())
                .collect();
            Claim {
                id: caps[1].parse().unwrap(),
                x: caps[2].parse().unwrap(),
                y: caps[3].parse().unwrap(),
                w: caps[4].parse().unwrap(),
                h: caps[5].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> usize {
    build_fabric_grid(input)
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Claim]) -> usize {
    let grid = build_fabric_grid(input);

    for claim in input {
        let occupancy = resolve_occupancy(claim);
        if occupancy
            .iter()
            .all(|(x, y)| grid.get(&(*x, *y)).unwrap() == &1)
        {
            return claim.id;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grok_input() {
        let expected = vec![
            Claim {
                id: 1,
                x: 1,
                y: 3,
                w: 4,
                h: 4,
            },
            Claim {
                id: 2,
                x: 3,
                y: 1,
                w: 4,
                h: 4,
            },
            Claim {
                id: 3,
                x: 5,
                y: 5,
                w: 2,
                h: 2,
            },
        ];

        assert_eq!(
            input_claims("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"),
            expected
        );
    }

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&input_claims("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")),
            4
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(&input_claims("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")),
            3
        );
    }
}
