use std::{fs, process, vec};

// Input: the number of Calories each Elf is carrying
fn main() {
	let (calories, len) = get_sorted_calories();

	// Question: How many Calories are being carried by the Elf carrying the most Calories?
	println!(
		"Puzzle 1: Largest amount of calories: {}",
		calories[len - 1]
	);

	// Question: Find the top three Elves carrying the most Calories.
	// How many Calories are those Elves carrying in total?
	let elfs: usize = 3;
	println!("Puzzle 2: {}", calories[len - elfs..].iter().sum::<i32>());
}

fn get_sorted_calories() -> (Vec<i32>, usize) {
	let input = fs::read_to_string("./input").expect("Could not read file...");

	let mut calories: Vec<i32> = vec![];
	let mut sum_calories: i32 = 0;
	for line in input.split("\n") {
		if line.len() == 0 {
			calories.push(sum_calories);
			sum_calories = 0;
			continue;
		}
		if let Ok(num) = line.parse::<i32>() {
			sum_calories += num;
		} else {
			eprintln!("Line: {}, is not a number!...", line);
			process::exit(1);
		}
	}
	calories.sort_by(|a, b| a.cmp(b));
	let len = calories.len();

	(calories, len)
}
