use std::env;
use std::fs;
use std::io::BufRead;
use std::time::Instant;

mod day11;

fn main() {
    let input = read_input("11");

    let before1 = Instant::now();
    let res1 = day11::part1(&input);
    println!("Part 1 result: {} in {:?}",res1,before1.elapsed());

    let before2 = Instant::now();
    let res2 = day11::part2(&input);
    println!("Part 2 result: {} in {:?}",res2,before2.elapsed());
}

fn read_input(day:&str) -> Vec<String> {
    let mut dir = env::current_dir().unwrap();
    dir.push("inputs");
    dir.push(format!("{}.txt",day));
    let file = fs::File::open(dir).expect("file not found");
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

