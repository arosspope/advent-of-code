//Day 1: Chronal Calibration
//
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_frequencies(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[isize]) -> isize {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[isize]) -> isize {
    let mut freq = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(freq);

    for num in input.iter().cycle() {
        freq += num;
        if frequencies.contains(&freq) {
            return freq;
        }
        frequencies.insert(freq);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    fn sample1() {
        assert_eq!(part2(&[1, -1]), 0);
        assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
    }
}
