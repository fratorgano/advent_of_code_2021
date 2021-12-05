use std::cmp::{max, min};

pub fn part1(input:&[String]) -> usize {
  let (parsed,max) = parse(input);
  count_overlap(parsed,max, false)
}
pub fn part2(input:&[String]) -> usize {
  let (parsed,max) = parse(input);
  count_overlap(parsed,max, true)
}

fn parse(input:&[String]) -> (Vec<((usize,usize),(usize,usize))>, (usize,usize)) {
  let mut parsed:Vec<((usize,usize),(usize,usize))> = vec![];
  let mut max_x = 0;
  let mut max_y = 0;
  for l in input {
      let mut split = l.split(" -> ");
      let first_pair = split.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
      let second_pair = split.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
      // not the pretties but I need to find max_x and max_y to size the array, 
      // since an array is faster than a map for this problem
      if first_pair[0] > max_x { max_x = first_pair[0] }
      if first_pair[1] > max_y { max_y = first_pair[1] }
      if second_pair[0] > max_x { max_x = second_pair[0] }
      if second_pair[1] > max_y { max_y = second_pair[1] }
      parsed.push(((first_pair[0],first_pair[1]),(second_pair[0],second_pair[1])));
  }
  (parsed,(max_x,max_y))
}

fn count_overlap (input:Vec<((usize,usize),(usize,usize))>, max:(usize,usize), diagonal:bool) -> usize{
  let mut vec:Vec<Vec<usize>> = vec![vec![0;max.1+1];max.0+1];
  for pair in input {
    if diagonal || pair.0.0 == pair.1.0 || pair.0.1 == pair.1.1 {
      let range = find_range_2(pair.0.0, pair.0.1, pair.1.0, pair.1.1);
      for pair in range {
        vec[pair.0][pair.1] += 1;
      }
    }
  }
  vec.iter().fold(0, |acc,x| acc + x.iter().fold(0, |acc,x| if *x>=2 {acc + 1} else {acc}))
}

fn find_range_2(x1:usize,y1:usize,x2:usize,y2:usize) -> Vec<(usize,usize)> {
  let mut answer = vec![];
  let x_diff = (x1 as i32 - x2 as i32 ).abs();
  let y_diff = (y1 as i32 - y2 as i32 ).abs();
  if x_diff == y_diff {
    let mut xs = (min(x1,x2)..=max(x1,x2)).collect::<Vec<usize>>();
    let mut ys = (min(y1,y2)..=max(y1,y2)).collect::<Vec<usize>>();
    if x1 < x2 {
      xs.reverse();
    }
    if y1 < y2 {
      ys.reverse();
    }
    for (x,y) in xs.iter().zip(ys.iter()) {
      answer.push((*x,*y));
    }
    return answer;
  } 
  for x in min(x1,x2)..=max(x1,x2) {
    for y in min(y1,y2)..=max(y1,y2) {
      answer.push((x,y));
    }
  }
  answer
}

#[cfg(test)]
mod tests {
  use super::{parse, count_overlap};
  #[test]
  fn test_part1() {
    let input = ["0,9 -> 5,9",
    "8,0 -> 0,8",
    "9,4 -> 3,4",
    "2,2 -> 2,1",
    "7,0 -> 7,4",
    "6,4 -> 2,0",
    "0,9 -> 2,9",
    "3,4 -> 1,4",
    "0,0 -> 8,8",
    "5,5 -> 8,2"];
    let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let (parsed, max) = parse(&input2);
    assert_eq!(5,count_overlap(parsed, max, false));
  }
  #[test]
  fn test_part2() {
    let input = ["0,9 -> 5,9",
    "8,0 -> 0,8",
    "9,4 -> 3,4",
    "2,2 -> 2,1",
    "7,0 -> 7,4",
    "6,4 -> 2,0",
    "0,9 -> 2,9",
    "3,4 -> 1,4",
    "0,0 -> 8,8",
    "5,5 -> 8,2"];
    let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let (parsed, max) = parse(&input2);
    assert_eq!(12,count_overlap(parsed, max, true));
  }
}