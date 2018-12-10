//Day 2: Inventory Management System
use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut twos, mut threes) = (0, 0);

    for id in input.lines() {
        let mut frequency_map = HashMap::new();
        for c in id.chars() {
            let freq = frequency_map.entry(c).or_insert(0);
            *freq += 1;
        }

        if frequency_map.iter().any(|(_, &count)| count == 2) {
            twos += 1;
        }

        if frequency_map.iter().any(|(_, &count)| count == 3) {
            threes += 1;
        }
    }

    twos * threes
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    for (pos, id1) in input.lines().enumerate() {
        // For each id, search through the rest of the list for similarity
        for id2 in input.lines().skip(pos) {
            let num_different_chars = id1
                .chars()
                .zip(id2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();

            if num_different_chars == 1 {
                return id1
                    .chars()
                    .zip(id2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c, _)| c)
                    .collect();
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&"abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
            12
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(&"abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"),
            "fgij"
        );
    }
}
