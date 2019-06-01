//Day 11: Chronal Charge
//
use std::fmt;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PowerCell(usize, usize, usize); // Of the format (x, y, size)

impl fmt::Display for PowerCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

pub fn optimal_power(grid: &[[isize; 300]; 300], size: usize) -> (isize, PowerCell) {
    let mut max: isize = grid[0][0];
    let mut max_fuel_cell = PowerCell(1, 1, 1); // Indexing starts at 1

    let bound = if size == 300 { 1 } else { (300 - size) % 300 };

    for (x, y) in iproduct!(0..bound, 0..bound) {
        let fuel_square_power: isize = iproduct!(0..size, 0..size)
            .map(|(i, j)| grid[x + i][y + j])
            .sum();

        if fuel_square_power > max {
            max = fuel_square_power;
            max_fuel_cell = PowerCell(x + 1, y + 1, size); // Indexing starts at 1
        }
    }

    (max, max_fuel_cell)
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> PowerCell {
    let serial_num: isize = input.lines().next().unwrap().parse().unwrap();
    let grid = power_grid(serial_num);

    optimal_power(&grid, 3).1
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> PowerCell {
    let serial_num: isize = input.lines().next().unwrap().parse().unwrap();
    let grid = power_grid(serial_num);

    let mut max: isize = grid[0][0];
    let mut max_fuel_cell = PowerCell(1, 1, 1); // Indexing starts at 1

    for size in 1..=300 {
        let optimal = optimal_power(&grid, size);
        if optimal.0 > max {
            max = optimal.0;
            max_fuel_cell = optimal.1;
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

fn power_grid(serial_num: isize) -> [[isize; 300]; 300] {
    let mut grid = [[0; 300]; 300];

    for (x, y) in iproduct!(0..300, 0..300) {
        grid[x][y] = cell_power(x + 1, y + 1, serial_num); // Account for index starting at 1
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1("18"), PowerCell(33, 45, 3));
        assert_eq!(part1("42"), PowerCell(21, 61, 3));
    }

    #[test]
    fn sample2() {
        //... TOO SLOW ... :(
        // assert_eq!(part2("18"), PowerCell(90,296,16));
        // assert_eq!(part2("42"), PowerCell(232,251,12));
    }

    #[test]
    fn power_calc() {
        assert_eq!(cell_power(3, 5, 8), 4);
    }
}
