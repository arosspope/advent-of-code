//Day 11: Chronal Charge
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PowerCell (usize, usize);

impl fmt::Display for PowerCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> PowerCell {
    let serial_num: isize = input.lines().next().unwrap().parse().unwrap();

    let mut max = 0;
    let mut max_fuel_cell: Option<PowerCell> = None;
    for (x, y) in (1..298).zip(1..298) {
        let fuel_square_power: isize = (0..2)
            .zip(0..2)
            .map(|(i, j)| cell_power(x + i, y + j, serial_num))
            .sum();

        if fuel_square_power > max {
            max = fuel_square_power;
            max_fuel_cell = Some(PowerCell(x, y));
        }
    }

    max_fuel_cell.unwrap()
}

fn cell_power(x: usize, y: usize, serial_num: isize) -> isize {
    let rackid = x as isize + 10;
    let mut power: isize = rackid * y as isize;
    power += serial_num as isize;
    power *= rackid;
    power = (power / 100) % 10;
    power - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("57"), PowerCell(122,79));
        assert_eq!(part1("39"), PowerCell(217,196));
        assert_eq!(part1("71"), PowerCell(101,153));
    }
}
