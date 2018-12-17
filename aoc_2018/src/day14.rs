//Day 14: Chocolate Charts
//
#[aoc(day14, part1)]
pub fn part1(input: &str) -> String {
    let goal = input.trim().parse::<usize>().unwrap();
    let mut recipes = vec![3, 7];

    let (mut elf1, mut elf2) = (0, 1);

    while recipes.len() < goal + 10 {
        let sum = recipes[elf1] + recipes[elf2];
        let mut digits: Vec<usize> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect();
        recipes.append(&mut digits);

        elf1 = (1 + elf1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (1 + elf2 + recipes[elf2] as usize) % recipes.len();
    }

    recipes[goal..goal + 10]
        .iter()
        .map(|c| format!("{}", c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("5"), "0124515891");
    }

    #[test]
    fn sample2() {
        assert_eq!(part1("18"), "9251071085");
    }

    #[test]
    fn sample3() {
        assert_eq!(part1("2018"), "5941429882");
    }
}
