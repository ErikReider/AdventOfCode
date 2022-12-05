use std::{char, collections::HashMap, fs, process};

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};
	let lines = input.split("\n").collect::<Vec<&str>>();

	// Get the initial crate structure lines
	let mut move_start_index: usize = 0;
	let mut crates_lines: Vec<&str> = vec![];
	for (i, line) in lines.iter().enumerate() {
		if line.starts_with(" 1") {
			move_start_index = i + 1;
			break;
		}
		crates_lines.push(*line);
	}
	// Get the crate structure
	let mut crate_map: HashMap<usize, Vec<char>> = HashMap::new();
	for line in crates_lines.iter().rev() {
		// Insert each crate into its corresponding section
		for (index, char) in line.chars().enumerate() {
			if char.is_alphabetic() {
				crate_map
					.entry((index + 3) / 4)
					.or_insert_with(Vec::new)
					.push(char);
			}
		}
	}

	puzzle(&lines, crate_map.clone(), &move_start_index, true);

	puzzle(&lines, crate_map.clone(), &move_start_index, false);
}

fn parse_move(line: &str) -> (usize, usize, usize) {
	let split: Vec<usize> = line
		.split(" ")
		.filter_map(|s| s.parse::<usize>().ok())
		.collect();
	return (split[0], split[1], split[2]);
}

fn puzzle<'a>(
	lines: &Vec<&'a str>,
	mut crate_map: HashMap<usize, Vec<char>>,
	start_index: &usize,
	reverse: bool,
) {
	// Move the crates
	let mut index: usize = *start_index;
	while index + 1 < lines.len() {
		index += 1;
		let line: &str = lines[index];
		if line.len() == 0 {
			continue;
		}
		let (amount, from, to) = parse_move(line);
		let entry = crate_map.entry(from).or_insert_with(Vec::new);
		let i = entry.len();
		let mut entries = entry.drain((i - amount)..i).collect::<Vec<char>>();
		// Reverses for puzzle 1
		if reverse {
			entries.reverse();
		}
		crate_map
			.entry(to)
			.or_insert_with(Vec::new)
			.append(&mut entries);
	}


	// Calculate result
	let mut top: String = "".to_owned();
	for i in 1..crate_map.len() + 1 {
		top.push(*crate_map[&i].last().unwrap());
	}
	println!("Puzzle {} result: {}", if reverse { 1 } else { 2 }, top);
}
