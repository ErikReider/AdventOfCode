use std::{cell::RefCell, fs, process, rc::Rc};

const TOTAL_SPACE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

struct Directory {
	parent: Option<Rc<RefCell<Directory>>>,
	children: Vec<Rc<RefCell<Directory>>>,
	files_size: usize,
}

impl Directory {
	pub fn new() -> Directory {
		return Directory {
			files_size: 0,
			children: vec![],
			parent: None,
		};
	}

	pub fn calc_total_size(&self) -> usize {
		if self.children.is_empty() {
			return self.files_size;
		} else {
			let children_size: Vec<usize> = self
				.children
				.iter()
				.map(|child| child.borrow().calc_total_size())
				.collect();
			return self.files_size + children_size.iter().sum::<usize>();
		}
	}

	pub fn get_children_sizes(&self) -> Vec<usize> {
		let calc_size = self.calc_total_size();
		if self.children.is_empty() {
			return vec![calc_size];
		} else {
			let mut children_sizes: Vec<usize> = vec![calc_size];
			for child in self.children.iter() {
				let value = child.borrow().get_children_sizes();
				children_sizes.extend_from_slice(&value);
			}
			return children_sizes;
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
	let lines = input.trim().split("\n").collect::<Vec<&str>>();

	solve_puzzles(&lines);
}

fn solve_puzzles(lines: &Vec<&str>) {
	let root = Rc::new(RefCell::new(Directory::new()));
	let mut current = Rc::clone(&root);
	let mut index: usize = 0;
	'main_loop: while index < lines.len() {
		let command: &str = lines[index];
		match command.get(..4) {
			Some("$ ls") => loop {
				index += 1;
				let output: &str = match lines.get(index) {
					Some(o) if o.starts_with("dir") => continue,
					Some(o) if !o.starts_with("$") => o,
					_ => continue 'main_loop,
				};
				let split = output.split(' ').collect::<Vec<&str>>();
				current.borrow_mut().files_size +=
					str::parse::<usize>(split[0]).unwrap();
			},
			Some("$ cd") => {
				current = match command.split(" ").last().unwrap() {
					"/" => Rc::clone(&root),
					".." => Rc::clone(
						Rc::clone(&current).borrow().parent.as_ref().unwrap(),
					),
					_ => {
						let child = Rc::new(RefCell::new(Directory::new()));
						child.borrow_mut().parent = Some(Rc::clone(&current));
						current.borrow_mut().children.push(Rc::clone(&child));
						child
					}
				};
			}
			c => {
				eprintln!("Command not valid!: {:?}", c);
				continue 'main_loop;
			}
		}
		index += 1;
	}
	// Puzzle 1
	let mut total_size: usize = 0;
	let mut sizes = root.borrow().get_children_sizes();
	sizes.sort();
	for size in sizes.iter() {
		if *size <= 100000 {
			total_size += size;
		}
	}
	println!("Puzzle 1: {}", total_size);

	// Puzzle 2
	// Find out space to remove
	let space_to_rm: usize =
		SPACE_NEEDED - (TOTAL_SPACE - root.borrow().calc_total_size());
	for size in sizes.iter() {
		if *size >= space_to_rm {
			println!("Puzzle 2: {}", size);
			break;
		}
	}
}
