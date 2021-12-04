#[path = "./advent.rs"]
mod advent;

pub fn part1() {
  let lines = advent::read_as_lines("day3.txt");
  let all_bits: Vec<Vec<i32>> = lines
  .iter()
  .map(|line| {
    let letters: Vec<char> = line.chars().collect();
    let bits: Vec<i32> = letters.iter().map(|v| if v == &'1' {1} else {0}).collect();
    bits
  })
  .collect();

  let bit_list = vec![0; 12];
  let bit_one_count = all_bits
  .iter()
  .fold(bit_list, |mut acc, bits| {
    for i in 0..acc.len() {
      acc[i] += bits[i];
    }
    acc
  });

  let half_bits = (all_bits.len() as i32) / 2;

  let get_common = |is_most: bool| -> Vec<String> {
    bit_one_count
    .iter()
    .map(|ones| (if is_most {ones > (&half_bits)} else {ones < (&half_bits)} as i32).to_string())
    .collect()
  };

  let binary_to_decimal = |binary: String| isize::from_str_radix(binary.as_str(), 2).unwrap();

  let binary_gamma = get_common(true).join("");
  let binary_epsilon = get_common(false).join("");

  let dec_gamma = binary_to_decimal(binary_gamma);
  let dec_epsilon = binary_to_decimal(binary_epsilon);
  let multi = dec_gamma * dec_epsilon;
  println!("Power Output: {}", multi);
}

pub fn part2() {

}