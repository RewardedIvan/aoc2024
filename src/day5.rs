use std::io;

pub fn day5() {
    println!("Enter the page ordering rules, then empty line, then pages to produce in each update, to exit enter END");

    let mut sum = 0;
    let mut sum_fixed = 0;
    let mut rules = Vec::<(i32, i32)>::new();
    let mut ordering_rules = true;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline

        if line == "END" {
            break;
        }

        if line == "" {
            ordering_rules = false;
            continue;
        }

        if ordering_rules {
            let mut numbers = line
                .split('|')
                .map(|s| s.parse::<i32>().expect("invalid number"));

            rules.push((
                numbers.next().expect("specify 2 numbers"),
                numbers.next().expect("specify 2 numbers")
            ));
        } else {
            let numbers = line
                .split(',')
                .map(|s| s.parse::<i32>().expect(&format!("invalid number '{}'", s).to_string()))
                .collect::<Vec<i32>>();

            let mut violation = false;
            let mut fixed = numbers.clone();
            let mut violation_repeat = true;
            while violation_repeat {
                violation_repeat = false;
                for r in &rules {
                    // note a and b are indexes
                    let a_opt = fixed.iter().position(|n| *n == r.0);
                    let b_opt = fixed.iter().position(|n| *n == r.1);

                    if a_opt.is_some() && b_opt.is_some() {
                        let a = a_opt.unwrap();
                        let b = b_opt.unwrap();

                        if a >= b {
                            violation = true;
                            violation_repeat = true;
                            fixed.swap(a, b);
                            //println!("{:?}, {a} {b}", fixed);
                        }
                    }
                }
            }

            //println!("{:?} {} {}", fixed, !violation, fixed[fixed.len() / 2]);
            if !violation {
                sum += numbers[numbers.len() / 2];
            } else {
                sum_fixed += fixed[fixed.len() / 2];
            }
        }
    }
    
    println!("sum of the middle page numbers: {}", sum);
    println!("sum of the middle page numbers on fixed: {}", sum_fixed);
}
