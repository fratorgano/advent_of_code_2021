pub fn part1(input:&[String]) -> usize {
  let matrix = parse(input);
  sim(matrix, 100)
}

pub fn part2(input:&[String]) -> usize {
  let matrix = parse(input);
  sync(matrix)
}

fn parse(input:&[String]) -> Vec<Vec<usize>> {
  let mut matrix = vec![];
  for l in input {
    let mut row = vec![];
    for c in l.chars() {
      row.push(c.to_digit(10).unwrap() as usize)
    }
    matrix.push(row);
  }
  matrix
}

fn sim(matrix:Vec<Vec<usize>>, days:usize) -> usize {
  let mut matrix = matrix;
  let mut flashes = 0;
  for _i in 0..days {
    let (flash,new_matrix) = step(matrix);
    matrix = new_matrix;
    flashes += flash;
  }
  flashes
}

fn sync(matrix:Vec<Vec<usize>>) -> usize {
  let all = matrix.len() * matrix[0].len();
  let mut matrix = matrix;
  let mut count = 0;
  loop {
    count += 1;
    let (flash,new_matrix) = step(matrix);
    if flash == all {
      return count;
    }
    matrix = new_matrix;
  }
}

fn step(matrix:Vec<Vec<usize>>) -> (usize, Vec<Vec<usize>>) {
  let mx = matrix.len();
  let my = matrix[0].len();
  let mut done = false;
  // +1 everything
  let mut new_matrix = vec![vec![0;matrix[0].len()];matrix.len()];
  for i in 0..matrix.len(){
    for j in 0..matrix[i].len() {
      new_matrix[i][j] = matrix[i][j] + 1;
    }
  }
  // check for flashes
  let mut flashes_coords = vec![];
  while !done {
    done = true;
    for i in 0..new_matrix.len(){
      for j in 0..new_matrix[i].len() {
        if new_matrix[i][j]>9 {
          done = false;
          new_matrix[i][j] = 0;
          let adjacents = adjacent(i,j,mx-1,my-1);
          for (x,y) in adjacents {
            new_matrix[x][y] += 1;
          }
          flashes_coords.push((i,j));
        }
      }
    }
  }
  let flashes = flashes_coords.len();
  for (x,y) in flashes_coords {
    new_matrix[x][y] = 0;
  }
  (flashes, new_matrix)
}

fn adjacent(x: usize, y: usize, mx: usize, my: usize) -> Vec<(usize,usize)> {
  let mut res = vec![];
  if y>0 { res.push((x,y-1))}
  if x>0 { res.push((x-1,y))}
  if x<mx { res.push((x+1,y))}
  if y<my { res.push((x,y+1))}
  // diagonals
  if x>0 && y<my {res.push((x-1,y+1))}
  if x>0 && y>0 {res.push((x-1,y-1))}
  if x<mx && y<my {res.push((x+1,y+1))}
  if x<mx && y>0 {res.push((x+1,y-1))}
  res
}

#[cfg(test)]
mod tests {
  use super::{parse,sim,sync};
  #[test]
  fn test_part1() {
    let string = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let parsed = parse(&input);
    let answer = sim(parsed, 100);
    assert_eq!(1656,answer);
  }
  #[test]
  fn test_part2() {
    let string = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let parsed = parse(&input);
    let answer = sync(parsed);
    assert_eq!(195,answer);
  }
}