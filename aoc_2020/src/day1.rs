//Day 1: Report Repair
//

#[aoc_generator(day1)]
pub fn input_frequencies(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[isize]) -> isize {
    for (pos, i) in input.iter().enumerate() {
        if *i >= 2020 {
            continue;
        }

        for j in input.iter().skip(pos) {
            if *j >= 2020 {
                continue;
            }

            if i + j == 2020 {
                return i * j
            }
        }

    }

    -1
}

#[aoc(day1, part2)]
pub fn part2(input: &[isize]) -> isize {
    for (posi, i) in input.iter().enumerate() {
        if *i >= 2020 {
            continue;
        }

        for (posj, j) in input.iter().skip(posi).enumerate() {
            let ijsum = i + j;

            if *j >= 2020 || ijsum >= 2020 {
                continue;
            }

            for k in input.iter().skip(posj) {
                if *k >= 2020 || k + ijsum > 2020 {
                    continue;
                }

                if k + ijsum == 2020 {
                    return i * j * k
                }
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950)
    }
}
