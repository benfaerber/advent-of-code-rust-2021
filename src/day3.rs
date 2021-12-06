#[path = "./advent.rs"]
mod advent;

pub fn part1() {
  let lines = advent::read_as_lines("day3.txt");
  let all_bits: Vec<Vec<i32>> = lines
  .iter()
  .map(|line| {
    let letters: Vec<char> = line.chars().collect();
    let bits: Vec<i32> = letters
    .iter()
    .map(|v| if v == &'1' {1} else {0}).collect();

    bits
  })
  .collect();

  let bit_list = vec![0; all_bits[0].len()];
  let bit_one_count = all_bits
  .iter()
  .fold(bit_list, |mut acc, bits| {
    for i in 0..acc.len() {
      acc[i] += bits[i];
    }
    acc
  });

  let half_bits = (all_bits.len() as i32) / 2;

  let get_common = |is_most: bool| -> String {
    let gt = |a: &i32, b: &i32| a > b;
    let lt = |a: &i32, b: &i32| a < b;
    let comp = if is_most {gt} else {lt};

    let str_list: Vec<String> = bit_one_count
    .iter()
    .map(|ones| (comp(ones, &half_bits) as i32).to_string())
    .collect();

    str_list.join("")
  };

  let binary_to_decimal = |binary: String| isize::from_str_radix(binary.as_str(), 2).unwrap();

  let binary_gamma = get_common(true);
  let binary_epsilon = get_common(false);

  let dec_gamma = binary_to_decimal(binary_gamma);
  let dec_epsilon = binary_to_decimal(binary_epsilon);
  let multi = dec_gamma * dec_epsilon;
  println!("Power Output: {}", multi);
}

fn get_bit_list() -> Vec<Vec<i32>> {
  let lines = advent::read_as_lines("day3.txt");
  let all_bits: Vec<Vec<i32>> = lines
  .iter()
  .map(|line| {
    let letters: Vec<char> = line.chars().collect();
    let bits: Vec<i32> = letters
    .iter()
    .map(|v| if v == &'1' {1} else {0}).collect();

    bits
  })
  .collect();

  all_bits
}

pub fn part2() {
  let get_common = |index: usize, bits: &Vec<Vec<i32>>, is_most: bool| -> i32 {
    let one_count = &bits
    .iter()
    .fold(0, |acc, b| acc + ((b[index] == 1) as i32));

    let total = &bits.len();
    let half = (total / 2) as i32;

    //println!("1: {}, total: {}, half: {}", one_count, total, half);
    let gte = |a, b| a >= b;
    let lte = |a, b| a <= b;
    let comp = if is_most {gte} else {lte};
    comp(one_count, &half) as i32
  };

  let remove_uncommon = |index: usize, mut bits: Vec<Vec<i32>>, most_common: i32| {
    let removed_bits: Vec<Vec<i32>> = bits
    .drain(..)
    .filter(|b| b[index] == most_common)
    .collect();

    removed_bits
  };

  let find_bit = |bits: Vec<Vec<i32>>, is_most: bool| -> String {
    let bit_range = 0..bits[0].len();
    let filtered_bit: Vec<Vec<i32>> = bit_range
    .fold(bits, |acc, index| {
      let common = get_common(index, &acc, is_most);
      println!("Is Most: {}, Common: {:?}", is_most, common);
      if acc.len() == 1 {return acc}
      let without_common = remove_uncommon(index, acc, common);
      without_common
    });

    let found = &filtered_bit[0];
    let str_list: Vec<&str> = found
    .iter()
    .map(|&v| if v == 1 {"1"} else {"0"})
    .collect();

    let binary_str = str_list.join("");

    binary_str
  };

  let binary_to_decimal = |binary: String| isize::from_str_radix(binary.as_str(), 2).unwrap();


  let all_bits = get_bit_list();
  let all_bits2 = get_bit_list();

  let most_common_bit = find_bit(all_bits, true);
  let least_common_bit = find_bit(all_bits2, false);

  let most_common_dec = binary_to_decimal(most_common_bit);
  let least_common_dec = binary_to_decimal(least_common_bit);
  let multi = most_common_dec * least_common_dec;

  println!("{} * {} = {}", most_common_dec, least_common_dec, multi);
  // let from_index: usize = 1;
  // let most_common = get_most_common(from_index, &all_bits);
  // let without_uncommon = remove_uncommon(from_index, all_bits, most_common);
  // println!("{:?}", without_uncommon.len());
}