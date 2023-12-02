use lazy_static::lazy_static;
use std::{collections::HashMap, fs};

lazy_static! {
	static ref NUMBER_TABLE: HashMap<&'static str, u32> = HashMap::from([
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
		("1", 1),
		("2", 2),
		("3", 3),
		("4", 4),
		("5", 5),
		("6", 6),
		("7", 7),
		("8", 8),
		("9", 9),
	]);
}

fn main() {
	let input = fs::read_to_string("./input").expect("Could not read file...");

	println!("Part 1: {}", get_calibration_values(&input, false));

	println!("Part 2: {}", get_calibration_values(&input, true));
}

fn get_calibration_values(input: &String, parse_spelled_out: bool) -> u32 {
	let mut sum: u32 = 0;
	for line in input.split("\n") {
		if line.len() == 0 {
			continue;
		}
		let mut numbers: Vec<u32> = vec![];
		let chars: Vec<char> = line.chars().collect();
		for (i, _) in line.chars().enumerate() {
			let mut string: String = String::new();
			for c in chars.iter().skip(i) {
				if !parse_spelled_out && !c.is_digit(10) {
					break;
				}
				string.push(*c);
				if let Some(val) = NUMBER_TABLE.get(string.as_str()) {
					numbers.push(*val);
					break;
				}
			}
		}
		if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
			sum += format!("{}{}", first, last).parse::<u32>().unwrap();
		}
	}

	sum
}
