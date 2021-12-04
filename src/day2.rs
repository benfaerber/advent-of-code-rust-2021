#[path = "./advent.rs"]
mod advent;

pub fn part1_old() {
  let raw_data = advent::read_as_string("day2.txt");
  let lines = raw_data.trim().split("\n");

  let mut horizontal: i32 = 0;
  let mut depth: i32 = 0;

  for line in lines {
    let chunks: Vec<&str> = line.split(" ").collect();
    let cmd: &str = chunks[0];
    let value: i32 = chunks[1].trim().parse::<i32>().unwrap();

    if cmd == "forward" {
      horizontal += value;
      continue;
    }

    let scalar: i32 = if cmd == "down" {1} else {-1};
    depth += value * scalar;
  }

  let multi = horizontal * depth;
  println!("Horizontal: {}, Depth: {}", horizontal, depth);
  println!("H * D = {}", multi);
}

pub fn part1() {
  let raw_data = advent::read_as_string("day2.txt");
  let lines = raw_data.trim().split("\n");

  let result = lines
  .map(|line| -> (i32, i32) {
    let chunks: Vec<&str> = line.split(" ").collect();
    let cmd: &str = chunks[0];
    let value: i32 = chunks[1].trim().parse::<i32>().unwrap();

    if cmd == "forward" {
      return (value, 0);
    }

    let scalar = if cmd == "down" {1} else {-1};
    (0, value * scalar)
  })
  .fold((0, 0), |acc, curr| (acc.0 + curr.0, acc.1 + curr.1));

  let (h, d) = result;
  let multi = h * d;
  println!("Horizontal: {}, Depth: {}", h, d);
  println!("H * D = {}", multi);
}

pub fn part2() {
  let raw_data = advent::read_as_string("day2.txt");
  let lines = raw_data.trim().split("\n");

  // Horizontal, Depth, Aim
  let result = lines
  .fold((0, 0, 0), |acc, line| -> (i32, i32, i32) {
    let (old_h, old_d, old_a) = acc;
    let chunks: Vec<&str> = line.split(" ").collect();
    let cmd: &str = chunks[0];
    let value: i32 = chunks[1].trim().parse::<i32>().unwrap();

    if cmd == "forward" {
      return (old_h + value, old_d + (old_a * value), old_a);
    }

    let scalar = if cmd == "down" {1} else {-1};
    (old_h, old_d, old_a + scalar * value)
  });

  let (h, d, a) = result;
  let multi = h * d;
  println!("Horizontal: {}, Depth: {}, Aim: {}", h, d, a);
  println!("H * D = {}", multi);
}