use std::fmt;

// I cheated on day 5 because I got stuck, I would like to revisit it later.

#[path = "./advent.rs"]
mod advent;

struct Point {
  x: i32,
  y: i32
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "X: {}, Y: {}", self.x, self.y)
  }
}

struct Line {
  start: Point,
  end: Point
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}) to ({})", self.start, self.end)
  }
}


pub fn part1() {
  let raw_lines = advent::read_as_lines("day5.txt");
  let lines: Vec<Line> = raw_lines
  .iter()
  .map(|raw_line| {
    let raw: Vec<&str> = raw_line.split(" -> ").collect();
    let to_int = |item: &str| item.trim().parse::<i32>().unwrap();
    let to_point = |v: &str| {
      let parts: Vec<&str> = v.split(",").collect();
      Point {x: to_int(parts[0]), y: to_int(parts[0])}
    };

    Line {
      start: to_point(raw[0]),
      end: to_point(raw[1])
    }
  })
  .collect();

  let straight_lines: Vec<&Line> = lines
  .iter()
  .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
  .collect();

  let covered_points: Vec<Vec<Point>> = straight_lines
  .iter()
  .map(|&line| {
    let is_hori = line.start.y == line.end.y;

    let range = if is_hori {
      line.start.x..line.end.x
    } else {
      line.start.y..line.end.y
    };

    let span: Vec<Point> = range
    .map(|i| {
      if is_hori {
        Point {x: i, y: line.start.y}
      } else {
        Point {x: line.start.x, y: i}
      }
    })
    .collect();

    span
  })
  .collect();

  let mut all_points: Vec<Point> = vec![];
  for mut l in covered_points {
    println!("{}", l.len());
    all_points.append(&mut l);
  }

  println!("{}", all_points.len());
}

pub fn part2() {

}