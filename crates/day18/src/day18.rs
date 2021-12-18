pub fn part1(input:&[String]) -> usize {
  let s_numbers = parse(input);
  calc_sum_magnitude(s_numbers)
}

pub fn part2(input:&[String]) -> usize {
  let s_numbers = parse(input);
  calc_largest_magnitude(s_numbers)
}

fn calc_sum_magnitude(s_numbers:Vec<SNumber>) -> usize {
  let mut sum = s_numbers[0].clone();
  for n in s_numbers.iter().skip(1) {
    sum = (sum + n.clone()).reduce();
  }
  sum.magnitude()
}

fn calc_largest_magnitude(s_numbers:Vec<SNumber>) -> usize {
  let mut max = 0;
  for i in 0..s_numbers.len() {
    for j in 0..s_numbers.len() {
      if i!=j {
        max = max.max((s_numbers[i].clone() + s_numbers[j].clone()).reduce().magnitude());
      }
    }
  }
  max
}

fn parse(input:&[String]) -> Vec<SNumber> {
  input.iter().map(|s| SNumber::parse(s.to_string())).collect()
}

#[derive(Debug, Clone)]
enum SNumber {
  Pair(Box<SNumber>, Box<SNumber>),
  Value(isize),
}
impl std::fmt::Display for SNumber {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      SNumber::Value(n) => write!(f,"{}",n),
      SNumber::Pair(l, r) => {
            write!(f,"[{},{}]",l,r)
      }
    }
  }
}

impl std::ops::Add for SNumber {
  type Output = Self;
  fn add(self,b:Self) -> Self {
    SNumber::Pair(Box::new(self),Box::new(b))
  }
}

impl SNumber {
  fn parse(inp: String) -> SNumber {
    if let Ok(a) = inp.parse::<isize>() {
      SNumber::Value(a)
    } else {
        let mut position = 0;
        let mut open = 0;
        for (i, ch) in inp.chars().enumerate() {
            match ch {
                '[' => open += 1,
                ']' => open -= 1,
                ',' if open == 1 =>  position = i,
                _ => {}
            }
        }
        SNumber::parse(inp[1..position].to_owned()) + SNumber::parse(inp[position+1..inp.chars().count()-1].to_owned())
    }
  }
  fn add_left(self, val: isize) -> Self {
    match self {
        SNumber::Value(n) => SNumber::Value(n + val),
        SNumber::Pair(l, r) => SNumber::Pair(Box::new(l.add_left(val)), r),
    }
  }
  fn add_right(self, val: isize) -> Self {
    match self {
        SNumber::Value(n) => SNumber::Value(n + val),
        SNumber::Pair(l, r) => SNumber::Pair(l, Box::new(r.add_right(val))),
    }
  }
  fn reduce(self) -> Self {
    let mut number = self;
    loop {
      let (next_number, res) = number.explode(0);
      number = next_number;
      if res.is_some() {
        continue;
      }
      let (next_number, res) = number.split();
      number = next_number;
      if !res {
        break;
      }
    }
    number
  }

  fn explode(self, depth:usize) -> (Self, Option<(Option<isize>, Option<isize>)>) {
    match self {
      SNumber::Pair(l,r) => {
        match (*l, *r) {
          (SNumber::Value(nl), SNumber::Value(nr)) if depth>=4 => {
            (SNumber::Value(0),Some((Some(nl),Some(nr))))
          }
          (l,r) => match l.explode(depth+1) {
            (l_reduced, Some((l_explode, r_explode))) => {
              let r_added = if let Some(r_explode) = r_explode {
                r.add_left(r_explode)
              } else {
                r
              };
              (
                SNumber::Pair(Box::new(l_reduced),Box::new(r_added)),
                Some((l_explode, None))
              )
            }
            (l_reduced, None) => match r.explode(depth+1) {
              (r_reduced, Some((l_explode,r_explode))) => {
                let l_added = if let Some(l_explode) = l_explode {
                  l_reduced.add_right(l_explode)
                } else {
                  l_reduced
                };
                (
                  SNumber::Pair(Box::new(l_added),Box::new(r_reduced)),
                  Some((None,r_explode))
                )
              }
              (r_reduced, None) => {
                (SNumber::Pair(Box::new(l_reduced),Box::new(r_reduced)),None)
              }
            }
          }
        }
      }
      SNumber::Value(_) => {
        (self,None)
      }
    }
  }
  fn split(self) -> (Self,bool){
    match self {
      SNumber::Pair(l,r) => {
        let (l_split,l_is_split) = l.split();
        if l_is_split {
          (SNumber::Pair(Box::new(l_split),r),true)
        } else {
          let (r_split,r_is_split) = r.split();
          (SNumber::Pair(Box::new(l_split),Box::new(r_split)),r_is_split)
        }
      }
      SNumber::Value(n) if n >= 10 => (
        Self::Pair(
            Box::new(SNumber::Value(n / 2)),
            Box::new(SNumber::Value(if n % 2 == 0 { n / 2 } else { n / 2 + 1 })),
        ),
        true,
        ),
        SNumber::Value(_) => (self, false),
      }
  }
  fn magnitude(&self) -> usize {
    match self {
      SNumber::Value(n) => *n as usize,
      SNumber::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{SNumber,parse,calc_largest_magnitude};
  static INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
  [[[5,[2,8]],4],[5,[[9,9],0]]]
  [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
  [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
  [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
  [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
  [[[[5,4],[7,7]],8],[[8,3],8]]
  [[9,3],[[9,9],[6,[4,9]]]]
  [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
  [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
  static INPUT2: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
  [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
  [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
  [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
  [7,[5,[[3,8],[1,4]]]]
  [[2,[2,2]],[8,[8,1]]]
  [2,9]
  [1,[[[9,3],9],[[9,0],[0,7]]]]
  [[[5,[7,4]],7],1]
  [[[[4,2],2],6],[8,7]]";

  #[test]
  fn test_sum_1() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let s_numbers = parse(&input);
    println!("{:?}",s_numbers[0]);
    let sum = s_numbers
            .clone()
            .into_iter()
            .reduce(|l, r| (l + r.clone()).reduce())
            .unwrap();
    assert_eq!("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",sum.to_string());
    assert_eq!(4140,sum.magnitude())
  }
  #[test]
  fn test_sum_2() {
    let input:Vec<String> = INPUT2.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let s_numbers = parse(&input);
    let sum = s_numbers
            .clone()
            .into_iter()
            .reduce(|l, r| {
              let answer = (l + r.clone()).reduce();
              println!("{}",answer.to_string());
              answer
            })
            .unwrap();
    assert_eq!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",sum.to_string());
  }
  #[test]
  fn test_magnitude() {
    assert_eq!(143,SNumber::parse("[[1,2],[[3,4],5]]".to_string()).magnitude());
    assert_eq!(1384,SNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string()).magnitude());
    assert_eq!(3488,SNumber::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string()).magnitude());
  }
  #[test]
  fn test_max_magnitude() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let s_numbers = parse(&input);
    let res = calc_largest_magnitude(s_numbers);
    assert_eq!(3993,res);

  }
}