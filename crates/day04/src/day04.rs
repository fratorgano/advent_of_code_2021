#[derive(Debug)]
struct Cell {
  value: usize,
  marked: bool
}
#[derive(Debug)]
struct Board {
  board: Vec<Vec<Cell>>,
  won: bool
}
impl Board {
  fn mark(&mut self, number:usize) -> bool{
    let mut won = false;
    let mut marked = false;
    for i in self.board.iter_mut() {
      let mut count_marked = 0;
      let i_len = i.len();
      for j in i {
        if j.value == number {
          marked = true;
          j.marked = true;
        }
        if j.marked {
          count_marked +=1
        }
        if count_marked == i_len {
          self.won = true;
          won = true;
        }
      }
    }
    if marked {
      for j in 0..5 {
        let mut count_true = 0;
        for i in 0..5 {
          if self.board[i][j].marked {
            count_true += 1;
          }
        }
        if count_true == 5 {
          self.won = true;
          return true
        }
      }
    }
    won
  }

  fn score(&mut self, number: usize) -> usize {
    let mut score = 0;
    for r in &self.board {
      for c in r {
        if !c.marked {
          score += c.value
        }
      }
    }
    score * number
  }

}

pub fn part1(input:&[String]) -> usize {
  let (draw_numbers,boards) = parse(input);
  play_bingo(draw_numbers, boards)
}
pub fn part2(input:&[String]) -> usize {
  let (draw_numbers,boards) = parse(input);
  play_bingo_2(draw_numbers, boards)
}

fn parse(input:&[String]) -> (Vec<usize>, Vec<Board>) {
  let draw_numbers:Vec<usize> = input[0].split(',').map(|s| s.parse::<usize>().unwrap()).collect();
  let mut boards:Vec<Board> = vec![]; 
  let mut board = vec![];
  for i in input.iter().skip(1) {
    if i.is_empty() {continue};
    let line:Vec<Cell> = i.split(' ').filter(|s| !s.is_empty()).map(|s| Cell {
      value: s.parse::<usize>().unwrap(),
      marked: false,
    }).collect();
    board.push(line);
    if board.len() == 5 {
      boards.push(Board {
        board,
        won: false,
      });
      board = vec![];
    }
  }
  (draw_numbers, boards)
}

fn play_bingo(draw_numbers:Vec<usize>,boards: Vec<Board>) -> usize{
  let mut boards = boards;
  for number in draw_numbers {
    for board in boards.iter_mut() {
      let won = board.mark(number);
      if won {
        return board.score(number)
      }
    }
  }
  1
}

fn play_bingo_2(draw_numbers:Vec<usize>,boards: Vec<Board>) -> usize{
  let mut boards = boards;
  let len = boards.len();
  let mut winners = 0;
  for number in draw_numbers {
    for board in boards.iter_mut() {
      if !board.won {
        let won = board.mark(number);
        if won {
          winners += 1;
          if winners == len {
            return board.score(number);
          }
        }
      }
    }
  }
  0
}

#[cfg(test)]
mod tests {
  use super::{parse, play_bingo, play_bingo_2};
  #[test]
  fn test_part1() {
    let input = ["7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
"",
"22 13 17 11  0",
" 8  2 23  4 24",
"21  9 14 16  7",
" 6 10  3 18  5",
" 1 12 20 15 19",
"",
" 3 15  0  2 22",
" 9 18 13 17  5",
"19  8  7 25 23",
"20 11 10 24  4",
"14 21 16 12  6",
"",
"14 21 17 24  4",
"10 16 15  9 19",
"18  8 23 26 20",
"22 11 13  6  5",
" 2  0 12  3  7"]
;
  let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
  let (draw_numbers,boards) = parse(&input2);
  assert_eq!(4512,play_bingo(draw_numbers, boards))
  }
  #[test]
  fn test_part2() {
    let input = ["7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
"",
"22 13 17 11  0",
" 8  2 23  4 24",
"21  9 14 16  7",
" 6 10  3 18  5",
" 1 12 20 15 19",
"",
" 3 15  0  2 22",
" 9 18 13 17  5",
"19  8  7 25 23",
"20 11 10 24  4",
"14 21 16 12  6",
"",
"14 21 17 24  4",
"10 16 15  9 19",
"18  8 23 26 20",
"22 11 13  6  5",
" 2  0 12  3  7"]
;
  let input2:Vec<String> = input.iter().map(|s| String::from(*s)).collect();
  let (draw_numbers,boards) = parse(&input2);
  assert_eq!(1924,play_bingo_2(draw_numbers, boards))
  }
}