use crate::problem::Problem;

pub struct Day1;

impl Problem for Day1 {
  fn part1(&self, input: Vec<String>) -> String {
    input.iter().map(|l| {
      let mut numbers = l.chars().filter(char::is_ascii_digit).collect::<Vec<_>>();
      let first = numbers.first().unwrap();
      let last = numbers.last().unwrap();
      format!("{}{}", first, last).parse::<u64>().unwrap()
    }).sum::<u64>().to_string()
  }
}