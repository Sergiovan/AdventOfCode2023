use crate::problem::Problem;

pub struct Day2;

struct Cubes {
  red: u64,
  green: u64,
  blue: u64
}

impl Problem for Day2 {
  fn part1(&self, input: Vec<String>) -> String {
    input.iter().map(|l| {
      let mut iter = l.split(':');
      let id = iter.next().unwrap().split(' ').last().unwrap().parse::<u64>().unwrap();
      let games = iter.next().unwrap().split(';');
      let minimum_cubes = games.map(|g| {
        let mut cubes = Cubes{ red: 0, green: 0, blue: 0 };
        
        for cube in g.trim().split(',') {
          let mut cube_parts = cube.trim().split(' ');
          let num = cube_parts.next().unwrap().parse::<u64>().unwrap();
          let color = cube_parts.next().unwrap().trim();

          match color {
            "red" => cubes.red += num,
            "green" => cubes.green += num,
            "blue" => cubes.blue += num,
            _ => panic!("Invalid color: {}", color)
          }
        }

        cubes
      }).reduce(|l, r| {
        Cubes {
          red: l.red.max(r.red),
          green: l.green.max(r.green),
          blue: l.blue.max(r.blue)
        }
      }).unwrap();

      if minimum_cubes.red > 12 || minimum_cubes.green > 13 || minimum_cubes.blue > 14 {
        0
      } else {
        id
      }

    }).sum::<u64>().to_string()
  }

  fn part2(&self, input: Vec<String>) -> String {
    input.iter().map(|l| {
      let mut iter = l.split(':');
      let id = iter.next().unwrap().split(' ').last().unwrap().parse::<u64>().unwrap();
      let games = iter.next().unwrap().split(';');
      let minimum_cubes = games.map(|g| {
        let mut cubes = Cubes{ red: 0, green: 0, blue: 0 };
        
        for cube in g.trim().split(',') {
          let mut cube_parts = cube.trim().split(' ');
          let num = cube_parts.next().unwrap().parse::<u64>().unwrap();
          let color = cube_parts.next().unwrap().trim();

          match color {
            "red" => cubes.red += num,
            "green" => cubes.green += num,
            "blue" => cubes.blue += num,
            _ => panic!("Invalid color: {}", color)
          }
        }

        cubes
      }).reduce(|l, r| {
        Cubes {
          red: l.red.max(r.red),
          green: l.green.max(r.green),
          blue: l.blue.max(r.blue)
        }
      }).unwrap();

      minimum_cubes.red * minimum_cubes.green * minimum_cubes.blue

    }).sum::<u64>().to_string()
  }
}