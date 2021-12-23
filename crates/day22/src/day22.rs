use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  let cubes = parse(input);
  count_on(cubes)
}

pub fn part2(input:&[String]) -> i64 {
  let cubes = parse(input);
  count_on_all(cubes)
}

fn count_on (cubes:Vec<Cube>) -> usize {
  // this wasn't a smart solution, it worked for a part 1 but had to find another
  // way for part 2
  let mut reactor:HashMap<(i64,i64,i64),bool> = HashMap::new();
  for i in cubes {
    let pairs = create_pairs(i.x, i.y, i.z);
    let b = matches!(i.t, Status::On);
    for p in pairs {
      reactor.insert(p,b);
    }
  }
  let mut on_count = 0;
  for (_k,v) in reactor {
    if v {
      on_count+=1;
    }
  }
  on_count
}

fn count_on_all (cubes:Vec<Cube>) -> i64 {
  let mut new_cubes = vec![];
  for cube in cubes {
    let mut to_add = vec![];
    for n_i in &new_cubes {
      if let Some(ic) = intersection(&cube,n_i) {
        to_add.push(ic);
      }
    }
    if matches!(cube.t, Status::On) {
      new_cubes.push(cube);
    }
    new_cubes.extend(to_add);
  }
  let mut on = 0;
  for i in new_cubes {
    let s = if matches!(i.t, Status::On) {1} else {-1};
    on += s * ((i.x.1 - i.x.0) + 1) * ((i.y.1 - i.y.0) + 1) * ((i.z.1 - i.z.0) + 1)
  }
  on
}

fn intersection(i1:&Cube,i2:&Cube) -> Option<Cube> {
  let i = Cube{
    x: (i1.x.0.max(i2.x.0),i1.x.1.min(i2.x.1)),
    y: (i1.y.0.max(i2.y.0),i1.y.1.min(i2.y.1)),
    z: (i1.z.0.max(i2.z.0),i1.z.1.min(i2.z.1)),
    t: if matches!(i2.t, Status::On) {Status::Off} else {Status::On}
  };
  if i.x.0 <= i.x.1 && i.y.0 <= i.y.1 && i.z.0 <= i.z.1 {
    Some(i)
  } else {
    None
  }
}

fn create_pairs(x:(i64,i64),y:(i64,i64),z:(i64,i64)) -> Vec<(i64,i64,i64)> {
  let mut pairs = vec![];
  let min_x = if x.0 < -50 {-50} else {x.0};
  let max_x = if x.1 > 50 {50} else {x.1};
  let min_y = if y.0 < -50 {-50} else {y.0};
  let max_y = if y.1 > 50 {50} else {y.1};
  let min_z = if z.0 < -50 {-50} else {z.0};
  let max_z = if z.1 > 50 {50} else {z.1};

  for i in min_x..=max_x {
    for j in min_y..=max_y {
      for k in min_z..=max_z {
        pairs.push((i,j,k))
      }
    }
  }

  pairs
}

fn parse(input:&[String]) -> Vec<Cube> {
  let mut res = vec![];
  for l in input {
    let split:Vec<&str> = l.split(' ').collect();
    let t = if split[0] == "on" {Status::On} else {Status::Off};
    let split:Vec<&str> = split[1].split(',').collect();
    let mut x = split[0].split('=').nth(1).unwrap().split("..");
    let mut y = split[1].split('=').nth(1).unwrap().split("..");
    let mut z = split[2].split('=').nth(1).unwrap().split("..");
    let x_s = x.next().unwrap().parse().unwrap();
    let x_e = x.next().unwrap().parse().unwrap();
    let y_s = y.next().unwrap().parse().unwrap();
    let y_e = y.next().unwrap().parse().unwrap();
    let z_s = z.next().unwrap().parse().unwrap();
    let z_e = z.next().unwrap().parse().unwrap();
    res.push(Cube{
      t,
      x:(x_s,x_e),
      y:(y_s,y_e),
      z:(z_s,z_e)
    })
  }
  res
}

#[derive(Debug,Clone)]
struct Cube {
  t: Status,
  x: (i64,i64),
  y: (i64,i64),
  z: (i64,i64)
}

#[derive(Debug,Clone)]
enum Status {
  On,
  Off
}

#[cfg(test)]
mod tests {
  use super::{parse,count_on,count_on_all};

  #[test]
  fn test_1() {
    let input:Vec<String> = INPUT1.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let cubes = parse(&input);
    assert_eq!(39,count_on(cubes)); 
  }
  #[test]
  fn test_2() {
    let input:Vec<String> = INPUT2.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let cubes = parse(&input);
    assert_eq!(590784,count_on(cubes)); 
  }
  #[test]
  fn test_3() {
    let input:Vec<String> = INPUT3.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let cubes = parse(&input);
    assert_eq!(2758514936282235,count_on_all(cubes)); 
  }

