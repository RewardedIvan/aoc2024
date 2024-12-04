use std::io;
use regex::Regex;

pub fn day3() {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex failed to compile");
    let mut sum: u64 = 0;

    println!("Enter corrupted memory, to exit enter END");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline

        if line == "END" {
            break;
        }

        for m in regex.captures_iter(&line) {
            let a: u64 = m.get(1).unwrap().as_str().parse().unwrap();
            let b: u64 = m.get(2).unwrap().as_str().parse().unwrap();
            sum += a * b;
        }
    }

    println!("final sum: {}", sum);
}
