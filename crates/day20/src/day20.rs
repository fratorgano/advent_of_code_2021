pub fn part1(input:&[String]) -> usize {
  let (iea,image) = parse(input);
  let final_image = enhance_passes(iea,image,2);
  count_lit(&final_image)
}

pub fn part2(input:&[String]) -> usize {
  let (iea,image) = parse(input);
  let final_image = enhance_passes(iea,image,50);
  count_lit(&final_image)
}
// reimplemented using Vecs after completing it with HashMap cause they were really slow, about 7 seconds for part2
// original, slow, version is below
fn parse(input:&[String]) -> ([u8;512],Vec<Vec<u8>>) {
  let mut iea = [0;512];
  let mut image = vec![];
  for (i,c) in input[0].chars().enumerate() {
    match c {
      '.' => iea[i] = 0,
      '#' => iea[i] = 1,
      _ => panic!("Unexpected character found")
    }
  }
  for line in input.iter().skip(2) {
    let mut row = vec![];
    for c in line.chars() {
      match c {
        '.' => row.push(0),
        '#' => row.push(1),
        _ => panic!("Unexpected character found")
      }
    }
    image.push(row);
  }
  (iea,image)
}

fn count_lit(image:&[Vec<u8>]) -> usize {
  let mut count = 0;
  for r in image {
    for v in r {
      count += *v as usize;
    }
  }
  count
}

fn enhance_passes(iea:[u8;512],mut image:Vec<Vec<u8>>,passes:u8) -> Vec<Vec<u8>> {
  for i in 0..passes {
    if i%2==1 {
      image = enhance(iea,image,iea[0]);
    } else {
      image = enhance(iea,image,0);
    }
  }
  image
}

fn enhance(iea:[u8;512],image:Vec<Vec<u8>>,default:u8) -> Vec<Vec<u8>> {
  let mut new_image = vec![vec![0;image[0].len()+2];image.len()+2];
  for r in 0..new_image.len() {
    for c in 0..new_image[0].len() {
      // edited a version found on reddit to not fail compilation with 
      // subtraction with overflow, it's about 10ms slower
      let adjacents = adjacent(r as isize -1,c as isize -1);
      //println!("{:?}",adjacents);
  
      let i = adjacents.iter().fold(0, |n, &(r1,c1)| {
        if r1<0 || c1<0 || r1>=(image.len() as isize) || c1>=(image[0].len() as isize) {
          let x = default as usize;
          n << 1 | x
        } else {
          let x = image[(r1) as usize][(c1) as usize] as usize;
          n << 1 | x
        }
      });
      new_image[r][c] = iea[i];
    }
  }
  new_image
}

fn adjacent(x: isize, y: isize) -> [(isize,isize);9] {
  [
    (x-1,y-1),
    (x-1,y),
    (x-1,y+1),

    (x,y-1),
    (x,y),
    (x,y+1),

    (x+1,y-1),
    (x+1,y),
    (x+1,y+1)
  ]
}

/* fn enhance_passes(iea:[char;512],mut image:HashMap<(isize,isize),char>,passes:usize) -> HashMap<(isize,isize),char> {
  for i in 0..passes {
    if i%2==1 {
      image = enhance(iea,image,&iea[0]);
    } else {
      image = enhance(iea,image,&'0');
    }
  }
  image
}

fn enhance(iea:[char;512],image:HashMap<(isize,isize),char>,default:&char) -> HashMap<(isize,isize),char> {
  let mut image_res = image.clone();
  for k in image.keys() {
    let adj = adjacent(k.0, k.1);
    for(x,y) in adj {
      let mut binary = String::new();
      let adj2 = adjacent(x,y);
      for (x1,y1) in adj2 {
        let value = image.get(&(y1,x1)).unwrap_or(default);
        binary = format!("{}{}",binary,value);
      }
      let i = binary_to_uint(binary.clone());
      
      image_res.insert((y,x), iea[i]);
    }
  }
  image_res
}

fn count_lit(input:&HashMap<(isize,isize),char>) -> usize {
  let mut count = 0;
  for v in input.values() {
    if v == &'1'{
      count+=1
    }
  }
  count
}


fn binary_to_uint(mut input:String) -> usize {
  let mut answer = 0;
  for c in input.chars() {
    answer <<= 1;
    if c=='1' {
      answer += 1;
    }
  }
  answer
} */


#[cfg(test)]
mod tests {
  use super::{parse,enhance_passes,count_lit};

  #[test]
  fn test_1() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (iea,image) = parse(&input);
    let final_image = enhance_passes(iea,image,2);
    assert_eq!(35,count_lit(&final_image)); 
  }
  #[test]
  fn test_2() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let (iea,image) = parse(&input);
    let final_image = enhance_passes(iea,image,50);
    assert_eq!(3351,count_lit(&final_image)); 
  }

  static INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
  
  #..#.
  #....
  ##..#
  ..#..
  ..###";
}