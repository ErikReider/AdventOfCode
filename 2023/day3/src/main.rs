use std::{
	char,
	collections::{HashMap, HashSet},
	fs,
	ops::Range,
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum SymbolType {
	Number(u32),
	Gear,
	Other,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
	symbol_value: SymbolType,
	x: Range<usize>,
	y: usize,
}

impl Symbol {
	fn get_symbol_at_pos(
		x: usize,
		x_offset: i32,
		y: usize,
		y_offset: i32,
		symbols: &mut HashSet<Symbol>,
		grid: &Vec<Vec<char>>,
	) {
		if let (y, false) = (y as i32).overflowing_add(y_offset) {
			if let (x, false) = (x as i32).overflowing_add(x_offset) {
				if let Some(row) = grid.get(y as usize) {
					if let Some(value) = row.get(x as usize) {
						if !value.is_digit(10) {
							let symbol_type: SymbolType = match value {
								'.' => return,
								'*' => SymbolType::Gear,
								_ => SymbolType::Other,
							};
							symbols.insert(Symbol {
								symbol_value: symbol_type,
								x: (x as usize)..(x as usize),
								y: (y as usize),
							});
						}
					}
				}
			}
		}
	}

	pub fn get_surrounding_symbols(
		&self,
		grid: &Vec<Vec<char>>,
	) -> HashSet<Symbol> {
		let mut symbols: HashSet<Symbol> = HashSet::new();
		for x in self.x.clone() {
			// Check start values if at end digit
			Symbol::get_symbol_at_pos(x, -1, self.y, -1, &mut symbols, grid);
			Symbol::get_symbol_at_pos(x, -1, self.y, 0, &mut symbols, grid);
			Symbol::get_symbol_at_pos(x, -1, self.y, 1, &mut symbols, grid);

			// Check above and below values
			Symbol::get_symbol_at_pos(x, 0, self.y, -1, &mut symbols, grid);
			Symbol::get_symbol_at_pos(x, 0, self.y, 1, &mut symbols, grid);

			// Check end values if at end digit
			Symbol::get_symbol_at_pos(x, 1, self.y, -1, &mut symbols, grid);
			Symbol::get_symbol_at_pos(x, 1, self.y, 0, &mut symbols, grid);
			Symbol::get_symbol_at_pos(x, 1, self.y, 1, &mut symbols, grid);
		}

		symbols
	}
}

fn main() {
	let (sum, gear_ratio) = solve();

	println!("Part 1: {:?}", sum);

	println!("Part 2: {:?}", gear_ratio);
}

fn get_number(chars: &Vec<char>, start_x: usize, y: usize) -> Symbol {
	let mut num: String = String::new();
	let mut end_x: usize = chars.len() - 1;
	for (x, c) in chars.iter().skip(start_x).enumerate() {
		if c.is_digit(10) {
			num.push(*c);
		} else {
			end_x = x + start_x;
			break;
		}
	}

	Symbol {
		symbol_value: SymbolType::Number(num.parse::<u32>().unwrap()),
		x: start_x..end_x,
		y,
	}
}

fn solve() -> (u32, u32) {
	let input = fs::read_to_string("./input").expect("Could not read file...");

	// Map out the grid and get each number with its coordinates
	let mut positions: Vec<Symbol> = vec![];
	let mut grid: Vec<Vec<char>> = vec![];
	for (y, line) in input.split("\n").enumerate() {
		if line.len() == 0 {
			continue;
		}

		let mut row: Vec<char> = vec![];

		let mut x: usize = 0;
		let chars: Vec<char> = line.chars().collect();
		let mut last_end_x = 0;
		while x < chars.len() {
			let c: &char = chars.get(x).unwrap();
			row.push(*c);

			if c.is_digit(10) {
				let pos: Symbol = get_number(&chars, x, y);
				if pos.x.end > last_end_x {
					last_end_x = pos.x.end;
					positions.push(pos);
				}
			}

			x += 1;
		}
		grid.push(row);
	}

	// Check each number for surrounding symbols
	let mut sum: u32 = 0;
	let mut gears: HashMap<String, Vec<u32>> = HashMap::new();
	for pos in positions {
		let symbols = pos.get_surrounding_symbols(&grid);
		if !symbols.is_empty() {
			if let SymbolType::Number(num) = pos.symbol_value {
				sum += num;
				// Get all gears
				for symbol in symbols.iter() {
					if symbol.symbol_value == SymbolType::Gear {
						let key = format!("{}x{}", symbol.x.start, symbol.y);
						if let Some(array) = gears.get_mut(&key) {
							array.push(num);
						} else {
							gears.insert(key, vec![num]);
						};
					}
				}
			}
		}
	}

	// Get Gear ratio sum
	let mut gear_ratio_sum: u32 = 0;
	for gear in gears.values() {
		if gear.len() == 2 {
			gear_ratio_sum += gear.iter().product::<u32>();
		}
	}

	(sum, gear_ratio_sum)
}
