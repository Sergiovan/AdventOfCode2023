use crate::problem::Problem;

pub struct Day1;

fn window(src: &str) -> impl Iterator<Item = &'_ str> {
  src.char_indices().map(move |(from, _)| {
    &src[from..]
  })
}

impl Problem for Day1 {
  fn part1(&self, input: Vec<String>) -> String {
    input.iter().map(|l| {
      let numbers = l.chars().filter(char::is_ascii_digit).collect::<Vec<_>>();
      let first = numbers.first().unwrap();
      let last = numbers.last().unwrap();
      format!("{}{}", first, last).parse::<u64>().unwrap()
    }).sum::<u64>().to_string()
  }

  fn part2(&self, input: Vec<String>) -> String {
    input.iter().map(|l| {
      let mut res = String::new();

      let windows = window(l);

      for w in windows {
        match w.chars().next().unwrap() {
          '0'..='9' => res.push(w.chars().next().unwrap()),
          'o' => {
            if w.starts_with("one") {
              res.push('1');
            }
          }
          't' => {
            if w.starts_with("two") {
              res.push('2');
            } else if w.starts_with("three") {
              res.push('3');
            }
          }
          'f' => {
            if w.starts_with("four") {
              res.push('4');
            } else if w.starts_with("five") {
              res.push('5');
            }
          }
          's' => {
            if w.starts_with("six") {
              res.push('6');
            } else if w.starts_with("seven") {
              res.push('7');
            }
          }
          'e' => {
            if w.starts_with("eight") {
              res.push('8');
            }
          }
          'n' => {
            if w.starts_with("nine") {
              res.push('9');
            }
          },
          _ => ()
        }
      }

      let first = res.chars().next().unwrap();
      let last = res.chars().last().unwrap();
      format!("{}{}", first, last).parse::<u64>().unwrap()

    }).sum::<u64>().to_string()
  }
}