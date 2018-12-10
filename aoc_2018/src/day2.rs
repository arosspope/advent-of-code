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
    String::from("test")
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
