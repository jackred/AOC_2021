use std::fs;

const WIDTH: usize = 12;

fn main() {
    let filename = "./data/day3";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<i32> = contents.lines().map(|s| i32::from_str_radix(s, 2).unwrap()).collect();
    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));    
}


// idk, I'm trying things
fn part_1(input:  &Vec<i32>) -> i32 {
    let a: i32 = input
	.iter()
	.fold(vec![0; WIDTH], |acc, e| {
	    acc
		.iter()
		.enumerate()
		.map(|(n, x)| (e >> WIDTH - 1 - n & 1) + x)
		.collect()
	})
	.into_iter()
	.enumerate()
	.map(|(n, x)| ((x as usize > input.len() / 2) as i32) * (2i32.pow(WIDTH as u32 - 1 - n as u32))).sum::<i32>();
    let b = 2i32.pow(WIDTH as u32) - 1 - a;
    a * b
}

// better than before tbh
fn part_2(input:  &Vec<i32>) -> i32 {
    let a = (1..WIDTH+1).rev().fold((0,0), |mut acco2, i| {
	let res = input.iter().fold((vec![0,0], vec![0,0], 0, 0), |mut acc, e| {
	    if e >> i == acco2.0 >> i {
		acc.0[(e >> i-1 & 1) as usize] += 1;
		acc.2 = *e;
		   
	    }
	    if e >> i == acco2.1 >> i {
		acc.1[(e >> i-1 & 1) as usize] += 1;
		acc.3 = *e;
	    }
	    acc
	});
	let s1 = res.0.iter().sum::<i32>();
	let s2 = res.1.iter().sum::<i32>();
	acco2.0 = match s1 {
	    1 => res.2,
	    0 | _ if res.0[0] > res.0[1] => acco2.0,
	    _  => acco2.0 + (1 << i-1),
	};
	acco2.1 = match s2 {
	    1 => res.3,
	    0 | _ if res.1[0] <= res.1[1] => acco2.1,
	    _  => acco2.1 + (1 << i-1),
	};
	acco2
    });
    a.0*a.1
}
