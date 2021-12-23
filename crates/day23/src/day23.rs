use std::collections::HashMap;
use std::collections::BinaryHeap;

pub fn part1(input:&[String]) -> i64 {
  let board = Gamestate{board: parse(input)};
  board.shortest_path()
}

pub fn part2(input:&[String]) -> i64 {
  let mut input:Vec<String> = input.to_vec();
  input.insert(3, "  #D#C#B#A#".to_string());
  input.insert(4, "  #D#B#A#C#".to_string());
  let board = Gamestate{board: parse(&input)};
  board.shortest_path()
}

fn parse(input:&[String]) -> Vec<Vec<Tile>> {
  let mut res = vec![];
  for l in input {
    let mut row = vec![];
    for c in l.chars() {
      let v = match c {
        '#' => Tile::Barrier,
        ' ' => Tile::Air,
        '.' => Tile::Empty,
        'A' => Tile::Full(Amphipod::Amber),
        'B' => Tile::Full(Amphipod::Bronze),
        'C' => Tile::Full(Amphipod::Copper),
        'D' => Tile::Full(Amphipod::Desert),
        _ => panic!("Unexpected char '{}' in input",c)
      };
      row.push(v)
    }
    res.push(row);
  }

  res
}
#[derive(Ord,PartialOrd,Eq,PartialEq,Hash,Clone)]
struct Gamestate {
  board:Vec<Vec<Tile>>
}

impl Gamestate {
  fn shortest_path(self) -> i64 {
    let mut dist:HashMap<Gamestate,i64> = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((0,self.clone()));
    while let Some((cost,state)) = q.pop() {
      if state.done() {
        return -cost;
      };
      if let Some(&c) = dist.get(&state) {
        if (-cost) > c {continue}
      }
      for (moves_cost,next_move) in state.moves() {
        let next_cost = -cost + moves_cost;
        let next_state = state.apply_move(next_move);
        if let Some(c) = dist.get(&next_state) {
          if *c<=next_cost {
            continue;
          }
        }
        dist.insert(next_state.clone(), next_cost);
        q.push((-next_cost,next_state));
      }
    }
    panic!("You shouldn't be here");
  }

  fn done(&self) -> bool {
    let mut v = true;
    for i in 0..self.board.len(){
      for j in 0..self.board[i].len(){
        if let Tile::Full(a) = self.board[i][j] {
          if i==1  {return false};
          v = v && (j==a.target());
        }
      }
    }
    v
  }

  fn moves(&self) -> Vec<(i64,Move)>{
    let max_depth = self.board.len()-1;
    // Moving into room
    let mut moves = vec![];
    //checking the hallway
    for y in 1..=11 {
      // if there's an Amphipod, calculate possible moves
      if let Tile::Full(a) = self.board[1][y] {
        // find Amphipod room
        let target = a.target();
        //check if hallway is clear
        let mut clear = true;
        if y>target {
          for p in target..y {
            if let Tile::Full(_) = self.board[1][p] {
              clear = false;
              break;
            }
          }
        } else {
          for p in y+1..=target {
            if let Tile::Full(_) = self.board[1][p] {
              clear = false;
              break;
            }
          }
        }
        // if hallway not clear, skip this one
        if !clear {continue};
        // check if it can fit
        let mut fit_type = true;
        let mut x_distance = 0;
        for x in 2..max_depth {
          if matches!(self.board[x][target],Tile::Empty) {
            x_distance+=1;
          }else if self.board[x][target] != self.board[1][y] {
            fit_type = false;
            break;
          }
        }
        if !fit_type {continue};
        let m = Move {
          from:(1,y),
          to:(1+x_distance as usize,target)
        };
        let cost = m.dist() * a.cost();
        moves.push((cost,m))
      }
    }
    // leaving rooms
    for y in [3,5,7,9] {
      for x in 2..max_depth {
        if let Tile::Full(a) = self.board[x][y] {
          //moving to the right
          for y_2 in y..=11 {
            if [1,2,4,6,8,10,11].contains(&y_2) {
              // check if the hallway is clear
              // if not, can't move right anymore 
              if let Tile::Full(_b) = self.board[1][y_2] {break;} 
              let m = Move {
                from: (x,y),
                to: (1,y_2)
              };
              let cost = m.dist() * a.cost();
              moves.push((cost,m))
            }
          }
          //moving to the left
          for y_2 in (1..y).rev() {
            if [1,2,4,6,8,10,11].contains(&y_2) {
              //check if the hallway is clear
              // if not, can't move left anymore 
              if let Tile::Full(_b) = self.board[1][y_2] {break;} 
              let m = Move {
                from: (x,y),
                to: (1,y_2)
              };
              let cost = m.dist() * a.cost();
              moves.push((cost,m))
            }
          }
          break;
        }
      }
    }
    moves
  }
  fn apply_move(&self,mv:Move) -> Gamestate {
    let (fx,fy) = mv.from;
    let (tx,ty) = mv.to;
    let mut m = self.board.clone();
    m[tx][ty] = m[fx][fy];
    m[fx][fy] = Tile::Empty;
    Gamestate {
      board:m
    }
  }
}

struct Move{
  from:(usize,usize),
  to:(usize,usize)
}
impl Move{
  fn dist(&self) -> i64 {
    (self.from.0 as i64 - self.to.0 as i64).abs() + (self.from.1 as i64 - self.to.1 as i64).abs()
  }
}

#[derive(PartialEq,Clone,Copy,Debug,Ord,PartialOrd,Eq,Hash)]
enum Tile {
  Empty,
  Full(Amphipod),
  Barrier,
  Air
}
impl std::fmt::Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Tile::Empty => write!(f,"."),
      Tile::Full(a) => write!(f,"{}",a),
      Tile::Barrier => write!(f,"#"),
      Tile::Air => write!(f," ")
    }
  }
}
#[derive(PartialEq,Clone,Copy,Debug,Ord,PartialOrd,Eq,Hash)]
enum Amphipod {
  Amber,
  Bronze,
  Copper,
  Desert
}
impl Amphipod {
  fn target(&self) -> usize {
    match self {
      Amphipod::Amber => 3,
      Amphipod::Bronze => 5,
      Amphipod::Copper => 7,
      Amphipod::Desert => 9,
    }
  }
  fn cost(&self) -> i64 {
    match self {
      Amphipod::Amber => 1,
      Amphipod::Bronze => 10,
      Amphipod::Copper => 100,
      Amphipod::Desert => 1000,
    }
  }
}
impl std::fmt::Display for Amphipod {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Amphipod::Amber => write!(f,"A"),
      Amphipod::Bronze => write!(f,"B"),
      Amphipod::Copper => write!(f,"C"),
      Amphipod::Desert => write!(f,"D"),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{Gamestate,parse};

  #[test]
  fn test_1() { 
    let input:Vec<String> = INPUT1.split('\n').into_iter().map(String::from).collect();
    let board = Gamestate{board: parse(&input)};
    // board.print_board();
    assert_eq!(12521,board.shortest_path());
  }
  #[test]
  fn test_2() {
    let input:Vec<String> = INPUT2.split('\n').into_iter().map(String::from).collect();
    let board = Gamestate{board: parse(&input)};
    // board.print_board();
    assert_eq!(44169,board.shortest_path());
  }

  static INPUT1: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

  static INPUT2: &str = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";
}