  static INPUT1: &str = "on x=10..12,y=10..12,z=10..12
  on x=11..13,y=11..13,z=11..13
  off x=9..11,y=9..11,z=9..11
  on x=10..10,y=10..10,z=10..10";
  static INPUT2: &str = "on x=-20..26,y=-36..17,z=-47..7
  on x=-20..33,y=-21..23,z=-26..28
  on x=-22..28,y=-29..23,z=-38..16
  on x=-46..7,y=-6..46,z=-50..-1
  on x=-49..1,y=-3..46,z=-24..28
  on x=2..47,y=-22..22,z=-23..27
  on x=-27..23,y=-28..26,z=-21..29
  on x=-39..5,y=-6..47,z=-3..44
  on x=-30..21,y=-8..43,z=-13..34
  on x=-22..26,y=-27..20,z=-29..19
  off x=-48..-32,y=26..41,z=-47..-37
  on x=-12..35,y=6..50,z=-50..-2
  off x=-48..-32,y=-32..-16,z=-15..-5
  on x=-18..26,y=-33..15,z=-7..46
  off x=-40..-22,y=-38..-28,z=23..41
  on x=-16..35,y=-41..10,z=-47..6
  off x=-32..-23,y=11..30,z=-14..3
  on x=-49..-5,y=-3..45,z=-29..18
  off x=18..30,y=-20..-8,z=-3..13
  on x=-41..9,y=-7..43,z=-33..15
  on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
  on x=967..23432,y=45373..81175,z=27513..53682";
  static INPUT3: &str = "on x=-5..47,y=-31..22,z=-19..33
  on x=-44..5,y=-27..21,z=-14..35
  on x=-49..-1,y=-11..42,z=-10..38
  on x=-20..34,y=-40..6,z=-44..1
  off x=26..39,y=40..50,z=-2..11
  on x=-41..5,y=-41..6,z=-36..8
  off x=-43..-33,y=-45..-28,z=7..25
  on x=-33..15,y=-32..19,z=-34..11
  off x=35..47,y=-46..-34,z=-11..5
  on x=-14..36,y=-6..44,z=-16..29
  on x=-57795..-6158,y=29564..72030,z=20435..90618
  on x=36731..105352,y=-21140..28532,z=16094..90401
  on x=30999..107136,y=-53464..15513,z=8553..71215
  on x=13528..83982,y=-99403..-27377,z=-24141..23996
  on x=-72682..-12347,y=18159..111354,z=7391..80950
  on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
  on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
  on x=-52752..22273,y=-49450..9096,z=54442..119054
  on x=-29982..40483,y=-108474..-28371,z=-24328..38471
  on x=-4958..62750,y=40422..118853,z=-7672..65583
  on x=55694..108686,y=-43367..46958,z=-26781..48729
  on x=-98497..-18186,y=-63569..3412,z=1232..88485
  on x=-726..56291,y=-62629..13224,z=18033..85226
  on x=-110886..-34664,y=-81338..-8658,z=8914..63723
  on x=-55829..24974,y=-16897..54165,z=-121762..-28058
  on x=-65152..-11147,y=22489..91432,z=-58782..1780
  on x=-120100..-32970,y=-46592..27473,z=-11695..61039
  on x=-18631..37533,y=-124565..-50804,z=-35667..28308
  on x=-57817..18248,y=49321..117703,z=5745..55881
  on x=14781..98692,y=-1341..70827,z=15753..70151
  on x=-34419..55919,y=-19626..40991,z=39015..114138
  on x=-60785..11593,y=-56135..2999,z=-95368..-26915
  on x=-32178..58085,y=17647..101866,z=-91405..-8878
  on x=-53655..12091,y=50097..105568,z=-75335..-4862
  on x=-111166..-40997,y=-71714..2688,z=5609..50954
  on x=-16602..70118,y=-98693..-44401,z=5197..76897
  on x=16383..101554,y=4615..83635,z=-44907..18747
  off x=-95822..-15171,y=-19987..48940,z=10804..104439
  on x=-89813..-14614,y=16069..88491,z=-3297..45228
  on x=41075..99376,y=-20427..49978,z=-52012..13762
  on x=-21330..50085,y=-17944..62733,z=-112280..-30197
  on x=-16478..35915,y=36008..118594,z=-7885..47086
  off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
  off x=2032..69770,y=-71013..4824,z=7471..94418
  on x=43670..120875,y=-42068..12382,z=-24787..38892
  off x=37514..111226,y=-45862..25743,z=-16714..54663
  off x=25699..97951,y=-30668..59918,z=-15349..69697
  off x=-44271..17935,y=-9516..60759,z=49131..112598
  on x=-61695..-5813,y=40978..94975,z=8655..80240
  off x=-101086..-9439,y=-7088..67543,z=33935..83858
  off x=18020..114017,y=-48931..32606,z=21474..89843
  off x=-77139..10506,y=-89994..-18797,z=-80..59318
  off x=8476..79288,y=-75520..11602,z=-96624..-24783
  on x=-47488..-1262,y=24338..100707,z=16292..72967
  off x=-84341..13987,y=2429..92914,z=-90671..-1318
  off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
  off x=-27365..46395,y=31009..98017,z=15428..76570
  off x=-70369..-16548,y=22648..78696,z=-1892..86821
  on x=-53470..21291,y=-120233..-33476,z=-44150..38147
  off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
}