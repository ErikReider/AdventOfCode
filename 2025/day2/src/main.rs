use std::fs;

fn main() {
	run("./test_input.txt");
	run("./input.txt");
}

fn run(path: &'static str) {
	let binding = fs::read_to_string(path).expect("Could not open file");
	let input: Vec<&str> = binding.trim().split(",").collect();

	let mut sum_part1: i64 = 0;
	let mut sum_part2: i64 = 0;
	for range in input {
		let (first_id, last_id): (i64, i64) = match range.trim().split_once("-") {
			Some(ids) => match (ids.0.parse(), ids.1.parse()) {
				(Ok(first), Ok(last)) => (first, last),
				_ => continue,
			},
			_ => continue,
		};

		for id in first_id..=last_id {
			let chars: Vec<char> = id.to_string().chars().collect();

			// Part 1: 12341234 -> 1234 1234
			// Split in half
			let (start, end) = chars.split_at(chars.len() / 2);
			if start == end {
				sum_part1 += id;
			}

			// Part 2: 123412341234 -> 1234 1234 1234
			// Split into smaller and smaller chunks until found:
			//	- 123412 341234
			//	- 12341 23412 34
			//	- 1234 1234 1234
			//	- 123 412 341 234
			//	- 12 34 12 34 12 34
			for chunk_size in 1..=(chars.len() / 2) {
				let mut chunks = chars.chunks(chunk_size);
				let first_chunk = chunks.next().unwrap();
				if !chunks.any(|chunk| chunk != first_chunk) {
					sum_part2 += id;
					break;
				}
			}
		}
	}
	println!("Part 1 Sum: {}", sum_part1);
	println!("Part 2 Sum: {}", sum_part2);
}
