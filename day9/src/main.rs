use std::{cell::RefCell, collections::HashSet, fs, process, rc::Rc};

enum Direction {
	Left,
	Right,
	Up,
	Down,
}

struct Segment {
	x: i32,
	y: i32,
}

impl Segment {
	fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn step(&mut self, dir: &Direction, follows: Option<Rc<RefCell<Segment>>>) {
		if let Some(follows) = follows {
			let follows = follows.borrow();
			// Skip step if already touching sibling
			if ((self.x - 1)..=(self.x + 1)).contains(&follows.x)
				&& ((self.y - 1)..=(self.y + 1)).contains(&follows.y)
			{
				return;
			}

			// Move diagonally if not on the same axis
			let edges: [(i32, i32); 4] =
				if follows.x != self.x && follows.y != self.y {
					// Check if any corner touches the sibling
					[
						(self.x + 1, self.y + 1), // Top right
						(self.x - 1, self.y + 1), // Top left
						(self.x + 1, self.y - 1), // Bottom right
						(self.x - 1, self.y - 1), // Bottom left
					]
				} else {
					// Check if any non-corner edges touches the sibling
					[
						(self.x + 1, self.y), // Middle right
						(self.x - 1, self.y), // Middle left
						(self.x, self.y + 1), // Bottom middle
						(self.x, self.y - 1), // Bottom middle
					]
				};
			for edge in edges {
				if ((edge.0 - 1)..=(edge.0 + 1)).contains(&follows.x)
					&& ((edge.1 - 1)..=(edge.1 + 1)).contains(&follows.y)
				{
					self.x = edge.0;
					self.y = edge.1;
					return;
				}
			}
		}
		// Move in dir if segment is on same axis or if head
		match dir {
			Direction::Left => self.x -= 1,
			Direction::Right => self.x += 1,
			Direction::Up => self.y += 1,
			Direction::Down => self.y -= 1,
		}
	}
}

fn main() {
	let input: String = match fs::read_to_string("./input") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("File Read Error: {}", e);
			process::exit(1);
		}
	};
	let lines = input.lines().collect::<Vec<&str>>();

	let puzzle1 = solve_puzzle(&lines, 2);
	println!("Puzzle 1: {}", puzzle1);

	let puzzle2 = solve_puzzle(&lines, 10);
	println!("Puzzle 2: {}", puzzle2);
}

fn solve_puzzle(lines: &Vec<&str>, num_segements: usize) -> usize {
	// Init segments
	let mut segments: Vec<Rc<RefCell<Segment>>> = Vec::new();
	for _ in 0..num_segements {
		segments.push(Rc::new(RefCell::new(Segment::new(0, 0))));
	}

	let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
	for line in lines {
		let (dir, moves) = line.split_at(2);
		let direction = match dir.trim_end() {
			"L" => Direction::Left,
			"R" => Direction::Right,
			"U" => Direction::Up,
			"D" => Direction::Down,
			e => {
				eprintln!("Could not parse {}!...", e);
				continue;
			}
		};
		for _ in 1..=str::parse::<i32>(moves).unwrap().abs() {
			// Move each segment
			for (i, seg) in segments.iter().enumerate() {
				let follows: Option<Rc<RefCell<Segment>>> = if i == 0 {
					None
				} else {
					Some(Rc::clone(&segments[i - 1]))
				};
				seg.borrow_mut().step(&direction, follows);
			}
			let t_pos = segments.last().unwrap().borrow();
			tail_visited.insert((t_pos.x, t_pos.y));
		}
	}
	tail_visited.len()
}
