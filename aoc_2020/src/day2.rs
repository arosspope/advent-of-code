//Day 2: Password Philosophy
//

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
    character: char,
    max: usize,
    min: usize,
}

#[derive(Debug, PartialEq)]
pub struct PasswordEntry {
    policy: PasswordPolicy,
    password: String
}


#[aoc_generator(day2)]
pub fn input_passwords(input: &str) -> Vec<PasswordEntry> {
    input.lines().map(|l| PasswordEntry {
        policy: PasswordPolicy {
            character: l.split(" ").nth(1).unwrap().chars().nth(0).unwrap(),
            min: l.split(" ").nth(0).unwrap().split('-').nth(0).unwrap().parse().unwrap(),
            max: l.split(" ").nth(0).unwrap().split('-').nth(1).unwrap().parse().unwrap(),
        },
        password: l.split(" ").nth(2).unwrap().to_string(),

        // String::from("abc"),
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[PasswordEntry]) -> isize {
    let mut valid = 0;

    for entry in input.iter() {
        let count = entry.password.matches(entry.policy.character).count();
        if count >= entry.policy.min && count <= entry.policy.max {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &[PasswordEntry]) -> isize {
    let mut valid = 0;

    for entry in input.iter() {
        let pos0_matches = entry.password.chars().nth(entry.policy.min - 1).unwrap() == entry.policy.character;
        let pos1_matches = entry.password.chars().nth(entry.policy.max - 1).unwrap() == entry.policy.character;

        if pos0_matches ^ pos1_matches {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, input_passwords};

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_passwords(&"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")), 1)
    }
}
