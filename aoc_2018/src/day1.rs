//Day1: Chronal Calibration

#[aoc_generator(day1)]
pub fn input_frequencies(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    1
}
