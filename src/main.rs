mod day1;
mod day2;
mod day3;

fn test() {

}

fn invalid_function(day: i32, part: i32) {
    println!("Day {} Pt. {} does not exist!", day, part)
}

fn main() {
    test();
    let day = 3;
    let part = 1;

    match (day, part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        (_, _) => invalid_function(day, part),
    };
}
