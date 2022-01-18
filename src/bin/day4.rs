use std::fs;
use std::collections::HashSet;

fn main() {
    let filename = "./data/day4";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let (seq, content) = contents.split_once("\n").unwrap();
    let lines: Vec<HashSet<u32>> = content.replace("\n\n","\n")
	.strip_prefix("\n")
	.unwrap()
    	.lines()
    	.map(|x| x.trim()
	     .replace("  "," ")
	     .split(" ")
	     .map(|y| y.parse::<u32>().unwrap())
	     .collect::<HashSet<u32>>())
	.collect();
    let seq: Vec<u32> = seq.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    println!("part 1: {}", part_1(&lines, &seq));
    println!("part 2: {}", part_2(&lines));  
}


fn part_1(input:  &Vec<HashSet<u32>>, seq: &Vec<u32>) -> u32 {
    let mut hash: HashSet<u32> = HashSet::new();
    let mut found: usize = 0;
    let mut last = 0;
    for i in seq {
	hash.insert(*i);
	let res = input.iter().enumerate().filter(|(n, x)| hash.intersection(x).count() == 5);
	let res = res.collect::<Vec<(usize, &HashSet<u32>)>>();
	if res.len() == 1 {
	    found = res[0].0;
	    last = *i;
	    break;
	}
    }
    let winner: HashSet<u32> = input[(found as f32/5.).floor() as usize * 5..(found as f32/5.).floor() as usize * 5 + 5].iter().fold(HashSet::new(), |mut acc, x| {
	acc.extend(x);
	acc
    });
    winner.difference(&hash).into_iter().fold(0, |acc, x| acc + x) * last
}

fn part_2(input:  &Vec<HashSet<u32>>) -> i32 {
    0
}
