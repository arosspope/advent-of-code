//Day 5: Alchemical Reduction

fn react_polymer(polymer: &str) -> usize {
    let search_iter = (0..26).map(|x| {
        let c = (x + 'a' as u8) as char;
        format!("{}{}", c, c.to_uppercase())
    }).chain((0..26).map(|x| {
        let c = (x + 'a' as u8) as char;
        format!("{}{}", c.to_uppercase(), c)
    }));

    let mut parsed: String = polymer.to_string();

    loop {
        let search_iter = search_iter.clone();
        let mut scanned = false;

        for search in search_iter {
            let tmp = parsed.replace(search.as_str(), "");
            if tmp.len() < parsed.len() {
                scanned = true;
                parsed = String::from(tmp);
            }
        }

        if !scanned {
            break;
        }
    }
    parsed.len()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    react_polymer(input)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let remove_iter = (0..26).map(|x| {
        (x + 'a' as u8) as char
    });

    let mut min = input.len();
    for r in remove_iter {
        let scrubbed = input.replace(r, "").replace(r.to_ascii_uppercase(), "");
        let polymers = react_polymer(&scrubbed);
        if polymers < min {
            min = polymers;
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            part1(&"dabAcCaCBAcCcaDA"),
            10
        );
    }

    #[test]
    fn sample2(){
        assert_eq!(
            part2(&"dabAcCaCBAcCcaDA"),
            4
        );
    }
}
