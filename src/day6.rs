use std::io;
use std::ops;
use std::collections::HashSet;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Pos {
    x: i32, y: i32
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

#[derive(Debug)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn get_forward(&self) -> Pos {
        match *self {
            Self::Up => Pos { x: 0, y: -1 },
            Self::Down => Pos { x: 0, y: 1 },
            Self::Right => Pos { x: 1, y: 0 },
            Self::Left => Pos { x: -1, y: 0 },
        }
    }

    fn next_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

pub fn day6() {
    println!("Enter map, to exit enter END");
    let mut inp = String::new();
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

    let map_size = Pos { x: lines[0].len() as i32, y: lines.len() as i32 };
    let mut dir = Direction::Up;
    let mut guard_pos = lines.iter().enumerate().find_map(|(y, l)| {
        let x = l.iter().position(|c| *c == '^');
        match x {
            Some(x) => Some(Pos {
                x: x as i32,
                y: y as i32
            }),
            None => None
        }
    }).expect("didn't find guard, use '^' in your input");
    
    //println!("guard: {:?}", guard_pos);
    //println!("mapsize: {:?}", map_size);

    let mut visits: HashSet<Pos> = HashSet::new();
    loop {
        let forward = dir.get_forward();
        let moved_pos = guard_pos + forward;

        // cool visualization
        /*if visits.len() >= 5000 {
            print!("{esc}c", esc = 27 as char);
            for (y, l) in lines.iter().enumerate() {
                for (x, c) in l.iter().enumerate() {
                    if x as i32 == moved_pos.x && y as i32 == moved_pos.y {
                        print!("!");
                    } else if x as i32 == guard_pos.x && y as i32 == guard_pos.y {
                        print!("&");
                    // this case tanks preformance
                    } else if visits.contains(&Pos { x: x as i32, y: y as i32 }) {
                        print!(",");
                    } else {
                        print!("{c}");
                    }
                }
                print!("\n");
            }
            print!("{}\n", visits.len());
            //let _ = io::stdin().lines().next().unwrap().expect("A");
            //std::thread::sleep(std::time::Duration::from_millis(10));
        }*/

        visits.insert(guard_pos);
        if  moved_pos.x >= map_size.x || moved_pos.y >= map_size.y ||
            moved_pos.x == -1 || moved_pos.y == -1 {
            break;
        }

        if lines[moved_pos.y as usize][moved_pos.x as usize] == '#' {
            dir = dir.next_clockwise();
            continue;
        }

        guard_pos = moved_pos;
    }

    println!("unique visits: {}", visits.len());
}
