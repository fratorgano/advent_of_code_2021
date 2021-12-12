use std::{
  collections::{HashMap, HashSet}
};

pub fn part1(input:&[String]) -> usize {
  let graph = parse(input);
  count_dfs(&graph,"start".to_string(),HashSet::new())
}

pub fn part2(input:&[String]) -> usize {
  let graph = parse(input);
  count_dfs_twice(&graph,"start".to_string(),HashSet::new(),false)
}

struct Graph {
  adj_list: HashMap<String, Vec<String>>,
}
impl Graph {
  fn new() -> Self {
      Self {
          adj_list: HashMap::new(),
      }
  }

  fn add_edge(&mut self, src: &str, dst: &str) {
      self.adj_list
          .entry(src.to_string())
          .or_insert_with(Vec::new)
          .push(dst.to_string());
  }

  fn add(&mut self, b: &str, a: &str) {
      self.add_edge(a, b);
      self.add_edge(b, a);
  }

  fn adj(&self, node: &str) -> impl Iterator<Item = &String> {
      self.adj_list[node].iter()
  }
}

fn parse(input:&[String]) -> Graph {
  let mut graph = Graph::new();
  for line in input {
    let mut split = line.split('-');
    let a = split.next().unwrap();
    let b = split.next().unwrap();
    graph.add(a, b)
  }
  graph
}

fn count_dfs(graph:&Graph,node:String,mut seen:HashSet<String>) -> usize {
  if seen.contains(&node) {
    return 0;
  }
  if node == "end" {
    return 1;
  }
  if node == node.to_lowercase() {
    seen.insert(node.to_string());
  }
  graph.adj(&node).map(|n| count_dfs(graph,n.to_string(),seen.clone())).sum()
}

fn count_dfs_twice(graph:&Graph,node:String,mut seen:HashSet<String>,mut double:bool) -> usize {
  if seen.contains(&node) {
    if double {
      return 0;
    } else {
      double = true;
    }
  }
  if node == "end" {
    return 1;
  }
  if node == node.to_lowercase() {
    seen.insert(node.to_string());
  }
  graph.adj(&node).filter(|n| *n != "start").map(|n| count_dfs_twice(graph,n.to_string(),seen.clone(),double)).sum()
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;
  use super::{parse,count_dfs,count_dfs_twice};
  #[test]
  fn test_part1_1() {
    let string = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    let result = count_dfs(&graph,"start".to_string(),HashSet::new());
    assert_eq!(10,result);
  }
  #[test]
  fn test_part1_2() {
    let string = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    let result = count_dfs(&graph,"start".to_string(),HashSet::new());
    assert_eq!(19,result);
  }
  #[test]
  fn test_part1_3() {
    let string = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    let result = count_dfs(&graph,"start".to_string(),HashSet::new());
    assert_eq!(226,result);
  }
  #[test]
  fn test_part2_1() {
    let string = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    let result = count_dfs_twice(&graph,"start".to_string(),HashSet::new(),false);
    assert_eq!(36,result);
  }
  #[test]
  fn test_part2_2() {
    let string = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    let result = count_dfs_twice(&graph,"start".to_string(),HashSet::new(),false);
    assert_eq!(103,result);
  }
  #[test]
  fn test_part2_3() {
    let string = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
    let input:Vec<String> = string.split('\n').into_iter().map(|s| String::from(s.trim())).collect();
    let graph = parse(&input);
    let result = count_dfs_twice(&graph,"start".to_string(),HashSet::new(),false);
    assert_eq!(3509,result);
  }
}