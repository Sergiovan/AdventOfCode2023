use crate::problem::Problem;

pub struct Day3;

fn in_range(area: ((i32, i32), (i32, i32)), point: (usize, usize)) -> bool {
  let ((y1, x1), (y2, x2)) = area;
  let (py, px) = point;
  let (px, py) = (px as i32, py as i32);

  x1 <= px && px <= x2 && y1 <= py && py <= y2
}

impl Problem for Day3 {
  fn part1(&self, input: Vec<String>) -> String {
    let bytes = input.iter().map(String::as_bytes).collect::<Vec<&[u8]>>();
    let bytes = bytes.as_slice();

    let mut first = None;
    let mut last = (0, 0);
    let mut buff = String::new();
    let mut found = Vec::new();

    for (y, line) in bytes.iter().enumerate() {
      for (x, c) in line.iter().enumerate() {
        match *c as char {
          '0' ..= '9' => {
            if first.is_none() {
              first = Some((y as i32, x as i32));
            }
            last = (y as i32, x as i32);

            buff.push(*c as char);
          }
          _ => {
            if let Some(oneth) = first {
              let ender = last;
              found.push((buff.parse::<u64>().unwrap(), ((oneth.0 - 1, oneth.1 - 1), (ender.0 + 1, ender.1 + 1))));
              buff = String::new();
              first = None;
              last = (0, 0);
            }
          }
        }
      }
    }

    let mut adjacents = Vec::new();

    for (y, line) in bytes.iter().enumerate() {
      for (x, c) in line.iter().enumerate() {
        match *c as char {
          '0' ..= '9' | '.' => (),
          _ => {
            let (yes, no): (Vec<_>, Vec<_>) = found.iter().partition(|(_, area)| {
              in_range(*area, (y, x))
            });
            found = no;
            yes.into_iter().for_each(|e| {
              adjacents.push(e.0);
            })
          }
        }
        // println!("{:?}", found);
      }
    }

    adjacents.iter().sum::<u64>().to_string()
  }
}