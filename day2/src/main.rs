// If both players choose the same shape, the round instead ends in a draw.
//
// Input: "The first column is what your opponent is going to play:
//		- A for Rock,
//		- B for Paper
//		- C for Scissors
// Col 2:
// - X for Rock
// - Y for Paper
// - Z for Scissors
//
// - 1 Rock
// - 2 Paper
// - 3 Scissors
//
// Your total score is the sum of your scores for each round.
// The score for a single round is the score for the shape you selected
// plus score for the outcome of the round.
//
// Winning every time would be suspicious, so the responses must have been carefully chosen.

use std::{collections::HashMap, fs, process};

const TABLE_PUZZLE1: [[i32; 3]; 3] = [
	//r p  s
	[3, 0, 6], // r
	[6, 3, 0], // p
	[0, 6, 3], // s
];

const TABLE_PUZZLE2: [[i32; 3]; 3] = [
	//L D  W
	[3, 1, 2], // r
	[1, 2, 3], // p
	[2, 3, 1], // s
];

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};
	let lines = input.trim().split("\n").collect::<Vec<&str>>();

	let conversion = HashMap::from([
		("A", 0),
		("X", 0),
		("B", 1),
		("Y", 1),
		("C", 2),
		("Z", 2),
	]);

	puzzle1(&lines, &conversion);

	puzzle2(&lines, &conversion);
}

// - 0 = lose
// - 3 = draw
// - 6 = win
fn puzzle1(lines: &Vec<&str>, conversion: &HashMap<&str, i32>) {
	let mut points: i32 = 0;
	for line in lines {
		let chars: Vec<&str> = line.trim().split(" ").collect();
		let opponent = conversion[chars[0]];
		let user = conversion[chars[1]];
		let result = TABLE_PUZZLE1[user as usize][opponent as usize];
		points += result + conversion[chars[1]] as i32 + 1;
	}
	println!("Puzzle 1 Points: {}", points);
}

// - X = lose
// - Y = draw
// - Z = win
fn puzzle2(lines: &Vec<&str>, conversion: &HashMap<&str, i32>) {
	let mut points: i32 = 0;
	for line in lines {
		let chars: Vec<&str> = line.trim().split(" ").collect();
		let opponent = conversion[chars[0]];
		let user = conversion[chars[1]];
		let result: i32 = TABLE_PUZZLE2[opponent as usize][user as usize];
		points += result + user as i32 * 3;
	}
	println!("Puzzle 2 Points: {}", points);
}
