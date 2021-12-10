pub fn part1(input:&[String]) -> usize {
  find_illegal(input)
}

pub fn part2(input:&[String]) -> usize {
  fix_incomplete(input)
}

fn find_illegal(input:&[String]) -> usize {
  let mut errors = vec![];
  for line in input {
    let mut stack = vec![];
    for c in line.chars() {
      if c=='(' || c=='['|| c=='{'|| c=='<' {
        stack.push(c);
      }
      if c==')' || c==']'|| c=='}'|| c=='>' {
        if let Some(v) = stack.pop()  {
          if check_open_close(v, c) {
            errors.push(c);
            break;
          }
        }
      }
    }
  }
  calc_score_errors(errors)
}

fn fix_incomplete(input:&[String]) -> usize {
  // find completitions for the incomplete
  let mut completions = vec![];
  for line in input {
    let mut stack = vec![];
    let mut wrong = false;
    for c in line.chars() {
      if c=='(' || c=='['|| c=='{'|| c=='<' {
        stack.push(c);
      }
      if c==')' || c==']'|| c=='}'|| c=='>' {
        if let Some(v) = stack.pop()  {
          if check_open_close(v, c) {
            wrong=true;
            break;
          }
        }
      }
    }
    if !wrong {
      completions.push(generate_completions(stack));
    }
  }
  // calculate score
  let mut scores:Vec<usize> = completions.into_iter().map(calc_score_completetions).collect();
  scores.sort_unstable();
  *scores.get(scores.len()/2).unwrap()
}

fn generate_completions(missing:Vec<char>) -> Vec<char> {
  let mut completions = vec![];
  for c in missing {
    match c {
      '(' => completions.push(')'),
      '[' => completions.push(']'),
      '{' => completions.push('}'),
      '<' => completions.push('>'),
      _ => panic!("Unexpected character")
    }
  }
  completions.reverse();
  completions
}

fn calc_score_completetions(compl:Vec<char>) -> usize {
  let mut score = 0;
  for c in compl {
    score *= 5;
    match c {
      ')' => score+=1,
      ']' => score+=2,
      '}' => score+=3,
      '>' => score+=4,
      _ => panic!("Unexpected character")
    }
  }
  score
}

fn check_open_close(open:char,close:char) -> bool {
  match open {
    '(' => close != ')',
    '[' => close != ']',
    '{' => close != '}',
    '<' => close != '>',
    _ => panic!("Unexpected character")
  }
}

fn calc_score_errors(errors:Vec<char>) -> usize {
  let mut score = 0;
  for c in errors {
    match c {
      ')' => score+=3,
      ']' => score+=57,
      '}' => score+=1197,
      '>' => score+=25137,
      _ => panic!("Unexpected character")
    }
  }
  score
}

#[cfg(test)]
mod tests {
  use super::{find_illegal,fix_incomplete};
  #[test]
  fn test_part1() {
    let string = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]"
    ;
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    assert_eq!(26397,find_illegal(&input));
  }
  #[test]
  fn test_part2() {
    let string = "[({(<(())[]>[[{[]{<()<>>
      [(()[<>])]({[<{<<[]>>(
      {([(<{}[<>[]}>{[]{[(<()>
      (((({<>}<{<{<>}{[]{[]{}
      [[<[([]))<([[{}[[()]]]
      [{[{({}]{}}([{[{{{}}([]
      {<[[]]>}<{[{[{[]{()[[[]
      [<(<(<(<{}))><([]([]()
      <{([([[(<>()){}]>(<<{{
      <{([{{}}[<[[[<>{}]]]>[]]"
  ;
  let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    assert_eq!(288957,fix_incomplete(&input));
  }
}