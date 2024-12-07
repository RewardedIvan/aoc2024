use std::io;

pub fn day7() {
    println!("Enter equations, to exit enter END");

    let mut solutions = 0;
    let mut sum = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline

        if line == "END" {
            break;
        }

        let sep = line.find(':').expect("input should contain ':'");
        let result = line[..sep].parse::<i64>().expect("result should be a number");
        let nums = line[sep+2..].split(' ').map(|n| n.parse::<i64>().expect("equation should contain numbers")).collect::<Vec<i64>>();

        //println!("{result} {nums:?}");

        // legit the first solution that came up to mind, binary!!
        for op in 0..(1 << (nums.len() - 1)) {
            let mut sol: i64 = nums.get(0).expect("atleast put 1 number in the equation").clone().into();
            //println!("sol = {sol}");
            //println!("{op:b}");

            for (i, n) in nums.iter().skip(1).enumerate() {
                let add = (op >> i) & 1;

                if add == 1 {
                    sol += n;
                    //println!("sol += {n}; {sol}");
                } else {
                    sol *= n;
                    //println!("sol *= {n}; {sol}");
                }
            }

            if sol == result {
                solutions += 1;
                sum += result;
                break;
            }
        }
    }

    println!("solutions: {solutions}, sum: {sum}");
}
