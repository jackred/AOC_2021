use std::fs;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
enum Commands {
    Up(i32),
    Forward(i32),
    Down(i32),
}


impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let mut sp = s.splitn(2, ' ');
	match sp.next().unwrap() {
	    "forward" => Ok(Commands::Forward(sp.next().unwrap().parse().unwrap())),
	    "up" => Ok(Commands::Up(sp.next().unwrap().parse().unwrap())),
	    "down" => Ok(Commands::Down(sp.next().unwrap().parse().unwrap())),
	    _ => Err(()),
	}
    }
}


fn main() {
    let filename = "./data/day2";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<Commands> = contents.lines().map(|x| x.parse().unwrap()).collect();
    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));    
}


fn part_1(input: &Vec<Commands>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for i in input {
	match i {
	    Commands::Up(j) => y -= j,
	    Commands::Forward(j) => x += j,
	    Commands::Down(j) => y += j
	}
    }
    return x * y
}

fn part_2(input: &Vec<Commands>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;
    for i in input {
	match i {
	    Commands::Up(j) => a -= j,
	    Commands::Forward(j) => {
		x += j;
		y += a * j
	    },
	    Commands::Down(j) => a += j
	}
    }
    return x * y
}
