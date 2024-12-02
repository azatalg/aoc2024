mod read;
mod day1;
use read::read;
mod day2;
fn main() {
    // Day1
    println!("Day 1");
    println!("{}", day1::solve1(&read("1")));
    println!("{}", day1::solve2(&read("1")));
    // Day2
    println!("Day 2");
    println!("{}", day2::solve1(&read("2")));
    println!("{}", day2::solve2(&read("2")));
}
