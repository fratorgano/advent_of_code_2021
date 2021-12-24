use std::collections::HashMap;

// Slow solution made only to test if I could get my answer from pc after doing it on paper
// I transcribed a bit of my paper solution in 24.txt

pub fn part1(input:&[String]) -> String {
  let program = parse(input);
  program.find_max_model()
}

pub fn part2(input:&[String]) -> String {
  let program = parse(input);
  program.find_min_model()
}

fn parse(input:&[String]) -> Program {
  let mut instructions = vec![];
  for l in input {
    let mut split = l.split(' ');
    let op = split.next().unwrap();
    let var_c = split.next().unwrap().chars().next().unwrap();
    let var = match var_c{
      'w' => 0,
      'x' => 1,
      'y' => 2,
      'z' => 3,
      _ => panic!("Unexected char: {}",var_c)
    };
    let instruction = match op {
      "inp" => {
        Op::Inp(var)
      },
      _ => {
        let value = split.next().unwrap();
        let v1 = Value::parse(value);
        match op {
          "add" => Op::Add(var,v1),
          "mul" => Op::Mul(var,v1),
          "div" => Op::Div(var,v1),
          "mod" => Op::Mod(var,v1),
          "eql" => Op::Eql(var,v1),
          _ => panic!("Unexpected input: {}",op)
        }
      }
    };
    instructions.push(instruction);
  }
  Program{instructions}
}

#[derive(Debug)]
struct Program {
  instructions:Vec<Op>
}
impl Program {
  fn find_max_model(&self) -> String {
    let range = [9,8,7,6,5,4,3,2,1];
    let blocks = self.instructions.chunks(18).map(|c| c.iter().copied().collect()).collect::<Vec<_>>();
    let res = Program::find_model(&blocks,0,0,&range,&mut HashMap::new()).unwrap();
    res.to_string().chars().rev().collect()
  }
  fn find_min_model(&self) -> String {
    let range = [1,2,3,4,5,6,7,8,9];
    let blocks = self.instructions.chunks(18).map(|c| c.iter().copied().collect()).collect::<Vec<_>>();
    let res = Program::find_model(&blocks,0,0,&range,&mut HashMap::new()).unwrap();
    res.to_string().chars().rev().collect()
  }

  fn find_model(blocks:&[Vec<Op>],block:usize,z:isize,range:&[isize;9],cache:&mut HashMap<(isize, usize), Option<isize>>) -> Option<isize> {
    if let Some(&answer) = cache.get(&(z,block)) {return answer};

    for &digit in range {
      let mut reg = [0,0,0,z];
      for &inst in &blocks[block] {
        inst.run(&mut reg,Some(digit))
      }
      let z = reg[3];
      if block+1 == blocks.len() {
        if z==0 {
          cache.insert((z,block), Some(digit));
          return Some(digit);
        }
        continue
      }
      if let Some(best) = Program::find_model(blocks,block+1,z,range,cache) {
        cache.insert((z,block), Some(best*10+digit));
        return Some(best*10+digit)
      }
    }

    cache.insert((z,block), None);
    None
  }
}
impl std::fmt::Display for Program {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      for v in &self.instructions {
        let _ = writeln!(f, "{}", v);
      }
      Ok(())
  }
}


#[derive(Debug,Copy,Clone)]
enum Op {
  Inp(usize),
  Add(usize,Value),
  Mul(usize,Value),
  Div(usize,Value),
  Mod(usize,Value),
  Eql(usize,Value)
}
impl Op {
  fn run (&self,reg:&mut [isize;4], input:Option<isize>) {
    match self {
      Op::Inp(a) => {
        reg[*a] = input.unwrap();
      }
      Op::Add(a,b) => {
        reg[*a] += b.val(reg);
      }
      Op::Mul(a,b) => {
        reg[*a] *= b.val(reg);
      }
      Op::Div(a,b) => {
        reg[*a] /= b.val(reg);
      }
      Op::Mod(a,b) => {
        reg[*a] %= b.val(reg);
      }
      Op::Eql(a,b) => {
        reg[*a] = if reg[*a]==b.val(reg) {1} else {0};
      }
    }
  }
}
impl std::fmt::Display for Op {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Op::Inp(p) => write!(f,"inp {}",p),
      Op::Add(a,b) => write!(f,"add {} {}",a,b),
      Op::Mul(a,b) => write!(f,"mul {} {}",a,b),
      Op::Div(a,b) => write!(f,"div {} {}",a,b),
      Op::Mod(a,b) => write!(f,"mod {} {}",a,b),
      Op::Eql(a,b) => write!(f,"eql {} {}",a,b),
    }
  }
}

#[derive(Debug,Copy,Clone)]
enum Value {
  Var(usize),
  Num(isize)
}
impl Value {
  fn parse(s:&str) -> Value {
    match s {
      "w" => Value::Var(0),
      "x" => Value::Var(1),
      "y" => Value::Var(2),
      "z" => Value::Var(3),
      _   => Value::Num(s.parse().unwrap()),
    }
  }
  fn val(&self,regs:&[isize;4]) -> isize {
    match self {
      Value::Num(n) => *n,
      Value::Var(c) => regs[*c]
    }
  }
}
impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Value::Var(c) => write!(f,"{}",c),
      Value::Num(n) => write!(f,"{}",n)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Op,Value};

  #[test]
  fn test_op() { 
    let mut reg = [1,2,3,4];
    let i = Some(2);
    Op::Inp(0).run(&mut reg,i);
    Op::Add(1,Value::Num(4)).run(&mut reg,i);
    Op::Mul(1,Value::Var(1)).run(&mut reg,i);
    Op::Div(1,Value::Num(4)).run(&mut reg,i);
    Op::Mod(1,Value::Num(4)).run(&mut reg,i);
    Op::Eql(1,Value::Num(2)).run(&mut reg,i);

    assert_eq!(0,reg[1])
    
  }
}