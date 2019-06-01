//Day 6: Chronal Coordinates
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Coordinate {
    fn manhattan_dist(&self, other: &Coordinate) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
    locations: Vec<Coordinate>, //All locations read from file
    finite: Vec<Coordinate>,    //All finite locations (within the bounds of the grid)
    bounds: Bounds,             //Coordiantes that cosntrain our grid
}

impl Grid {
    fn new(locations: Vec<Coordinate>) -> Grid {
        //Find the bounding planes
        let min_x = locations.iter().min_by_key(|coord| coord.x).unwrap().x;
        let max_x = locations.iter().max_by_key(|coord| coord.x).unwrap().x;
        let min_y = locations.iter().min_by_key(|coord| coord.y).unwrap().y;
        let max_y = locations.iter().max_by_key(|coord| coord.y).unwrap().y;
        //Find all the finite locations
        let finite: Vec<Coordinate> = locations
            .iter()
            .cloned()
            .filter(|coord| {
                (coord.x > min_x && coord.x < max_x) && (coord.y > min_y && coord.y < max_y)
            })
            .collect();

        Grid {
            locations,
            finite,
            bounds: Bounds {
                min_x,
                max_x,
                min_y,
                max_y,
            },
        }
    }
}

#[aoc_generator(day6)]
pub fn input_coordiantes(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            let coords: Vec<usize> = l.split(',').map(|c| c.trim().parse().unwrap()).collect();
            Coordinate {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Coordinate]) -> usize {
    let grid = Grid::new(input.to_vec());
    let mut closest_locations: Vec<Coordinate> = Vec::new();

    for x in grid.bounds.min_x..grid.bounds.max_x + 1 {
        for y in grid.bounds.min_y..grid.bounds.max_y + 1 {
            //At each bounding point, calculate the closest location and determine if it is
            //indeed the closest (not equidistant to another location)
            let current = Coordinate { x, y };
            let closest = grid
                .locations
                .iter()
                .min_by_key(|o| o.manhattan_dist(&current))
                .unwrap();
            let dist = closest.manhattan_dist(&current);

            if grid
                .locations
                .iter()
                .filter(|o| o.manhattan_dist(&current) == dist)
                .count()
                == 1
            {
                closest_locations.push(closest.clone());
            }
        }
    }

    let mut max = 0;
    for f in grid.finite.iter() {
        let area = closest_locations.iter().filter(|o| o == &f).count();
        if area > max {
            max = area;
        }
    }

    max
}

#[aoc(day6, part2)]
pub fn part2(input: &[Coordinate]) -> usize {
    let grid = Grid::new(input.to_vec());

    let mut cnt = 0;
    for x in grid.bounds.min_x..grid.bounds.max_x + 1 {
        for y in grid.bounds.min_y..grid.bounds.max_y + 1 {
            let current = Coordinate { x, y };
            let sum: usize = grid
                .locations
                .iter()
                .map(|o| o.manhattan_dist(&current))
                .sum();
            if sum < 10000 {
                cnt += 1;
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "1, 1\n\
                             1, 6\n\
                             8, 3\n\
                             3, 4\n\
                             5, 5\n\
                             8, 9";

    #[test]
    fn grok_input() {
        let expected = vec![
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 1, y: 6 },
            Coordinate { x: 8, y: 3 },
            Coordinate { x: 3, y: 4 },
            Coordinate { x: 5, y: 5 },
            Coordinate { x: 8, y: 9 },
        ];
        assert_eq!(input_coordiantes(TEST_STR), expected);
    }

    #[test]
    fn distance() {
        let a = Coordinate { x: 6, y: 6 };
        let b = Coordinate { x: 0, y: 0 };

        assert_eq!(a.manhattan_dist(&b), 12);
        assert_eq!(a.manhattan_dist(&b), b.manhattan_dist(&a));
    }

    #[test]
    fn generate_grid() {
        let finite = vec![Coordinate { x: 3, y: 4 }, Coordinate { x: 5, y: 5 }];
        let bounds = Bounds {
            min_x: 1,
            max_x: 8,
            min_y: 1,
            max_y: 9,
        };

        let grid = Grid::new(input_coordiantes(TEST_STR));
        assert_eq!(grid.finite, finite);
        assert_eq!(grid.bounds, bounds);
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_coordiantes(TEST_STR)), 17);
    }
}
