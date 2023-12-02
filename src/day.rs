mod day01;
mod day02;

use crate::problem::Problem;

pub fn run_task(input: Vec<String>, day: u64, part: u8) -> String {
  let day: Box<dyn Problem> = match day {
    1 => Box::new(day01::Day1),
    2 => Box::new(day02::Day2),
    3 => unimplemented!("Day not implemented yet"),
    4 => unimplemented!("Day not implemented yet"),
    5 => unimplemented!("Day not implemented yet"),
    6 => unimplemented!("Day not implemented yet"),
    7 => unimplemented!("Day not implemented yet"),
    8 => unimplemented!("Day not implemented yet"),
    9 => unimplemented!("Day not implemented yet"),
    10 => unimplemented!("Day not implemented yet"),
    11 => unimplemented!("Day not implemented yet"),
    12 => unimplemented!("Day not implemented yet"),
    13 => unimplemented!("Day not implemented yet"),
    14 => unimplemented!("Day not implemented yet"),
    15 => unimplemented!("Day not implemented yet"),
    16 => unimplemented!("Day not implemented yet"),
    17 => unimplemented!("Day not implemented yet"),
    18 => unimplemented!("Day not implemented yet"),
    19 => unimplemented!("Day not implemented yet"),
    20 => unimplemented!("Day not implemented yet"),
    21 => unimplemented!("Day not implemented yet"),
    22 => unimplemented!("Day not implemented yet"),
    23 => unimplemented!("Day not implemented yet"),
    24 => unimplemented!("Day not implemented yet"),
    25 => unimplemented!("Day not implemented yet"),
    _ => panic!("Invalid day chosen: {}", day)
  };

  match part {
    1 => day.part1(input),
    2 => day.part2(input),
    _ => panic!("Invalid part chosen: {}", part)
  }
}