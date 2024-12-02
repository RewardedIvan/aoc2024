use std::io;

pub fn two_col_input() -> [Vec<i32>; 2] {
    let mut cols = [
        Vec::<i32>::new(),
        Vec::<i32>::new()
    ];

    println!("Enter two columns using <tab> seperation, to exit enter END");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline
        
        if line == "END" {
            break;
        }

        let split = line.split('\t').collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("there should be 2 columns");
        }

        for i in 0..2 {
            let num = split[i].parse::<i32>().expect("not a valid number");

            // weird sorting that i made up, but it works.. at O(n^2)
            let mut found = false;
            for j in 0..cols[i].len() {
                if cols[i][j] > num {
                    cols[i].insert(j - 0, num);
                    found = true;
                    break;
                }
            }

            if !found {
                cols[i].push(num);
            }
        }
    }

    return cols;
}

pub fn day1() {
    let cols = two_col_input();

    let mut sum = 0;
    for i in 0..cols[0].len() {
        let a = cols[0][i];
        let b = cols[1][i];
        let distance = a.abs_diff(b);

        println!("Distance between {} and {} = {}", a, b, distance);

        sum += distance;
    }

    println!("Sum of distances: {}", sum);
}
