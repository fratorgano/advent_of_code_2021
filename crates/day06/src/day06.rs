pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  simulate_n_days(parsed,80)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  simulate_n_days(parsed,256)
}
fn parse(input:&[String]) -> Vec<usize> {
  input[0].split(',').map(|s| s.parse().unwrap()).collect()
}

fn simulate_n_days(pop:Vec<usize>, n:usize) -> usize {
  let mut ages:[usize;9] = [0;9];
  for fish in pop {
    ages[fish] += 1;
  }
  for _i in 0..n {
    // you could also use ages.rotate_left(1), but that was a tiny bit slower
    let new = ages[0];
    for i in 1..ages.len() {
      ages[i-1] = ages[i];
    }
    ages[6] += new;
    ages[8] = new;
  }
  ages.iter().sum()
}

#[cfg(test)]
mod tests {
  use super::{parse,simulate_n_days};
  #[test]
  fn test_part1() {
    let strings = ["3,4,3,1,2"];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input);
    let result = simulate_n_days(parsed, 80);
    assert_eq!(5934,result);
  }
  #[test]
  fn test_part2() {
    let strings = ["3,4,3,1,2"];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input);
    let result = simulate_n_days(parsed, 256);
    assert_eq!(26984457539,result);
  }
}