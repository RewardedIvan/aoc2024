use std::io;

pub fn day2() {
    let mut reports: Vec<Vec<i32>> = Vec::new();

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

        let mut levels = Vec::new();
        for e in split {
            levels.push(e.parse::<i32>().expect("not a valid number"));
        }

        reports.push(levels);
    }

    let mut safe_reports = 0;
    for r in reports.iter() {
        let mut is_unsafe = false;
        // increase = a - b < 0
        let increase = r[0] - r[1] < 0;

        for i in 1..r.len() {
            let a = r[i - 1];
            let b = r[i];
            let diff = a - b;

            //println!("diff between {a} and {b} is {diff}");
            if diff.abs() > 3 || diff == 0 || increase != (diff < 0) {
                is_unsafe = true;
                break;
            }
        }

        //println!("report deemed unsafe: {is_unsafe}");
        if !is_unsafe {
            safe_reports += 1;
        }
    }

    println!("safe reports: {}, unsafe reports: {}", safe_reports, reports.len() - safe_reports);
}
