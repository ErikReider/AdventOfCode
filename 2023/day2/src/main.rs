use std::{collections::HashMap, fs};

fn main() {
	let (sum, product) = solve();

	println!("Part 1: {:?}", sum);

	println!("Part 2: {:?}", product);
}

fn solve() -> (u32, u32) {
	let input = fs::read_to_string("./input").expect("Could not read file...");

	let color_max_values: HashMap<&str, u32> =
		HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

	let mut sum: u32 = 0;
	let mut product: u32 = 0;
	for line in input.split("\n") {
		if line.len() == 0 {
			continue;
		}
		let mut exceeds_maximum: bool = false;
		let mut color_values: HashMap<&str, u32> = HashMap::new();

		let (id, data) = line.split_once(": ").unwrap();
		let id = id.trim_start_matches("Game ").parse::<u32>().unwrap();
		for game in data.split("; ").collect::<Vec<&str>>() {
			for play in game.split(", ").collect::<Vec<&str>>() {
				let (num, color) = play.split_once(" ").unwrap();
				let num = num.parse::<u32>().unwrap_or(0);
				if num > *color_max_values.get(color).unwrap_or(&0) {
					exceeds_maximum = true;
				}
				if num > *color_values.get(color).unwrap_or(&0) {
					color_values.insert(color, num);
				}
			}
		}
		if !exceeds_maximum {
			sum += id;
		}
		product += color_values.values().product::<u32>();
	}

	(sum, product)
}
