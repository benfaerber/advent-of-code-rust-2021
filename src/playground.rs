fn plus(required_num: i32, optional_num: Option<i32>) -> i32 {
  required_num + optional_num.unwrap_or(0)
}

fn playground_test() {

  let sum_1 = plus(10, Some(23));
  let sum_2 = plus(10, None);
  println!("Sum 1: {}, Sum 2: {}", sum_1, sum_2);

}

pub fn run() {
  println!("Begin Playground");
  playground_test();
  println!("End Playground");
}