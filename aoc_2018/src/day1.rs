//Day1: Chronal Calibration

#[aoc_generator(day1)]
pub fn input_frequencies(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
   fn sample1() {
       assert_eq!(part2(&vec![1, -1]), 0);
       assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
       assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
       assert_eq!(part2(&vec![7, 7, -2, 7, -4]), 14);
   }
}
