use std::fs;

fn main() {
	// Part 1
	run::<2>("./test_input.txt"); // 357
	run::<2>("./input.txt"); // 17155

	// Part 2
	run::<12>("./test_input.txt"); // 3121910778619
	run::<12>("./input.txt"); // 169685670469164
}

fn run<const N_DIGITS: usize>(path: &'static str) {
	let binding = fs::read_to_string(path).expect("Could not open file");
	let mut sum: u64 = 0;
	for line in binding.trim().split("\n").collect::<Vec<&str>>() {
		let digits: Vec<char> = line.chars().collect();
		// Sliding window but limiting the window size to fit remaining N digits
		let mut offset: usize = 0;
		for limit in (0..N_DIGITS).rev() {
			let mut largest: u64 = 0;
			let mut largest_index: usize = 0;
			let window = digits.iter().take(digits.len() - limit).skip(offset);
			for (i, digit) in window.enumerate() {
				let digit = digit.to_digit(10).expect("Could not convert digit") as u64;
				if digit > largest {
					largest_index = offset + i + 1;
					largest = digit;
					// End early
					if digit == 9 {
						break;
					}
				}
			}
			offset = largest_index;
			sum += largest * 10_u64.pow(limit as u32);
		}
	}
	println!("Sum: {}", sum);
}
