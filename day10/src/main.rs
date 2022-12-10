use std::{fs, process};

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};

	let mut register: i32 = 1;
	let mut cycle: i32 = 0;
	let mut sum_signal_strength: i32 = 0;
	let mut crt = [[' '; 40]; 6];
	let mut draw_row: usize = 0;
	for line in input.lines() {
		let split: Vec<&str> = line.split(' ').collect();
		let (ticks, value): (usize, i32) = match split[0] {
			"addx" => (2, split[1].parse().unwrap()),
			"noop" => (1, 0),
			_ => continue,
		};
		for _ in 0..ticks {
			// Draw
			let row_cycle: i32 = cycle - 40 * draw_row as i32;
			let sprite_pos = register - 1..=register + 1;
			if sprite_pos.contains(&row_cycle) {
				crt[draw_row][row_cycle as usize] = '#';
			}

			cycle += 1;
			if cycle % 40 == 0 {
				draw_row += 1;
			}
			if cycle % 40 - 20 == 0 {
				sum_signal_strength += cycle * register;
			}
		}

		register += value;
	}
	println!("Puzzle 1: {}\n", sum_signal_strength);

	println!("Puzzle 2:");
	for row in crt {
		for c in row {
			print!("{}", c);
		}
		print!("\n");
	}
}
