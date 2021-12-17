pub fn part1(input:&[String]) -> usize {
  let target_area = parse(input);
  target_area.max_height()
}

pub fn part2(input:&[String]) -> usize {
  let target_area = parse(input);
  target_area.count_trajectories()
}

struct Coord {
  x:isize,
  y:isize
}

struct TargetArea {
  top_left:Coord,
  bottom_right:Coord
}
impl TargetArea {
  fn max_height(&self) -> usize {
    let min_y = self.min_y();
    (min_y * (min_y - 1) / 2) as usize
  }

  fn min_y(&self) -> isize {
    std::cmp::min(self.bottom_right.y,self.top_left.y).abs()
  }

  fn count_trajectories(&self) -> usize {
    let mut count = 0;
    let min_y = self.min_y();
    let max_x = std::cmp::max(self.bottom_right.x,self.top_left.x);
    for x in 0..=max_x {
      for y in -min_y..=min_y {
        let traj = Coord{x,y};
        if self.check_trajectory(traj) {
          count+=1;
        }
      }
    }
    count
  }

  fn check_trajectory(&self, trajectory:Coord) -> bool {
    let mut i = 400;

    let mut x_increase = trajectory.x;
    let mut y_increase = trajectory.y;

    let min_y = std::cmp::min(self.bottom_right.y,self.top_left.y);

    let mut curr_pos = Coord{x:0,y:0};
    while !self.hit(&curr_pos) {
      curr_pos = Coord{
        x: (curr_pos.x + x_increase),
        y: (curr_pos.y + y_increase)
      };
      // update x_increase
      if x_increase>0 {
        x_increase -=1;
      }
      if x_increase<0 {
        x_increase +=1;
      }
      // update y_increase
      y_increase -= 1;

      i-=1;
      // if y is too low, it'll never hit or give up bruteforcing
      if i==0 || curr_pos.y<min_y {
        return false;
      }
    }
    true
  }

  fn hit(&self, coord:&Coord) -> bool {
    (coord.x >= self.top_left.x && coord.x <= self.bottom_right.x) && 
    (coord.y <= self.top_left.y && coord.y >= self.bottom_right.y)
  }
}

fn parse(input:&[String]) -> TargetArea {
  let mut split = input[0].split("target area: x=");
  split = split.nth(1).unwrap().split(", y=");
  let mut xs = split.next().unwrap().split("..");
  let mut ys = split.next().unwrap().split("..");
  let x1 = xs.next().unwrap().parse().unwrap();
  let x2 = xs.next().unwrap().parse().unwrap();
  let y1 = ys.next().unwrap().parse().unwrap();
  let y2 = ys.next().unwrap().parse().unwrap();

  TargetArea {
    top_left: Coord {x:x1,y:y2},
    bottom_right: Coord {x:x2,y:y1},
  }
}


#[cfg(test)]
mod tests {
  use super::{parse};
  static INPUT: &str = "target area: x=20..30, y=-10..-5";

  #[test]
  fn test_target_area_max() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let target_area = parse(&input);
    assert_eq!(10,target_area.min_y());
  }

  #[test]
  fn test_target_area_trajectories() {
    let input:Vec<String> = INPUT.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let target_area = parse(&input);
    assert_eq!(112,target_area.count_trajectories());
  }
}