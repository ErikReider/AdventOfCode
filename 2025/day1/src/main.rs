use std::fs;

fn main() {
	run("./test_input.txt");
	run("./input.txt");
}

fn run(path: &'static str) {
	let binding = fs::read_to_string(path).expect("Could not open file");
	let input: Vec<&str> = binding.trim().split("\n").collect();

	let mut start: i32 = 50;
	let mut end_at_zero: i32 = 0;
	let mut touch_zero: i32 = 0;
	for line in input {
		let (dir, diff) = line.split_at(1);
		let (dir, diff) = match (dir, diff.parse::<i32>()) {
			("L", Ok(diff)) => (-1, -diff),
			("R", Ok(diff)) => (1, diff),
			_ => panic!(),
		};

		// Brute force :)
		for i in 0..diff.abs() {
			if (start + (i * dir)).rem_euclid(100) == 0 {
				touch_zero += 1;
			}
		}

		start = (start + diff).rem_euclid(100);
		if start == 0 {
			end_at_zero += 1;
		}
	}
	println!("File: {}", path);
	println!("  End at zero: {}", end_at_zero);
	println!("  Touch zero: {}", touch_zero);
}
