use std::fs;
use std::cmp::min;

fn main() {
    let filename = "./data/day6";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input: Vec<isize> = contents.split(',').map(|x| x.parse().unwrap()).collect();
    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input.clone()));
}


fn part_1(mut input: Vec<isize>) -> usize {
    for _ in 1..=80 {
	let size = input.len();
	for j in 0..size {
	    match input[j] {
		0 => {
		    input[j] = 6;
		    input.push(8);
		}
		_ => {
		    input[j] -= 1;
		}
	    }
	}
    }
    input.len()
}


fn part_2(input: Vec<isize>) -> usize {
    let mut new: Vec<usize> = input.iter().fold(vec![0;9], |mut acc, x| {
	acc[*x as usize] += 1;
	acc
    });
    (0..256).for_each(|_| {
	let spawn = new[0];
	(0..8).for_each(|y| {
	    new[y] = new[y+1]
	});
	new[8] = spawn;
	new[6] += spawn;
    });
    new.iter().sum::<usize>()
}


