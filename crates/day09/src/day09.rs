use std::collections::VecDeque; 

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  find_low_points(parsed)
}

pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  find_basins(parsed)
}

fn parse(input:&[String]) -> Vec<Vec<usize>> {
  let mut res = vec![];
  for line in input {
    let mut vec = vec![];
    for c in line.chars() {
      let number = c.to_digit(10).unwrap() as usize;
      vec.push(number);
    }
    res.push(vec);
  }
  res
}

fn find_low_points(matrix:Vec<Vec<usize>>) -> usize {
  let mut low_points = vec![];
  let mx = matrix[0].len() - 1;
  let my = matrix.len() - 1;
  for j in 0..matrix.len() {
    for i in 0..matrix[j].len() {
      let center_value = matrix[j][i];
      let adjacent = adjacent(i,j,mx,my);
      let mut low_point = true;
      for (x,y) in adjacent {
        if matrix[y][x] <= center_value {
          low_point = false;
          break;
        }
      }
      if low_point {
        low_points.push(center_value);
      }
    }
  }
  low_points.iter().map(|v| v+1).reduce(|acc,v| acc+v).unwrap() as usize
}

fn adjacent(x: usize, y: usize, mx: usize, my: usize) -> Vec<(usize,usize)> {
  let mut res = vec![];
  if y>0 { res.push((x,y-1))}
  if x>0 { res.push((x-1,y))}
  if x<mx { res.push((x+1,y))}
  if y<my { res.push((x,y+1))}
  res
}

fn find_basins(matrix: Vec<Vec<usize>>) -> usize {
  let mut res = vec![];
  let mut matrix = matrix;
  let mx = matrix[0].len() - 1;
  let my = matrix.len() - 1;
  for i in 0..mx {
    for j in 0..my {
      res.push(fill(&mut matrix,i,j));
    }
  }
  res.sort_unstable_by(|a,b| b.cmp(a));
  res.into_iter().take(3).product()
}

fn fill(matrix:&mut Vec<Vec<usize>>, x:usize, y:usize) -> usize {
  let mx = matrix[0].len() - 1;
  let my = matrix.len() - 1;
  let mut filled = 0;
  let mut queue:VecDeque<(usize,usize)> = VecDeque::new();
  queue.push_back((x,y));
  while let Some((x,y)) = queue.pop_front() {
    if matrix[y][x] != 9 {
      matrix[y][x] = 9;
      filled += 1;
      adjacent(x, y, mx, my).into_iter().for_each(|c| queue.push_back(c));
    }
  }
  filled
}

#[cfg(test)]
mod tests {
  use super::{parse,find_low_points,find_basins};
  #[test]
  fn test_part1() {
    let strings = [
      "2199943210",
      "3987894921",
      "9856789892",
      "8767896789",
      "9899965678",
    ];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input);
    let answer = find_low_points(parsed);
    assert_eq!(15,answer);
  }
  #[test]
  fn test_part2() {
    let strings = [
      "2199943210",
      "3987894921",
      "9856789892",
      "8767896789",
      "9899965678",
    ];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input);
    let res = find_basins(parsed);
    assert_eq!(1134,res);
  }
}