use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse_1(input);
  count_1_4_7_8_output(parsed)
}

pub fn part2(input:&[String]) -> usize {
  let parsed = parse_2(input);
  decode_sum(parsed)
}

fn parse_1(input:&[String]) -> Vec<Vec<String>> {
  let mut parsed = vec![];
  for line in input {
    parsed.push(line.split(" | ").nth(1).unwrap().split(' ').map(String::from).collect::<Vec<String>>())
  }
  parsed
}
fn parse_2(input:&[String]) -> Vec<(Vec<String>,Vec<String>)> {
  let mut parsed = vec![];
  for line in input {
    let mut split = line.split(" | ");
    let i = split.next().unwrap().split(' ').map(|s| {
      let mut chars = s.chars().collect::<Vec<char>>();
      chars.sort_unstable();
      String::from_iter(chars)
    }
    ).collect::<Vec<String>>();
    let o = split.next().unwrap().split(' ').map(|s| {
      let mut chars = s.chars().collect::<Vec<char>>();
      chars.sort_unstable();
      String::from_iter(chars)
    }
    ).collect::<Vec<String>>();
    parsed.push((i,o))
  }
  parsed
}

fn decode_sum(digits:Vec<(Vec<String>,Vec<String>)>) -> usize {
  let mut sum = 0;
  for (i,o) in &digits{
    let translate_table = deduce((i.to_owned(),o.to_owned()));
    sum += o.iter().map(|s| translate_table.get(s).unwrap()).fold(0,|out,digit| out * 10 + digit);
  }
  sum
}

fn deduce((i,o):(Vec<String>,Vec<String>)) -> HashMap<String,usize>{
  let mut translate_table = HashMap::new();
  let all = [i,o].concat();

  // Find 1,4,7,8
  let str_1 = String::from(all.iter().find(|s| s.len()==2).unwrap());
  let str_4 = String::from(all.iter().find(|s| s.len()==4).unwrap());
  let str_7 = String::from(all.iter().find(|s| s.len()==3).unwrap());
  let str_8 = String::from(all.iter().find(|s| s.len()==7).unwrap());

  // Looked up some help on the subreddit cause I got stuck after figuring out 9 in the wrong way

  // Find 9, contains all of 4
  let str_9 = String::from(all.iter().find(|s|  s.len()==6 && str_4.chars().find(|&c| !s.contains(c))== None).unwrap());
  let bottom = str_9.chars().find(|&c| !str_4.contains(c) && !str_7.contains(c)).unwrap();
  let bottom_left = str_8.chars().find(|&c| !str_9.contains(c)).unwrap();

  // Find 3, contains all of 7
  let str_3 = String::from(all.iter().find(|&s| s.len() == 5 && str_7.chars().find(|&c| !s.contains(c))== None).unwrap());
  let middle = str_3.chars().find(|&c| !str_7.contains(c) && c!=bottom).unwrap();
  let top_left = str_9.chars().find(|&c| !str_3.contains(c)).unwrap();

  // Find 0, only six length that doesn't have middle
  let str_0 = String::from(all.iter().find(|&s| s.len() == 6 && !s.contains(middle)).unwrap());

  // Find 6, only six length to have both middle and bottom_left
  let str_6 = String::from(all.iter().find(|&s| s.len() == 6 && s.contains(middle) && s.contains(bottom_left)).unwrap());

  // Find 2, only five length with bottom_left
  let str_2 = String::from(all.iter().find(|&s| s.len() == 5 && s.contains(bottom_left)).unwrap());

  // Find 5, only five length with top_left
  let str_5 = String::from(all.iter().find(|&s| s.len() == 5 && s.contains(top_left)).unwrap());
  
  translate_table.insert(str_0, 0);
  translate_table.insert(str_1, 1);
  translate_table.insert(str_2, 2);
  translate_table.insert(str_3, 3);
  translate_table.insert(str_4, 4);
  translate_table.insert(str_5, 5);
  translate_table.insert(str_6, 6);
  translate_table.insert(str_7, 7);
  translate_table.insert(str_8, 8);
  translate_table.insert(str_9, 9);
  translate_table
}

fn count_1_4_7_8_output(input:Vec<Vec<String>>) -> usize {
  let mut count = 0;
  for line in input{
    for word in line {
      if word.len() == 2 || word.len() == 4 || word.len() == 3 || word.len() == 7 {
        count+=1;
      }
    }
  }
  count
}

#[cfg(test)]
mod tests {
  use super::{parse_1, count_1_4_7_8_output, parse_2, decode_sum};
  #[test]
  fn test_part1() {
    let strings = ["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
    "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
    "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
    "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
    "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
    "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
    "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
    "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
    "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse_1(&input);
    let result = count_1_4_7_8_output(parsed);
    assert_eq!(26,result);
  }
  #[test]
  fn test_part2() {
    let strings = ["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
    "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
    "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
    "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
    "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
    "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
    "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
    "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
    "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"];
    let input:Vec<String> = strings.iter().map(|s| String::from(*s)).collect();
    let parsed = parse_2(&input);
    let result = decode_sum(parsed);
    assert_eq!(61229,result);
  }
}