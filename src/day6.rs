#[path = "./advent.rs"]
mod advent;

fn get_laternfish_data() -> Vec<i32> {
  let values: Vec<i32> = advent::read_as_string("day6.txt")
  .trim()
  .split(",")
  .map(|v| v.parse::<i32>().unwrap())
  .collect();

  values
}

fn simulate_day(todays_data: &Vec<i32>) -> Vec<i32> {
  let to_add: &mut Vec<i32> = &mut vec![];

  let baby_value = 8;
  let mother_value = 6;

  let mut tomorrows_data: Vec<i32> = todays_data
  .iter()
  .map(|v| {
    let val = v - 1;
    if val >= 0 {
      val
    } else {
      to_add.push(baby_value);
      mother_value
    }
  })
  .collect();

  tomorrows_data.extend(to_add.iter().copied());
  tomorrows_data
}

pub fn part1() {
  let initial_data: Vec<i32> = get_laternfish_data();
  let days_to_similate = 80;
  let day_range = 0..days_to_similate;

  let final_fish_data = day_range.fold(initial_data, |yesterday, day_index| {
    println!("Day {}", day_index);
    simulate_day(&yesterday)
  });
  let fish_count = final_fish_data.len();

  println!("Amount of lantern fish after {} days: {}", days_to_similate, fish_count);
}

pub fn part2() {

}