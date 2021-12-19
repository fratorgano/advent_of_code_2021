use std::collections::HashSet;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  let (map,_scanners) = create_map(parsed);
  map.len()
}

pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let (_map,scanners) = create_map(parsed);
  find_max_distance(scanners)
}

fn parse(input:&[String]) -> Vec<Vec<(i64,i64,i64)>> {
  let mut res = vec![];
  let mut row = vec![];
  for l in input {
    if l.is_empty() {
      res.push(row);
      row = vec![];
    }else if !l.contains("scanner") {
        let mut split = l.split(',');
        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();
        let z = split.next().unwrap().parse::<i64>().unwrap();
        row.push((x,y,z))
      }
  }
  res.push(row);
  res
}

fn find_max_distance(scanners:Vec<(i64,i64,i64)>) -> usize {
  // find max of manhattan distances between scanners
  let mut max = 0;
  for s1 in &scanners {
    for s2 in &scanners {
      let dist = calc_distance(s1,s2);
      if dist>max {
        max = dist;
      }
    }
  }
  max
}

fn calc_distance((x1,y1,z1):&(i64,i64,i64),(x2,y2,z2):&(i64,i64,i64)) -> usize {
  // calc manhattan distance between 2 points
  ((x1-x2).abs() + (y1-y2).abs() + (z1-z2).abs()) as usize
}


fn create_map(mut scanners: Vec<Vec<(i64,i64,i64)>>) -> (HashSet<(i64,i64,i64)>,Vec<(i64,i64,i64)>) {
  let mut map = HashSet::new();
  let mut scanner_positions = vec!();
  // initialize hashset with first set of beacon's positions
  for i in &scanners[0] {
    map.insert(*i);
  }
  // keep iterating until every scanner's beacon has been aligned
  loop {
    for i in 0..scanners.len() {
      // try to align
      if let Some((map_update,scanner_pos)) = align(&map,scanners[i].to_vec()) {
        // if successful, update map, save scanner position and remove current batch of beacon from the list
        scanner_positions.push(scanner_pos);
        map.extend(map_update);
        scanners.remove(i);
        break;
      }
    }
    // when every beacon has been aligned, exit
    if scanners.is_empty() {
      break;
    }
  }
  // return position of all the beacons and the position of the scanners
  (map,scanner_positions)
  
}
fn align(curr_map:&HashSet<(i64,i64,i64)>, scan:Vec<(i64,i64,i64)>) -> Option<(Vec<(i64,i64,i64)>,(i64,i64,i64))> {
  // for every possible rotation, try to find if it aligns
  for rot in 0..24 {
    // rotate current beacon batch
    let rotated:Vec<(i64,i64,i64)> = scan.iter().map(|x| rotate(*x,rot)).collect();
    // find the coordinates difference between curr_map and rotated
    // which is basically every possible position of the scanner that made those points
    let mut difference = vec![];
    for (x1,y1,z1) in curr_map {
      for (x2,y2,z2) in &rotated {
        difference.push((x1-x2,y1-y2,z1-z2));
      }
    }
    // for each difference value, check if it aligns
    for (dx,dy,dz) in difference {
      // translate rotated points by difference and check if atleast 12 align
      let translated = rotated.iter().map(|(x,y,z)| (x+dx,y+dy,z+dz));
      let mut count = 0;
      for v in translated.clone() {
        if curr_map.contains(&v){
          count+=1;
        }
      }
      if count>=12 {
        // if atleast 12 align, return the new map points and the position of the scanner
        return Some((translated.collect(),(dx,dy,dz)))
      }
    }
  }
  None
}

