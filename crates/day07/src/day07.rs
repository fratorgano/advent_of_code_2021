pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  align_crabs(parsed,calculate_fuel_cost)
}

pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  align_crabs(parsed,calculate_fuel_cost_incremental)
}

fn parse(input:&[String]) -> Vec<usize> {
  input[0].split(',').map(|s| s.parse().unwrap()).collect()
}

fn align_crabs(crabs:Vec<usize>, fuel_function:fn(&[usize],usize)->usize) -> usize {
  // gradient search starting from average
  let mut position = (crabs.iter().sum::<usize>() as f32 / crabs.len() as f32).floor() as usize;
  let fuel_avg = fuel_function(&crabs, position);
  let increment;
  if fuel_function(&crabs, position-1)<fuel_avg {
    increment = false
  }else {
    increment = true
  }
  let mut curr_fuel = fuel_function(&crabs, position);
  loop {
    if increment {
      position += 1;
    } else {
      position -= 1;
    }
    let fuel = fuel_function(&crabs, position);
      if fuel>curr_fuel {
        break;
      }
      curr_fuel = fuel;
  }
  curr_fuel
}

fn calculate_fuel_cost(crabs:&[usize], starting_position:usize) -> usize {
  let mut fuel = 0;
  for c in crabs {
    fuel += (*c as i64 - starting_position as i64).abs() as usize
  }
  fuel
}

fn calculate_fuel_cost_incremental(crabs:&[usize], starting_position:usize) -> usize {
  let mut fuel = 0;
  for c in crabs {
    let movement = (*c as i64 - starting_position as i64).abs() as usize;
    fuel += movement * (movement + 1) / 2;
  }
  fuel
}

#[cfg(test)]
mod tests {
  use super::{parse,align_crabs,calculate_fuel_cost,calculate_fuel_cost_incremental};
  #[test]
  fn test_part1() {
    let strings = ["16,1,2,0,4,2,7,1,2,14"];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input);
    let result = align_crabs(parsed, calculate_fuel_cost);
    assert_eq!(37,result);
  }
  #[test]
  fn test_part2() {
    let strings = ["16,1,2,0,4,2,7,1,2,14"];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse(&input);
    let result = align_crabs(parsed, calculate_fuel_cost_incremental);
    assert_eq!(168,result);
  }
}