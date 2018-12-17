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

fn ends_with(s1: &[u8], s2: &[u8]) -> Option<usize> {
    if s1.len() > s2.len() {
        if &s1[s1.len() - s2.len()..] == s2 {
            return Some(s1.len() - s2.len());
        }
    }

    if s1.len() - 1 >= s2.len() {
        if &s1[s1.len() - 1 - s2.len()..s1.len() - 1] == s2 {
            return Some(s1.len() - s2.len() - 1);
        }
    }

    None
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let score: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut recipes: Vec<u8> = vec![3, 7];
    let (mut elf1, mut elf2) = (0, 1);

    loop {
        let sum = recipes[elf1] + recipes[elf2];
        let mut digits: Vec<u8> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u8)
            .collect();
        recipes.append(&mut digits);

        elf1 = (1 + elf1 + recipes[elf1] as usize) % recipes.len();
        elf2 = (1 + elf2 + recipes[elf2] as usize) % recipes.len();

        if let Some(num_recipies) = ends_with(&recipes, &score) {
            return num_recipies;
        }
    }
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

    #[test]
    fn sample4() {
        assert_eq!(part2("51589"), 9);
    }

    #[test]
    fn sample5() {
        assert_eq!(part2("01245"), 5);
    }

    #[test]
    fn sample6() {
        assert_eq!(part2("92510"), 18);
    }

    #[test]
    fn sample7() {
        assert_eq!(part2("59414"), 2018);
    }
}
