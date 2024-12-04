use std::io;
use std::io::Write;

mod day1;
mod day2;
mod day3;

fn main() {
    print!("Day "); io::stdout().flush().expect("flush failed");

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("io error");

    input_str.pop();
    match input_str.parse::<i32>().expect("not a valid number") {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        _ => println!("day not implemented / doesn't exist")
    }
}
