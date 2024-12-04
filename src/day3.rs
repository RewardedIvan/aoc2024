use std::io;
use regex::{Regex,Captures};

pub fn day3() {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex failed to compile");
    let regex_toggles = Regex::new(r"(don't\(\))|(do\(\))").expect("toggle regex failed to compile");
    let mut sum: u64 = 0;
    let mut toggle_sum: u64 = 0;

    let mut inp = String::new();
    println!("Enter corrupted memory, to exit enter END");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline
        inp += &line;

        if line == "END" {
            break;
        }
    }

    // reversed list of every toggle (and their indexes)
    let mut toggles =
        regex_toggles.captures_iter(&inp)
        .collect::<Vec<Captures>>();
    toggles.reverse();

    for m in regex.captures_iter(&inp) {
        let idx = m.get(0).unwrap().start();
        let a: u64 = m.get(1).unwrap().as_str().parse().unwrap();
        let b: u64 = m.get(2).unwrap().as_str().parse().unwrap();

        sum += a * b;

        let mut enabled = true;
        //let mut t = "";
        //let mut t_i = 0;
        //let mut t_l = 0;
        for toggle_m in toggles.as_slice() {
            if toggle_m.get(0).unwrap().end() <= idx {
                enabled = toggle_m.get(1).is_none();
                //t = toggle_m.get(0).unwrap().as_str();
                //t_i = toggle_m.get(0).unwrap().end();
                //t_l = toggle_m.get(0).unwrap().start();
                break;
            }
        }

        if enabled {
            toggle_sum += a * b;
        }

        //println!("{} [{a}*{b}] at {idx} is {enabled}, because of {t} at {t_i}", m.get(0).unwrap().as_str());
        //println!("\x1b[31m{}\x1b[90m{}\x1b[31m{}\x1b[0m", &inp[t_l..t_i], &inp[t_i..idx], &inp[idx..m.get(0).unwrap().end()])
    }

    println!("final sum: {}", sum);
    println!("sum with toggles: {}", toggle_sum);
}
