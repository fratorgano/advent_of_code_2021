use std::collections::BinaryHeap;

pub fn part1(input:&[String]) -> isize {
  let matrix = parse(input);
  shortes_path((0,0),(matrix.len()-1,matrix[0].len()-1),&matrix)
}

pub fn part2(input:&[String]) -> isize {
  let matrix = expand_parse(parse(input));
  shortes_path((0,0),(matrix.len()-1,matrix[0].len()-1),&matrix)
}

fn shortes_path(from:(usize,usize),to:(usize,usize),maze: &[Vec<isize>]) -> isize {
  let mx = maze[0].len();
  let my = maze.len();
  let mut dist = vec![vec![isize::MAX; maze[0].len()]; maze.len()];
  let mut q = BinaryHeap::new();
  q.push((0,from.0,from.1));
  while let Some((cost,x,y)) = q.pop() {
    if (x,y) == to { return -cost; }
    if -cost > dist[x][y] { continue; }
    for (x1,y1) in adjacent(x, y, mx, my) {
      if maze.get(x1).and_then(|row| row.get(y1)).is_none() {
        continue;
      }
      let next_cost = -cost + maze[x1][y1];
      if next_cost < dist[x1][y1] {
        q.push((-next_cost, x1,y1));
        dist[x1][y1] = next_cost;
      }
    }
  }
  unreachable!()
}

fn parse(input:&[String]) -> Vec<Vec<isize>> {
  let mut matrix = vec![];

  for line in input {
    let mut row = vec![];
    for c in line.chars() {
      row.push(c.to_digit(10).unwrap() as isize)
    }
    matrix.push(row);
  }
  matrix
}

fn expand_parse(input:Vec<Vec<isize>>) -> Vec<Vec<isize>> {
  let mut matrix = vec![];

  for y in 0..(5*input.len()) {
    let mut row = vec![];
    for x in 0..(5*input[0].len()) {
      let x_depth = (x/input.len()) as isize;
      let y_depth = (y/input[0].len()) as isize;
      let mut cost = input[y%input.len()][x%input[0].len()] + y_depth + x_depth;
      if cost>=10 {cost-=9}
      row.push(cost);
    }
    matrix.push(row);
  }
  matrix
}

fn adjacent(x: usize, y: usize, mx: usize, my: usize) -> Vec<(usize,usize)> {
  let mut res = vec![];
  if y>0 { res.push((x,y-1))}
  if x>0 { res.push((x-1,y))}
  if x<mx { res.push((x+1,y))}
  if y<my { res.push((x,y+1))}
  res
}


#[cfg(test)]
mod tests {
  use super::{parse,expand_parse,shortes_path};
  static INPUT: &str = "1163751742
  1381373672
  2136511328
  3694931569
  7463417111
  1319128137
  1359912421
  3125421639
  1293138521
  2311944581";

  #[test]
  fn test_part1() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    assert_eq!(40,shortes_path((0,0),(graph.len()-1,graph[0].len()-1),&graph))
  }
  #[test]
  fn test_part2() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = expand_parse(parse(&input));
    print!("{:?}",graph);
    assert_eq!(315,shortes_path((0,0),(graph.len()-1,graph[0].len()-1),&graph))
  }
}