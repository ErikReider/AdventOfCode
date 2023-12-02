use std::{
	collections::{HashSet, VecDeque},
	fs, process,
};

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};
	let chars: Vec<char> = input.trim().chars().collect();

	look_for_marker(&chars, 4, 1);

	look_for_marker(&chars, 14, 2);
}

fn look_for_marker(chars: &Vec<char>, look_back: usize, puzzle_num: i8) {
	let mut queue: VecDeque<char> = VecDeque::new();
	for (i, c) in chars.iter().enumerate() {
		queue.push_back(*c);
		if queue.len() == look_back + 1 {
			queue.pop_front();
			let set: HashSet<char> = HashSet::from_iter(queue.clone());
			if set.len() == look_back {
				println!("Puzzle {} result: {}", puzzle_num, i + 1);
				break;
			}
		}
	}
}