fn rotate((x,y,z): (i64,i64,i64), rot: usize) -> (i64,i64,i64) {
  match rot {
    0  => ( x,  y,  z),
    1  => ( x, -y, -z),
    2  => ( x,  z, -y),
    3  => ( x, -z,  y),
    4  => (-x,  y, -z),
    5  => (-x, -y,  z),
    6  => (-x,  z,  y),
    7  => (-x, -z, -y),
    8  => ( y,  x, -z),
    9  => ( y,  z,  x),
    10 => ( y, -x,  z),
    11 => ( y, -z, -x),
    12 => (-y,  x,  z),
    13 => (-y, -x, -z),
    14 => (-y,  z, -x),
    15 => (-y, -z,  x),
    16 => ( z,  x,  y),
    17 => ( z,  y, -x),
    18 => ( z, -x, -y),
    19 => ( z, -y,  x),
    20 => (-z,  x, -y),
    21 => (-z,  y,  x),
    22 => (-z, -x,  y),
    23 => (-z, -y, -x),
    _ => panic!("Unexpected rotation value")
  }
}


#[cfg(test)]
mod tests {
  use super::{parse,create_map,find_max_distance};

  #[test]
  fn test() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let parsed = parse(&input);
    let (map,scanners) = create_map(parsed);
    assert_eq!(79,map.len());
    assert_eq!(3621,find_max_distance(scanners));
  }

  static INPUT: &str = "--- scanner 0 ---
  404,-588,-901
  528,-643,409
  -838,591,734
  390,-675,-793
  -537,-823,-458
  -485,-357,347
  -345,-311,381
  -661,-816,-575
  -876,649,763
  -618,-824,-621
  553,345,-567
  474,580,667
  -447,-329,318
  -584,868,-557
  544,-627,-890
  564,392,-477
  455,729,728
  -892,524,684
  -689,845,-530
  423,-701,434
  7,-33,-71
  630,319,-379
  443,580,662
  -789,900,-551
  459,-707,401
  
  --- scanner 1 ---
  686,422,578
  605,423,415
  515,917,-361
  -336,658,858
  95,138,22
  -476,619,847
  -340,-569,-846
  567,-361,727
  -460,603,-452
  669,-402,600
  729,430,532
  -500,-761,534
  -322,571,750
  -466,-666,-811
  -429,-592,574
  -355,545,-477
  703,-491,-529
  -328,-685,520
  413,935,-424
  -391,539,-444
  586,-435,557
  -364,-763,-893
  807,-499,-711
  755,-354,-619
  553,889,-390
  
  --- scanner 2 ---
  649,640,665
  682,-795,504
  -784,533,-524
  -644,584,-595
  -588,-843,648
  -30,6,44
  -674,560,763
  500,723,-460
  609,671,-379
  -555,-800,653
  -675,-892,-343
  697,-426,-610
  578,704,681
  493,664,-388
  -671,-858,530
  -667,343,800
  571,-461,-707
  -138,-166,112
  -889,563,-600
  646,-828,498
  640,759,510
  -630,509,768
  -681,-892,-333
  673,-379,-804
  -742,-814,-386
  577,-820,562
  
  --- scanner 3 ---
  -589,542,597
  605,-692,669
  -500,565,-823
  -660,373,557
  -458,-679,-417
  -488,449,543
  -626,468,-788
  338,-750,-386
  528,-832,-391
  562,-778,733
  -938,-730,414
  543,643,-506
  -524,371,-870
  407,773,750
  -104,29,83
  378,-903,-323
  -778,-728,485
  426,699,580
  -438,-605,-362
  -469,-447,-387
  509,732,623
  647,635,-688
  -868,-804,481
  614,-800,639
  595,780,-596
  
  --- scanner 4 ---
  727,592,562
  -293,-554,779
  441,611,-461
  -714,465,-776
  -743,427,-804
  -660,-479,-426
  832,-632,460
  927,-485,-438
  408,393,-506
  466,436,-512
  110,16,151
  -258,-428,682
  -393,719,612
  -211,-452,876
  808,-476,-593
  -575,615,604
  -485,667,467
  -680,325,-822
  -627,-443,-432
  872,-547,-609
  833,512,582
  807,604,487
  839,-516,451
  891,-625,532
  -652,-548,-490
  30,-46,-14";
}