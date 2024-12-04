use std::io;

pub fn day4() {
    let mut inp = String::new();
    println!("Enter the the word search, to exit enter END");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("io error");

        if line == "END\n" {
            break;
        }

        inp += &line;
    }

    let lines: Vec<Vec<char>> =
        inp.lines()
        .map(|l|
            l.chars().collect()
        ).collect();

    let mut appearances = 0;
    let mut appearances_x = 0;

    let lines_len = lines.len();
    for y in 0..lines_len {
        let line_len = lines[y].len();
        for x in 0..line_len {
            let x_right_space = line_len - x - 1;
            let x_left_space = x;
            let y_down_space = lines_len - y - 1;
            let y_up_space = y;

            // horizontal
            if x_right_space >= 3 && (
                (lines[y][x] == 'X' && lines[y][x+1] == 'M' && lines[y][x+2] == 'A' && lines[y][x+3] == 'S') ||
                (lines[y][x] == 'S' && lines[y][x+1] == 'A' && lines[y][x+2] == 'M' && lines[y][x+3] == 'X')
            ) {
                appearances += 1;
            }
            // vertical
            if y_down_space >= 3 && (
                (lines[y][x] == 'X' && lines[y+1][x] == 'M' && lines[y+2][x] == 'A' && lines[y+3][x] == 'S') ||
                (lines[y][x] == 'S' && lines[y+1][x] == 'A' && lines[y+2][x] == 'M' && lines[y+3][x] == 'X')
            ) {
                appearances += 1;
            }

            // right diagonal
            if y_down_space >= 3 && x_right_space >= 3 && (
                (lines[y][x] == 'X' && lines[y+1][x+1] == 'M' && lines[y+2][x+2] == 'A' && lines[y+3][x+3] == 'S') ||
                (lines[y][x] == 'S' && lines[y+1][x+1] == 'A' && lines[y+2][x+2] == 'M' && lines[y+3][x+3] == 'X')
            ) {
                appearances += 1;
                //println!("right diagonal check at {},{}: {}{}{}{}", x, y, lines[y][x], lines[y+1][x+1], lines[y+2][x+2], lines[y+3][x+3]);
            }
            // left diagonal
            if x_left_space >= 3 && y_down_space >= 3 && (
                (lines[y][x] == 'X' && lines[y+1][x-1] == 'M' && lines[y+2][x-2] == 'A' && lines[y+3][x-3] == 'S') ||
                (lines[y][x] == 'S' && lines[y+1][x-1] == 'A' && lines[y+2][x-2] == 'M' && lines[y+3][x-3] == 'X')
            ) {
                appearances += 1;
                //println!("left diagonal check at {},{}: {}{}{}{}", x, y, lines[y][x], lines[y+1][x-1], lines[y+2][x-2], lines[y+3][x-3]);
            }

            if x_left_space >= 1 && y_down_space >= 1 && x_right_space >= 1 && y_up_space >= 1 && lines[y][x] == 'A' && (
                // M..    S..
                // .A. or .A.
                // ..S    ..M
                ((lines[y+1][x-1] == 'M' && lines[y-1][x+1] == 'S') || (lines[y+1][x-1] == 'S' && lines[y-1][x+1] == 'M')) &&
                // ..M    ..S
                // .A. or .A.
                // S..    M..
                ((lines[y+1][x+1] == 'M' && lines[y-1][x-1] == 'S') || (lines[y+1][x+1] == 'S' && lines[y-1][x-1] == 'M'))
            ) {
                appearances_x += 1;
            }
        }
    }

    println!("appearances: {}", appearances);
    println!("MAS as an X shape: {}", appearances_x);
}
