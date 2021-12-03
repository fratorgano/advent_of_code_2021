pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  let (gamma,epsilon) = find_gamma_epsilon_rate(parsed);
  gamma * epsilon
}
pub fn part2(input:&[String]) -> usize {
  let oxigen = oxigen_rating(input);
  let co2 = co2_rating(input);
  oxigen*co2
}

fn parse(input:&[String]) -> Vec<Vec<char>> {
  let len = input[0].len();
  let mut answer = vec![vec![]; len];
  for s in input {
    for (i,c) in s.chars().enumerate() {
      answer[i].push(c);
    }
  }
  answer
}

fn find_gamma_epsilon_rate(input:Vec<Vec<char>>) -> (usize,usize) {
  let mut gamma_rate = String::new();
  let mut epsilon_rate = String::new();
  for a in input {
    let (mc,lc) = find_most_least_common(&a);
    gamma_rate.push(mc);
    epsilon_rate.push(lc);
  }
  (binary_to_uint(gamma_rate), binary_to_uint(epsilon_rate))
}

fn find_most_least_common(input:&[char]) -> (char,char) {
  let mut zeroes = 0;
  for c in input {
    if *c=='0' {zeroes += 1}
  }
  let ones = input.len() - zeroes;
  if ones>=zeroes {
    ('1','0')
  }else {
    ('0','1')
  }
}

fn oxigen_rating(input: &[String]) -> usize {
  let mut input_copy = input.to_owned();
  for i in 0..input_copy[0].len() {
    let bits = parse(&input_copy);
    let len = input_copy.len()-1;
    let a = &bits[i];
    let (mc,_lc) = find_most_least_common(a);
    for (i,b) in a.iter().rev().enumerate() {
      if *b != mc { input_copy.remove(len-i); }
    }
    if input_copy.len() == 1 {
      return binary_to_uint(input_copy[0].to_owned());
    }
  }
  1
}

fn co2_rating(input: &[String]) -> usize {
  let mut input_copy = input.to_owned();
  for i in 0..input_copy[0].len() {
    let bits = parse(&input_copy);
    let len = input_copy.len()-1;
    let a = &bits[i];
    let (_mc,lc) = find_most_least_common(a);
    for (i,b) in a.iter().rev().enumerate() {
      if *b != lc { input_copy.remove(len-i); }
    }
    if input_copy.len() == 1 {
      return binary_to_uint(input_copy[0].to_owned());
    }
  }
  1
}

fn binary_to_uint(input:String) -> usize {
  let mut answer = 0;
  for c in input.chars() {
    answer <<= 1;
    if c=='1' {
      answer += 1;
    }
  }
  answer
}

#[cfg(test)]
mod tests {
  use super::{parse, find_gamma_epsilon_rate, oxigen_rating, co2_rating};
  #[test]
  fn test_part1() {
    let input = ["00100", "11110", "10110", "10111", "10101", "01111","00111","11100","10000","11001","00010","01010"];
    let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input2);
    let answer = find_gamma_epsilon_rate(parsed);
    assert_eq!((22,9),answer)
  }
  #[test]
  fn test_part2() {
    let input = ["00100", "11110", "10110", "10111", "10101", "01111","00111","11100","10000","11001","00010","01010"];
    let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let oxigen = oxigen_rating(&input2);
    let co2 = co2_rating(&input2);
    assert_eq!(230,oxigen*co2)
  }
}