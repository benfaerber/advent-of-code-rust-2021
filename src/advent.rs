use std::fs;

pub fn read_as_string(filename: &str) -> String {
  let raw_file = fs::read_to_string(format!("./inputs/{}", filename))
    .expect("File cannot be read!");
  raw_file
}

pub fn read_as_lines(filename: &str) -> Vec<String> {
  let raw_data = read_as_string(filename);
  let lines: Vec<String> = raw_data
  .trim()
  .split("\n")
  .map(|v| v.to_string())
  .collect();

  lines
}

pub fn read_as_i32_vec(filename: &str) -> Vec<i32> {
  let raw_data = read_as_string(filename);
  let lines = raw_data.split("\n").collect::<Vec<&str>>();
  let values: Vec<i32> = lines
    .iter()
    .map(|v| v.trim().parse::<i32>().unwrap())
    .collect();

  values
}
