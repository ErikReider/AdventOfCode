// Each rucksack has two large compartments.
// All items of a given type are meant to go into exactly one of the two compartments.
// The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

// Input: a list of all of the items currently in each rucksack
//
// Find Errors.

// Every item type is identified by a single lowercase or uppercase letter
//		(that is, a and A refer to different types of items).
//
// The list of items for each rucksack is given as characters all on a single line.
// A given rucksack always has the same number of items in each of its two compartments,
// so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
//
// To help prioritize item rearrangement, every item type can be converted to a priority:
//   - Lowercase item types a through z have priorities 1 through 26.
//   - Uppercase item types A through Z have priorities 27 through 52.

use std::{
	collections::{HashMap, HashSet},
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
	let lines = input.trim().split("\n").collect::<Vec<&str>>();

	puzzle1(&lines);

	puzzle2(&lines);
}

/// Converts Latin Alphabet char into u32 where a-z = 1-16, A-Z = 27-52
fn char_to_u32(character: &char) -> u32 {
	let num = character.to_owned() as u32 - 38;
	if num > 58 {
		return num - 58;
	}
	return num;
}

fn puzzle1(lines: &Vec<&str>) {
	let mut sum_priority: u32 = 0;
	for line in lines {
		// Split the string in half
		let (comp1, comp2) = line.split_at(line.len() / 2);
		// Convert the last half of the string into a HashMap for faster lookup
		let chars: HashSet<char> = HashSet::from_iter(comp2.chars());
		for character in comp1.chars() {
			if let Some(_) = chars.get(&character) {
				sum_priority += char_to_u32(&character);
				break;
			}
		}
	}
	println!("Puzzle 1 Sum: {}", sum_priority);
}

fn puzzle2(lines: &Vec<&str>) {
	let mut sum_priority: u32 = 0;
	let mut index: usize = 0;
	'lines_loop: while index < lines.len() {
		let chars_vec: [HashSet<char>; 3] = [
			HashSet::from_iter(lines[index].chars()),
			HashSet::from_iter(lines[index + 1].chars()),
			HashSet::from_iter(lines[index + 2].chars()),
		];
		index += 3;

		let mut common_chars: HashMap<char, u32> = HashMap::new();
		for chars in chars_vec {
			for c in chars {
				if let Some(i) = common_chars.get_mut(&c) {
					// increase the usage char count
					*i += 1;
					if *i == 3 {
						sum_priority += char_to_u32(&c);
						continue 'lines_loop;
					}
				} else {
					// Insert char if it doesn't exist
					common_chars.insert(c, 1);
				}
			}
		}
	}
	println!("Puzzle 2 Sum: {}", sum_priority);
}
