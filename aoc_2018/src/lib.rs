extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate log;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
// pub mod day10; -> This is compiled as a seperate binary `$ cargo run day10`
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

aoc_lib! { year = 2018 }
