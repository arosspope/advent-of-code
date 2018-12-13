//Day 11: Chronal Charge

use std::fmt;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PowerCell (usize, usize, usize); // Of the format (x, y, size)

impl fmt::Display for PowerCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> PowerCell {
    let serial_num: isize = input.lines().next().unwrap().parse().unwrap();

    let mut max: isize = 0;
    let mut max_fuel_cell = PowerCell(0, 0, 3);

    for (x, y) in iproduct!(1..299, 1..299) {
        let fuel_square_power: isize = iproduct!(0..3, 0..3)
            .map(|(i, j)| cell_power(x + i, y + j, serial_num))
            .sum();

        if fuel_square_power > max {
            max = fuel_square_power;
            max_fuel_cell = PowerCell(x, y, 3);
        }
    }

    max_fuel_cell
}

fn cell_power(x: usize, y: usize, serial_num: isize) -> isize {
    let rackid = x + 10;
    let mut power: isize = (rackid * y) as isize;
    power += serial_num;
    power *= rackid as isize;
    power = (power / 100) % 10;
    power - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("18"), PowerCell(33,45,3));
        assert_eq!(part1("42"), PowerCell(21,61,3));
    }

    #[test]
    fn power_calc() {
        assert_eq!(cell_power(3, 5, 8), 4);
    }
}
