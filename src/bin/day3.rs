use std::fs;
use std::cmp::{min};


fn main() {
    let filename = "./data/day3";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<Vec<i32>> = contents.lines().map(|s| String::from(s).chars().map(|x| if x == '1' {1} else {-1}).collect::<Vec<i32>>()).collect();
    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));    
}


// idk, I'm trying things
fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    let res = input.iter().fold(vec![0; input[0].len()], |mut acc, e| {
	for i in 0..e.len() {
	    acc[i] += e[i];
	}
	acc
    });
    let res = res.iter().fold(String::new(),|mut acc, &e| {
	acc.push(if e > 0 {'1'} else {'0'});
	acc
    });
    let gamma = i32::from_str_radix(res.as_str(), 2).unwrap();
    let epsilon = 2i32.pow(input[0].len() as u32)-1 - gamma;
    gamma * epsilon
}



// yes it is extremly ugly
// trying to figure out how to rust, but after a full day of work, I can barely think and solve the problem
// so doing a nnice thing in rust will come later
// my brain is dead
fn part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut n = input.len() as i32;
    let mut m = input.len() as i32;
    let mut o_r: Vec<i32> = vec![];
    let mut c_r: Vec<i32> = vec![];
    let mut co2: Vec<i32> = vec![];
    let mut o2 : Vec<i32> = vec![];
    while n > 1 || m > 1 {
	let res = input.into_iter().fold(vec![0, 0, -1,-1,0,0, 0], |mut acc, e| {
	    if e[..o_r.len()] == o_r {
		acc[0] += e[o_r.len()];
		acc[2] = acc[4];
		acc[5] += 1;
	    }
	    if e[..c_r.len()] == c_r {
		acc[1] += e[c_r.len()];
		acc[3] = acc[4];
		acc[6] += 1;
	    }
	    acc[4] += 1;
	    acc
	});
	o_r.push(if res[0] >= 0 {1} else {-1});
	c_r.push(if res[1] >= 0 {-1} else {1});
	if n != 1 { 
	    if o_r.len() == input[0].len() {
		n = 1;
		o2 = o_r.clone();
	    } else {
		n = res[5];
		o2 = input[res[2] as usize].clone();
	    }
	}
	if m != 1 {
	    if c_r.len() == input[0].len() {
		m = 1;
		co2 = c_r.clone();
	    } else {
		m = res[6];
		co2 = input[res[3] as usize].clone();
	    }
	}	    
    };
    let o2 = o2.iter().fold(String::new(),|mut acc, &e| {
	acc.push(if e > 0 {'1'} else {'0'});
	acc
    });
    let co2 = co2.iter().fold(String::new(),|mut acc, &e| {
	acc.push(if e > 0 {'1'} else {'0'});
	acc
    });
    let o2 = i32::from_str_radix(o2.as_str(), 2).unwrap();
    let co2 = i32::from_str_radix(co2.as_str(), 2).unwrap();
    o2 * co2
}
