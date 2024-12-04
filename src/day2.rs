use std::io;

pub fn check(levels: &mut Vec<i32>, dont_recurse: bool) -> (bool, bool) {
    let levels_len = levels.len();
    let mut failed = false;
    let mut increase = true;

    //println!("{:?}", levels);

    for i in 1..levels_len {
        let a = levels[i - 1];
        let b = levels[i];
        let diff = a - b;

        if i == 1 {
            // increase = a - b < 0
            increase = diff < 0;
        }

        //println!("diff between {a} and {b}[i: {i}] is {diff} (going up: {}, for report: {})", diff < 0, increase);
        if diff.abs() > 3 || diff == 0 {
            //println!("deemed unsafe because of jump");
            failed = true;
            break;
        } else if increase != (diff < 0) {
            //println!("deemed unsafe, because of increase");
            failed = true;
            break;
        }
    }

    if !dont_recurse && failed {
        // tanks preformance, but honestly i've been working on this for too long
        for i in 0..levels_len {
            let removed = levels.remove(i);
            let res = check(levels, true).0;
            if res {
                return (!failed, true);
            } else {
                levels.insert(i, removed);
            }
        }

        (!failed, false)
    } else {
        (!failed, !failed)
    }
}

pub fn day2() {
    let mut reports = 0;
    let mut safe_reports = 0;
    let mut safe_reports_problem_dampener = 0;

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

        let mut levels = Vec::<i32>::new();
        for e in split {
            levels.push(e.parse::<i32>().expect("not a valid number"));
        }

        let res = check(&mut levels, false);

        reports += 1;
        if res.0 {
            //println!("report is usually safe");
            safe_reports += 1;
            safe_reports_problem_dampener += 1;
        } else if res.1 {
            //println!("report is safe by problem dampener standards");
            safe_reports_problem_dampener += 1;
        } else {
            //println!("report is unsafe.");
        }
        //println!("-------------------------------------------------");
    }

    println!("total reports: {}", reports);
    println!("safe reports: {}, unsafe reports: {}", safe_reports, reports - safe_reports);
    println!("with problem dampener:");
    println!("safe reports: {}, unsafe reports: {}", safe_reports_problem_dampener, reports - safe_reports_problem_dampener);
}
