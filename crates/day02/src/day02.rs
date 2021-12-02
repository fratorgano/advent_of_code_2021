pub fn part1(input:&[String]) -> usize {
  let position = pilot(input);
  position.0 * position.1
}
pub fn part2(input:&[String]) -> usize {
  let position = pilot2(input);
  position.0 * position.1
}

fn pilot(input:&[String]) -> (usize, usize) {
  let mut position = (0,0);
  for s in input {
    let split:Vec<&str> = s.split(' ').collect();
    match split[0] {
      "forward" => position.0 += split[1].parse::<usize>().unwrap(),
      "down" => position.1 += split[1].parse::<usize>().unwrap(),
      "up" => position.1 -= split[1].parse::<usize>().unwrap(),
      _ => panic!("Unexpected input")
    }
  }
  position
}

fn pilot2(input:&[String]) -> (usize, usize, usize) {
  let mut position = (0,0,0);
  for s in input {
    let split:Vec<&str> = s.split(' ').collect();
    match split[0] {
      "forward" => {
        position.0 += split[1].parse::<usize>().unwrap();
        position.1 += position.2 * split[1].parse::<usize>().unwrap();
      },
      "down" => position.2 += split[1].parse::<usize>().unwrap(),
      "up" => position.2 -= split[1].parse::<usize>().unwrap(),
      _ => panic!("Unexpected input")
    }
  }
  position
}

#[cfg(test)]
mod tests {
  use super::{pilot, pilot2};
  #[test]
  fn test_part1() {
    let input = ["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
    let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let position = pilot(&input2);
    assert_eq!(150, position.0 * position.1)
  }
  #[test]
  fn test_part2() {
    let input = ["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
    let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let position = pilot2(&input2);
    assert_eq!(900, position.0 * position.1)
  }
}