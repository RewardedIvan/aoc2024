use std::io;
use std::collections::HashMap;

pub fn day1() {
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

    let mut sum = 0;
    let mut dups = HashMap::<i32, i32>::new();
    for i in 0..cols[0].len() {
        let a = cols[0][i];
        let b = cols[1][i];
        let distance = a.abs_diff(b);

        match dups.get_mut(&b) {
            Some(count) => {
                *count += 1;
            },
            None => {
                dups.insert(b, 1);
            }
        }

        //println!("distance between {} and {} = {}", a, b, distance);

        sum += distance;
    }

    println!("sum of distances: {}", sum);

    let mut similarity_score = 0;
    for e in cols[0].iter() {
        similarity_score += e * dups.get(&e).unwrap_or(&0);
    }
    println!("similarity score: {}", similarity_score);
}
