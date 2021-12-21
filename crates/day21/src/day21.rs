use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  play(parse(input))
}

pub fn part2(input:&[String]) -> usize {
  play2(parse(input))
}

fn parse(input:&[String]) -> Vec<usize> {
  let mut res = vec![];
  for l in input {
    res.push(l.split(": ").nth(1).unwrap().parse().unwrap())
  }
  res
}

fn play(mut positions:Vec<usize>) -> usize {
  let mut scores = vec![0;positions.len()];
  let mut last_roll = 0;
  let mut tot_rolls = 0;
  let mut turn = 0;
  loop {
    let mut mov = 0;
    tot_rolls+=3;
    for _i in 0..3 {
      last_roll = (last_roll+1) % 100;
      mov += last_roll;
    }
    positions[turn] += mov;
    while positions[turn] > 10 { positions[turn] -= 10 }
    scores[turn] += positions[turn];
    if scores[turn]>=1000 {
      break;
    }
    turn = (turn+1)%2;
  }
  scores[(turn+1)%2]*tot_rolls
}

#[derive(std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash, Clone, Copy)]
struct GameState {
  s1:usize,
  s2:usize,
  p1:usize,
  p2:usize
}

fn dirac_game(game_state:GameState,store:&mut HashMap<GameState,(usize,usize)>) -> (usize,usize) {
  if game_state.s1 >= 21 {return (1,0);}
  if game_state.s2 >= 21 {return (0,1);}
  if let Some(&result) = store.get(&game_state) {
    return result
  };
  let mut result = (0,0);
  // There's probably some optimization that can be done here, using the distribution of dices rolls
  // since there are only 7 (3,4,5,6,7,8,9) possible results of 3 rolls of d3 while I am doing 3**3
  for d1 in [1,2,3] {
    for d2 in [1,2,3] {
      for d3 in [1,2,3] {
        let mov = d1+d2+d3;
        let mut new_p1 = game_state.p1 + mov;
        if new_p1>10 {
          new_p1 -= 10
        }
        // switching players for next turn so I only need to work with p1 and s1
        let next_game_state = GameState{
          p1: game_state.p2,
          p2: new_p1,
          s1: game_state.s2,
          s2: game_state.s1 + new_p1
        };
        let (s1,s2) = dirac_game(next_game_state, store);
        result.0 += s2;
        result.1 += s1;
      }
    }
  }
  store.insert(game_state, result);
  result
}

fn play2(players:Vec<usize>) -> usize {
  let game_state = GameState {
    s1: 0,
    s2: 0,
    p1: players[0],
    p2: players[1]
  };
  let mut store:HashMap<GameState,(usize,usize)> = HashMap::new();
  let (s0,s1) = dirac_game(game_state, &mut store);
  s0.max(s1)
}

#[cfg(test)]
mod tests {
  use super::{parse,play,play2};

  #[test]
  fn test_1() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let players = parse(&input);
    assert_eq!(739785,play(players)); 
  }
  #[test]
  fn test_2() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let players = parse(&input);
    assert_eq!(444356092776315,play2(players)); 
  }

  static INPUT: &str = "Player 1 starting position: 4
  Player 2 starting position: 8";
}