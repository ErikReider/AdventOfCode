use std::{fs, process};

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};
	let lines = input.trim().split("\n").collect::<Vec<&str>>();

	// Init Map
	let mut map: Vec<Vec<usize>> = Vec::new();
	for line in lines {
		let mut row: Vec<usize> = Vec::new();
		for c in line.trim().chars() {
			let num: usize = c.to_digit(10).unwrap() as usize;
			row.push(num);
		}
		map.push(row);
	}
	let height = map.len();
	let width = map[0].len();

	let mut inner_visible: usize = 0;
	let mut scenic_score: usize = 0;
	for y in 1..height - 1 {
		for x in 1..width - 1 {
			let tree: usize = map[y][x];
			let left: Vec<usize> = map[y][0..x].iter().cloned().rev().collect();
			let right: Vec<usize> = map[y][x + 1..width].to_vec();
			let top: Vec<usize> =
				map[0..y].iter().map(|r| r[x]).rev().collect();
			let bottom: Vec<usize> =
				map[y + 1..height].iter().map(|r| r[x]).collect();

			let mut visible_score: usize = 0;
			let mut score: usize = 1;
			'dirs_loop: for dir in [&left, &right, &top, &bottom] {
				for (i, height) in dir.iter().enumerate() {
					if height >= &tree {
						visible_score += 1;
						score *= i + 1;
						continue 'dirs_loop;
					}
				}
				score *= dir.len();
			}
			if visible_score < 4 {
				inner_visible += 1;
			}
			if score > scenic_score {
				scenic_score = score;
			}
		}
	}

	let visible = 2 * (height + width) - 4 + inner_visible;
	println!("Puzzle 1: {}", visible);
	println!("Puzzle 2: {}", scenic_score);
}
