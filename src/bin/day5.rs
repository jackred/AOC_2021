use std::fs;
use std::str::FromStr;
use std::cmp::{max, min};


#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

impl Coord {
    fn is_on_same_line(one: Coord, two: Coord) -> bool {
	return one.y == two.y
    }

    fn is_on_same_columns(one: Coord, two: Coord) -> bool {
	return one.x == two.x
    }

}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let sp: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
	Ok(Coord{x: sp[0], y: sp[1]})
    }
}

#[derive(Debug)]
pub enum LineState {
    Vertical,
    Horizontal,
    Diagonal
}

#[derive(Debug)]
pub struct Line {
    pub start: Coord,
    pub end: Coord,
    pub state: LineState
}

impl Line {
    fn get_state(one: Coord, two: Coord) -> LineState {
	if Coord::is_on_same_line(one, two){
	    return LineState::Horizontal
	} else if Coord::is_on_same_columns(one, two){
	    return LineState::Vertical
	} else {
	    return LineState::Diagonal
	}
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let sp: Vec<Coord>= s.split(" -> ").map(|x| x.parse().unwrap()).collect();
	Ok(Line{start: sp[0], end: sp[1], state: Line::get_state(sp[0], sp[1])})
    }
}


fn main() {
    let filename = "./data/day5";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let input: Vec<Line> = contents.lines().map(|x| x.parse().unwrap()).collect();
    // don't want to implement max for line or coord
    // cause there's no definition of a max line or max coordinate
    let mut maxi: Coord = input.iter().fold(Coord{x:0, y:0}, |mut acc, x| {
	acc.x = max(acc.x, max(x.start.x, x.end.x));
	acc.y = max(acc.y, max(x.start.y, x.end.y));
	acc
    });
    maxi.x += 1;
    maxi.y += 1;
    println!("part 1: {}", part_1(&input, &maxi));
    println!("part 2: {}", part_2(&input, &maxi));
}


fn part_1(input: &Vec<Line>, maxi: &Coord) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; maxi.x as usize]; maxi.y as usize];
    input.iter().for_each(|e| {
	match e.state {
	    LineState::Horizontal => {
		let a = min(e.start.x, e.end.x);
		let b = max(e.start.x, e.end.x);
		(a..=b).for_each(|i| grid[e.start.y][i]+=1)
	    },
	    LineState::Vertical => {
		let a = min(e.start.y, e.end.y);
		let b = max(e.start.y, e.end.y);
		(a..=b).for_each(|i| grid[i][e.start.x]+=1)
	    },
	    LineState::Diagonal => {}
	}
    });
    grid.iter().fold(0, |acc, x| acc + x.iter().filter(|y| {**y >= 2}).count())    
}


fn part_2(input: &Vec<Line>, maxi: &Coord) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; maxi.x as usize]; maxi.y as usize];
    input.iter().for_each(|e| {
	match e.state {
	    LineState::Horizontal => {
		let a = min(e.start.x, e.end.x);
		let b = max(e.start.x, e.end.x);
		(a..=b).for_each(|i| grid[e.start.y][i]+=1)
	    },
	    LineState::Vertical => {
		let a = min(e.start.y, e.end.y);
		let b = max(e.start.y, e.end.y);
		(a..=b).for_each(|i| grid[i][e.start.x]+=1)
	    },
	    LineState::Diagonal => {
		let diff = (e.start.x as isize - e.end.x as isize).abs();
		let x_inc = if e.start.x > e.end.x {-1} else {1};
		let y_inc = if e.start.y > e.end.y {-1} else {1};
		(0..=diff).for_each(|i: isize| grid[(e.start.y as isize + i*y_inc) as usize][(e.start.x as isize + i*x_inc) as usize]+=1)
	    }
	}
    });
    grid.iter().fold(0, |acc, x| acc + x.iter().filter(|y| {**y >= 2}).count())    
}
