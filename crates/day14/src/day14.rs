use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  let mut polymer = parse(input);
  step_n(&mut polymer,10)
}

pub fn part2(input:&[String]) -> usize {
  let mut polymer = parse(input);
  step_n(&mut polymer,40)
}

fn step_n(polymer:&mut Polymer, steps:usize) -> usize {
  for _ in 0..steps{
    polymer.react();
  }
  polymer.calc_score()
}

// looked up ways to make solution look more rusty
// so I created a struct polymer and associated methods

struct Polymer {
  units: HashMap<(char,char),usize>,
  rules: HashMap<(char,char),char>,
  first: char,
  last: char
}

impl Polymer {
  fn react(&mut self) {
      let mut new_units = HashMap::new();
      for (rule, ins) in self.rules.iter() {
          if let Some(count) = self.units.get(rule) {
              *new_units.entry((rule.0, *ins)).or_insert(0) += count;
              *new_units.entry((*ins, rule.1)).or_insert(0) += count;
          }
      }
      self.units = new_units;
  }

  fn calc_score(&self) -> usize {
      let mut counts = HashMap::new();
      for ((a, b), count) in self.units.iter() {
          *counts.entry(*a).or_insert(0) += count;
          *counts.entry(*b).or_insert(0) += count;
      }
      // everything except first and last are counted twice
      // because of sliding window
      *counts.entry(self.first).or_insert(0) += 1;
      *counts.entry(self.last).or_insert(0) += 1;

      let max = counts.values().max().unwrap();
      let min = counts.values().min().unwrap();
      (max - min) / 2
  }
}

fn parse(input:&[String]) -> Polymer {
  let mut rules = HashMap::new();
  let mut pairs = HashMap::new();
  let start = input.iter().next().unwrap();
  let first = start.chars().next().unwrap();
  let last = start.chars().nth(start.len()-1).unwrap();

  input.iter().next().unwrap().as_bytes().windows(2).for_each(|window| {
    let counter = pairs.entry(
      (window[0] as char, window[1] as char)
    ).or_insert(0);
    *counter += 1;
  });

  for line in input.iter().skip(2) {
    let mut from_to = line.split(" -> ");
    let mut chars = from_to.next().unwrap().chars();
    let from = (chars.next().unwrap(),chars.next().unwrap());
    let to = from_to.next().unwrap().chars().next().unwrap();
    rules.insert(from, to);
  }
  Polymer {
    units:pairs,
    rules,
    first,
    last
  }
}

#[cfg(test)]
mod tests {
  use super::{parse,step_n};
  static INPUT: &str = "NNCB

  CH -> B
  HH -> N
  CB -> H
  NH -> C
  HB -> C
  HC -> B
  HN -> C
  NN -> C
  BH -> H
  NC -> B
  NB -> B
  BN -> B
  BB -> N
  BC -> B
  CC -> N
  CN -> C";

  #[test]
  fn test_part1() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let mut polymer = parse(&input);
    assert_eq!(1588,step_n(&mut polymer,10))
  }
  #[test]
  fn test_part2() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let mut polymer = parse(&input);
    assert_eq!(2188189693529,step_n(&mut polymer,40))
  }
}