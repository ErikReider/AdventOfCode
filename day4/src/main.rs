// Every section has a unique ID number,
// and each Elf is assigned a range of section IDs.
//
// Input: Big list of the section assignments for each pair

use std::{collections::HashSet, fs, process};

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};
	let lines = input.trim().split("\n");

	let mut puzzle_1_duplicates: u32 = 0;
	let mut puzzle_2_duplicates: u32 = 0;
	for line in lines {
		let sections: Vec<&str> = line.split(",").collect();
		let first = get_assigned_areas(sections[0]);
		let second = get_assigned_areas(sections[1]);

		let set: HashSet<u8> =
			HashSet::from_iter([first.to_owned(), second.to_owned()].concat());

		// Puzzle 1
		if first.len().max(second.len()) == set.len() {
			puzzle_1_duplicates += 1;
		}

		// Puzzle 2
		if first.len() + second.len() != set.len() {
			puzzle_2_duplicates += 1;
		}
	}
	println!("Puzzle 1 duplicates: {}", puzzle_1_duplicates);
	println!("Puzzle 2 duplicates: {}", puzzle_2_duplicates);
}

fn get_assigned_areas(area: &str) -> Vec<u8> {
	let split: Vec<&str> = area.split("-").collect();
	let first: u8 = str::parse(split[0]).unwrap();
	let last: u8 = str::parse(split[1]).unwrap();
	return (first..last + 1).collect();
}
