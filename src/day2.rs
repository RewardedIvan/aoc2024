use std::io;

pub fn day2() {
    let mut safe_reports = 0;
    let mut reports = 0;

    println!("Enter levels seperated by spaces, to exit enter END");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline

        if line == "END" {
            break;
        }

        let split = line.split(' ').collect::<Vec<&str>>();
        if split.len() < 2 {
            panic!("there should be atleast 2 levels in a report");
        }

        let mut is_unsafe = false;
        let mut increase = true;
        for i in 1..split.len() {
            let a = split[i - 1].parse::<i32>().expect("not a valid number");
            let b = split[i].parse::<i32>().expect("not a valid number");
            let diff = a - b;

            if i == 1 {
                // increase = a - b < 0
                increase = diff < 0;
            }

            //println!("diff between {a} and {b} is {diff}");
            if diff.abs() > 3 || diff == 0 || increase != (diff < 0) {
                is_unsafe = true;
                break;
            }
        }

        //println!("report deemed unsafe: {is_unsafe}");
        reports += 1;

        if !is_unsafe {
            safe_reports += 1;
        }
    }

    println!("safe reports: {}, unsafe reports: {}, total: {}", safe_reports, reports - safe_reports, reports);
}
