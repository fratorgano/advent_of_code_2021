pub fn part1(input:&[String]) -> usize {
  let floor = parse(input);
  simulate(floor)
}

fn parse(input:&[String]) -> Vec<Vec<u8>> {
  let mut floor = vec![];
  for s in input.iter() {
    let mut row = vec![];
    for c in s.chars() {
      let p = match c {
        '.' => 0,
        '>' => 1,
        'v' => 2,
        _ => panic!("Unexpected char during input partsing: {}",c)
      };
      row.push(p);
    }
    floor.push(row);
  }
  floor
}

fn simulate(mut floor:Vec<Vec<u8>>)-> usize {
  let floor = &mut floor;
  let mut i = 0;
  let mut move_e = true;
  let mut move_s = true;
  while move_e || move_s {
    move_e = move_east(floor);
    move_s = move_south(floor);
    i+=1;
  }
  i
}

fn move_east(floor:&mut Vec<Vec<u8>>) -> bool {
  let mut new_floor = vec![vec![0;floor[0].len()];floor.len()];
  let mut moved = false;
  for r in 0..floor.len() {
    for c in 0..floor[0].len() {
      match floor[r][c] {
        1 => {
          let new_c = (c+1)%floor[0].len();
          if floor[r][new_c] == 0 {
            new_floor[r][new_c] = 1;
            moved = true;
          }else{
            new_floor[r][c] = 1;
          }
        }
        2 => new_floor[r][c] = 2,
        _ => {}
      }
    }
  }
  *floor = new_floor;
  moved
}

fn move_south(floor:&mut Vec<Vec<u8>>) -> bool {
  let mut new_floor = vec![vec![0;floor[0].len()];floor.len()];
  let mut moved = false;
  for r in 0..floor.len() {
    for c in 0..floor[0].len() {
      match floor[r][c] {
        2 => {
          let new_r = (r+1)%floor.len();
          if floor[new_r][c] == 0 {
            new_floor[new_r][c] = 2;
            moved = true;
          }else{
            new_floor[r][c] = 2;
          }
        }
        1 => new_floor[r][c] = 1,
        _ => {}
      }
    }
  }
  *floor = new_floor;
  moved
}


#[cfg(test)]
mod tests {
  use super::{parse,simulate};
  #[test]
  fn test() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(String::from).collect();
    let floor = parse(&input);
    assert_eq!(58,simulate(floor))
  }

  static INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
}