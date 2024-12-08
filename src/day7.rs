use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Add, Mul, Cat
}

trait ReturnOneLiner<T> {
    fn extend_and_return(self, other: Self) -> Self;
}

impl<T> ReturnOneLiner<T> for Vec<T> {
    #[inline]
    fn extend_and_return(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

fn generate_combinations(size: usize, operators: &[Op]) -> Vec<Vec<Op>> {
    (2..size).fold(
        operators.iter().map(|c| operators.iter().map(move |&d| vec!(d.to_owned(), *c))).flatten().collect(),
        |acc, _| acc.into_iter().map(|c| operators.iter().map(move |&d| vec![d.to_owned()].extend_and_return(c.to_owned()))).flatten().collect()
    )
}

fn solve(operators: &Vec<Op>, nums: &Vec<u64>, result: &u64) -> bool {
    //println!("{:?}", generate_combinations(nums.len() - 1, operators));
    for c in generate_combinations(nums.len() - 1, operators) {
        let mut sol = nums[0];

        for (i, o) in c.iter().enumerate() {
            let n = nums[i + 1];
            match *o {
                Op::Add => sol += n,
                Op::Mul => sol *= n,
                Op::Cat => {
                    // I REFUSE to use string cat and parse back
                    let mut digits = 1;
                    let mut power = 10;
                    while n / power >= 1 {
                        power = 10_u64.pow(digits);
                        digits += 1;
                    }
                    //println!("{}", sol * power + n);

                    sol = sol * power + n;
                },
            };
        }

        if *result == sol {
            return true;
        }
    }

    //println!("{:?}", generate_combinations(nums.len() - 1, operators).len());
    false
}

pub fn day7() {
    println!("Enter equations, to exit enter END");

    let mut solutions = 0;
    let mut solutions_cat = 0;
    let mut sum = 0;
    let mut sum_cat = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");
        line.pop(); // remove newline

        if line == "END" {
            break;
        }

        let sep = line.find(':').expect("input should contain ':'");
        let result = line[..sep].parse::<u64>().expect("result should be a number");
        let nums = line[sep+2..].split(' ').map(|n| n.parse::<u64>().expect("equation should contain numbers")).collect::<Vec<u64>>();

        //println!("{line:?}");
        //println!("{result} {nums:?}");

        // legit the first solution that came up to mind, binary!!
        // part2 edit: you're trolling me

        if solve(&vec!(Op::Add, Op::Mul), &nums, &result) {
            solutions += 1;
            sum += result;
        }
        if solve(&vec!(Op::Add, Op::Mul, Op::Cat), &nums, &result) {
            solutions_cat += 1;
            sum_cat += result;
        } else {
            //println!("no solution")
        }
    }

    println!("solutions: {solutions}, sum: {sum}");
    println!("with cat operator:");
    println!("solutions: {solutions_cat}, sum: {sum_cat}");
}
