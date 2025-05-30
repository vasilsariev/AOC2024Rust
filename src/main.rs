mod day01;
mod day02;
mod day03;

fn main() {
    let day_number = std::env::args()
        .nth(1)
        .expect("Please pass day number, e.g. `cargo run -- 01`");

    let (part1, part2) = match day_number.as_str() {
        "01" => day01::solve(),
        "02" => day02::solve(),
        "03" => day03::solve(),
        _ => panic!("Day not implemented!"),
    };

    println!(
        "Day {} solutions are: 1 -> {}, 2 -> {}",
        day_number, part1, part2
    );
}
