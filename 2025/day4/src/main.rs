use std::fs;

fn main() {
	// Part 1: 13, Part 2: 43
	run("./test_input.txt");
	// Part 1: 1495, Part 2: 8768
	run("./input.txt");
}

fn run(path: &'static str) {
	let binding = fs::read_to_string(path).expect("Could not open file");
	let mut chars: Vec<Vec<char>> = binding
		.trim()
		.split("\n")
		.map(|l| l.chars().collect::<Vec<char>>())
		.collect();

	let mut first_removed = 0;
	let mut total_removed = 0;
	while let n_removed = remove_rols(&mut chars)
		&& n_removed > 0
	{
		if first_removed == 0 {
			first_removed = n_removed;
		}
		total_removed += n_removed;
	}
	println!("First Removed {} rolls", first_removed);
	println!("Num Removed   {} rolls", total_removed);
}

fn remove_rols(chars: &mut [Vec<char>]) -> u32 {
	#[rustfmt::skip]
	const INDICES: [(isize, isize); 8] = [
		(-1, -1), (0, -1), (1, -1),
		(-1, 0), /* (0, 0), */ (1, 0),
		(-1, 1), (0, 1), (1, 1),
	];

	let mut n_removed = 0;
	let initial_state = chars.to_owned();
	for (y_offset, line) in initial_state.iter().enumerate() {
		for (x_offset, &current_char) in line.iter().enumerate() {
			if current_char != '@' {
				continue;
			}
			let mut count = 0;
			for (x, y) in INDICES {
				if let (Some(x), Some(y)) = (
					x_offset.checked_add_signed(x),
					y_offset.checked_add_signed(y),
				) && x < line.len()
					&& y < initial_state.len()
					&& initial_state[y][x] == '@'
				{
					count += 1;
				}
			}
			if count < 4 {
				chars[y_offset][x_offset] = 'x';
				n_removed += 1;
			}
		}
	}
	n_removed
}
