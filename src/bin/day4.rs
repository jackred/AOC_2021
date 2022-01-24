use std::fs;
use std::collections::HashSet;

fn main() {
    let filename = "./data/day4";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let (seq, content) = contents.split_once("\n").unwrap();
    let content: Vec<Vec<u32>> = content.replace("\n\n","\n")
    	.strip_prefix("\n")
    	.unwrap()
    	.lines()
    	.map(|x| x.trim()
    	     .replace("  "," ")
    	     .split(" ")
    	     .map(|y| y.parse::<u32>().unwrap())
    	     .collect::<Vec<u32>>())
    	.collect();
    let lines = content.iter().map(|x| HashSet::from_iter(x.iter().cloned())).collect();
    let columns = (0..content.len()).step_by(5).fold(vec![], |mut acc: Vec<HashSet<u32>>, n| {
	content[n..n+5].into_iter().enumerate().for_each(|(m, x)| {
	    for i in 0..5 {
		if m == 0 {
		    acc.push(HashSet::from([x[i]]));
		} else {
		    acc[n+i].insert(x[i]);
		}
	    }
	});
	acc
    });
    let seq: Vec<u32> = seq.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    println!("part 1: {}", part_1(&lines, &columns,  &seq));
    println!("part 2: {}", part_2(&lines, &columns,  &seq));
}


fn part_1(lines:  &Vec<HashSet<u32>>, columns: &Vec<HashSet<u32>>, seq: &Vec<u32>) -> u32 {
    let mut hash: HashSet<u32> = HashSet::new();
    let mut found: usize = 0;
    let mut last = 0;
    for i in seq {
	hash.insert(*i);
	let res = lines.iter().enumerate().filter(|(_, x)| hash.intersection(x).count() == 5);
	let res = res.collect::<Vec<(usize, &HashSet<u32>)>>();
	if res.len() == 1 {
	    found = res[0].0;
	    last = *i;
	    break;
	} else {
	    let res = columns.iter().enumerate().filter(|(_, x)| hash.intersection(x).count() == 5);
	    let res = res.collect::<Vec<(usize, &HashSet<u32>)>>();
	    if res.len() == 1 {
		found = res[0].0;
		last = *i;
		break;
	    }
	}
    }
    let winner: HashSet<u32> = lines[(found as f32/5.).floor() as usize * 5..(found as f32/5.).floor() as usize * 5 + 5].iter().fold(HashSet::new(), |mut acc, x| {
	acc.extend(x);
	acc
    });
    winner.difference(&hash).into_iter().fold(0, |acc, x| acc + x) * last
}

fn part_2(lines: &Vec<HashSet<u32>>, columns: &Vec<HashSet<u32>>, seq: &Vec<u32>) -> u32 {
    let mut hash: HashSet<u32> = HashSet::new();
    let mut found: usize = 0;
    'outer: for i in seq.iter().rev() {
	hash.insert(*i);
	for j in (5..lines.len()).step_by(5).rev() {
	    let b1 = lines[j-5..j].iter().all(|x| hash.intersection(x).count() >= 1);
	    let b2 = columns[j-5..j].iter().all(|x| hash.intersection(x).count() >= 1);
	    if b1 && b2 {
		found = j;
		break 'outer;
	    }
	}
    }
    let mut f_lines = vec![HashSet::new(); 5]; 
    f_lines.clone_from_slice(&lines[found-5..found]);
    let mut f_cols = vec![HashSet::new(); 5]; 
    f_cols.clone_from_slice(&columns[found-5..found]);
    let res = seq.iter().find(|x| {
	let b1 = f_lines.iter_mut().map(|y| {
	    (y).remove(x);
	    y.len() == 0
	}).fold(false, |acc, b| acc || b);
	let b2 = f_cols.iter_mut().map(|y| {
	    (y).remove(x);
	    y.len() == 0
	}).fold(false, |acc, b| acc || b);
	b1 || b2
    }).unwrap();
    f_cols.iter().fold(0, |acc, x| x.iter().sum::<u32>() + acc) * res
}
