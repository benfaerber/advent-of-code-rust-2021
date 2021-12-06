mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn invalid_function(day: i32, part: i32) {
    println!("Day {} Pt. {} does not exist!", day, part)
}

fn main() {
    // Current Day
    let day = 4;
    let part = 1;

    match (day, part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        (4, 1) => day4::part1(),
        (4, 2) => day4::part2(),
        (5, 1) => day5::part1(),
        (5, 2) => day5::part2(),
        (6, 1) => day6::part1(),
        (6, 2) => day6::part2(),
        (7, 1) => day7::part1(),
        (7, 2) => day7::part2(),
        (8, 1) => day8::part1(),
        (8, 2) => day8::part2(),
        (9, 1) => day9::part1(),
        (9, 2) => day9::part2(),
        (_, _) => invalid_function(day, part),
    };
}
