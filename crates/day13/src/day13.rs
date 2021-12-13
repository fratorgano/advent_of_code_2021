use std::collections::HashSet;

pub fn part1(input:&[String]) -> usize {
  let (map,folds) = parse(input);
  count_values(map,folds)
}

pub fn part2(input:&[String]) -> usize {
  let (map,folds) = parse(input);
  count_values_all(map,folds)
}

fn count_values(set:HashSet<(usize,usize)>, folds:Vec<(usize,usize)>) -> usize {
  fold(set.clone(),folds[0]).len()
}

fn count_values_all(set:HashSet<(usize,usize)>, folds:Vec<(usize,usize)>) -> usize {
  let answer = folds.into_iter().fold(set, fold);
  print_map_as_matrix(&answer,40,5);
  answer.len()
}

fn fold(set:HashSet<(usize,usize)>, fold:(usize,usize)) -> HashSet<(usize,usize)> {
  set.iter().map(|(x,y)| {
    if fold.0 == 0 && y>&fold.1{
      let new_y = fold.1-(y-fold.1); 
      (*x,new_y)
    }else if fold.1 == 0 && x>&fold.0{
      let new_x = fold.0 - (x-fold.0);
      (new_x,*y)
    } else {
      (*x,*y)
    }
  }).collect()
}

fn parse(input:&[String]) -> (HashSet<(usize,usize)>, Vec<(usize,usize)>) {
  let mut set = HashSet::new();
  let mut fold_instructions = vec![];
  
  for line in input {
    if line.contains("fold along") {
      // fold along x=655
      // fold along y=447
      let mut splitted = line.split("fold along ");
      let last_part = splitted.nth(1).unwrap();
      let mut splitted2 = last_part.split('=');
      let value = splitted2.nth(1).unwrap().parse().unwrap();
      if last_part.contains("x=") {
        fold_instructions.push((value,0))
      }else{
        fold_instructions.push((0,value))
      }
    } else if !line.is_empty(){
      // 755,677
      let mut splitted = line.split(',');
      let v1 = splitted.next().unwrap().parse().unwrap();
      let v2 = splitted.next().unwrap().parse().unwrap();
      set.insert((v1,v2));
    }
  }
  (set,fold_instructions)
}

fn print_map_as_matrix(set:&HashSet<(usize,usize)>,mx:usize,my:usize) {
  for i in 0..=my {
    for j in 0..=mx {
      if set.contains(&(j,i)) {
        print!("#");
      } else {
        print!(" ");
      }
    }
    println!();
  }
  println!();
}

#[cfg(test)]
mod tests {
  use super::{parse,count_values,fold};
  static INPUT: &str = "6,10
  0,14
  9,10
  0,3
  10,4
  4,11
  6,0
  6,12
  4,1
  0,13
  10,12
  3,4
  3,0
  8,4
  1,10
  2,14
  8,10
  9,0
  
  fold along y=7
  fold along x=5";

  #[test]
  fn test_part1() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (map,folds) = parse(&input);
    assert_eq!(18,map.len());
    let map2 = fold(map,folds[0]);
    assert_eq!(17,map2.len());
    let map3 = fold(map2,folds[1]);
    assert_eq!(16,map3.len());
  }
  #[test]
  fn test_part2() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (map,folds) = parse(&input);
    assert_eq!(17,count_values(map, folds));
  }
}