//Day 11: Chronal Charge

#[aoc_generator(day11)]
pub fn input_serialid(input: &str) -> usize {
    input.lines().next().unwrap().parse().unwrap()
}

#[aoc(day11, part1)]
pub fn part1(input: &usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "5432\n";

    #[test]
    fn grok_input() {
        assert_eq!(input_serialid(TEST_STR), 5432);
    }
}
