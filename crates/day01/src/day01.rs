pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  count_increases(&parsed)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  count_sliding_increases(&parsed)
}

fn parse(strings:&[String]) -> Vec<usize> {
  strings.iter().map(|s| s.parse::<usize>().unwrap()).collect()
}

fn count_increases(numbers:&[usize]) -> usize {
  let mut increases = 0;
  let mut before = numbers[0];
  for n in numbers.iter() {
    if *n > before {increases+=1}
    before = *n;
  } 
  increases
}

fn count_sliding_increases(numbers:&[usize]) -> usize {
  // could have used numbers.windows(n)
  let mut increases = 0;
  let mut windows = vec![];
  for (i,n) in numbers.iter().enumerate() {
    if let Some(n2) = numbers.get(i+1) {
      if let Some(n3) = numbers.get(i+2) {
        let window = n + n2 + n3;
        windows.push(window);
      }
    }
  }
  let mut before = windows[0];
  for w in windows {
    if w>before {increases+=1}
    before = w;
  } 
  increases
}