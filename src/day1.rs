#[path = "./advent.rs"]
mod advent;

pub fn part1() {
  let values = advent::read_as_i32_vec("day1.txt");

  let increase_count: i32 = values
    .iter()
    .enumerate()
    .map(|(i, v)| -> i32 {
      if i == 0 { return 0 }
      let prev = &values[i - 1];

      (prev < v) as i32
    })
    .sum();

  println!("{}", increase_count);
}

pub fn part2() {
  let values = advent::read_as_i32_vec("day1.txt");
  let get_sum = |i: usize| values[i] + values[i+1] + values[i+2];
  let range = 1..values.len() - 2;
  let result: i32 = range
    .map(|i| (get_sum(i-1) < get_sum(i)) as i32)
    .sum();

  println!("{}", result);
}