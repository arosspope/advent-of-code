// Day 10: The Stars Align
//
// This day requires a visualisation of the problem to solve. As such, this day is
// compiled as a seperate binary. Run with: `cargo run day10`
//
extern crate regex;

use std::str;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main(){
    let path = Path::new("input/2018/day10.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let mut sky = NightSky::new(&contents);

    for _ in 0..1_000_00 {
        sky.tick();
        let (w, h) = sky.dimensions();
        if w <= 80 && h <= 80 {
            writeln!(io::stdout(), "t @ {}s", sky.time).unwrap();
            writeln!(io::stdout(), "{}", sky.display().trim()).unwrap();
        }
    }
}


#[derive(Default, PartialEq, Debug, Eq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Default, PartialEq, Debug, Eq, Clone)]
struct Light {
    position: Point,
    velocity: (isize, isize), //x velocity, y velocity
}

#[derive(Default, PartialEq, Debug, Eq, Clone)]
struct NightSky {
    lights: Vec<Light>,
    time: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct Bounds {
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
}

impl Bounds {
    fn normalaise(&self, p: &Point) -> Point {
        let (x, y): (isize, isize);
        if self.minx >= 0 {
            x = p.x - self.minx;
        } else {
            x = p.x + self.minx.abs();
        }
        if self.miny >= 0 {
            y = p.y - self.miny;
        } else {
            y = p.y + self.miny.abs();
        }
        Point { x, y }
    }
}

impl NightSky {
    fn new(input: &str) -> NightSky {
        let re = Regex::new(
            r"(?x)
                    position=<\s*(?P<x>[-0-9]+),\s*(?P<y>[-0-9]+)>
                    \s+
                    velocity=<\s*(?P<vx>[-0-9]+),\s*(?P<vy>[-0-9]+)>
                ",
        )
        .unwrap();

        let lights = input
            .lines()
            .map(|l| {
                let caps = re.captures(l).unwrap();
                Light {
                    position: Point{ x: caps["x"].parse().unwrap(), y: caps["y"].parse().unwrap()},
                    velocity: (caps["vx"].parse().unwrap(), caps["vy"].parse().unwrap()),
                }
            })
            .collect();

        NightSky { lights, time: 0 }
    }

    fn tick(&mut self) {
        self.lights.iter_mut().for_each(|l| {
            l.position.x += l.velocity.0;
            l.position.y += l.velocity.1;
        });
        self.time += 1;
    }

    fn display(&self) -> String {
        let b = self.bounds();
        let mut grid = vec![vec![b'.'; (b.maxx - b.minx + 1) as usize]; (b.maxy - b.miny + 1) as usize];
        for l in &self.lights {
            let p = b.normalaise(&l.position);
            grid[p.y as usize][p.x as usize] = b'#';
        }

        let mut buffer = String::new();
        for row in grid {
            buffer.push_str(str::from_utf8(&row).unwrap());
            buffer.push('\n');
        }

        buffer
    }

    //Returns top left point, and bottom right point
    fn bounds(&self) -> Bounds {
        let maxy = self
            .lights
            .iter()
            .max_by_key(|point| point.position.y)
            .unwrap()
            .position
            .y;
        let miny = self
            .lights
            .iter()
            .min_by_key(|point| point.position.y)
            .unwrap()
            .position
            .y;
        let maxx = self
            .lights
            .iter()
            .max_by_key(|point| point.position.x)
            .unwrap()
            .position
            .x;
        let minx = self
            .lights
            .iter()
            .min_by_key(|point| point.position.x)
            .unwrap()
            .position
            .x;

        Bounds { minx, maxy, maxx, miny }
    }

    fn dimensions(&self) -> (usize, usize) {
        let b = self.bounds();
        ((b.maxx - b.minx + 1) as usize, (b.maxy - b.miny + 1) as usize)
    }
}

impl std::convert::AsRef<NightSky> for NightSky {
    fn as_ref(&self) -> &NightSky {
        &self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "position=< 9,  1> velocity=< 0,  2>\n\
                             position=< 7,  0> velocity=<-1,  0>\n\
                             position=< 3, -2> velocity=<-1,  1>\n\
                             position=< 6, 10> velocity=<-2, -1>\n\
                             position=< 2, -4> velocity=< 2,  2>";



    #[test]
    fn day10_grok_input() {
        let lights = vec![
            Light {
                position: Point { x: 9, y: 1},
                velocity: (0, 2),
            },
            Light {
                position: Point { x: 7, y: 0},
                velocity: (-1, 0),
            },
            Light {
                position: Point { x: 3, y: -2},
                velocity: (-1, 1),
            },
            Light {
                position: Point { x: 6, y: 10},
                velocity: (-2, -1),
            },
            Light {
                position: Point { x: 2, y: -4},
                velocity: (2, 2),
            },
        ];

        assert_eq!(NightSky::new(TEST_STR), NightSky { lights, time: 0 });
    }

}
