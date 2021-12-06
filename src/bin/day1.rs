use std::fs;

fn main() {
    let filename = "./data/day1";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut j = 0;
    let mut k = 0;
    let mut prev: Vec<i32> = vec![];
    let mut prev_sum: Vec<i32> = vec![];
    for s in contents.split("\n") {
	let i = s.parse::<i32>().unwrap();
	if prev.len() >= 1 && i > *prev.last().unwrap() {
	    j += 1;
	}
	if prev.len() > 1{
	    let sum = i + prev[prev.len()-1] + prev[prev.len()-2];
	    prev_sum.push(sum);
	}
	if prev_sum.len() > 1 && *prev_sum.last().unwrap() > prev_sum[prev_sum.len()-2] {
	    k += 1;
	}
	prev.push(i)
    }
    println!("part I: {}", j);
    println!("part II: {}", k);
}
