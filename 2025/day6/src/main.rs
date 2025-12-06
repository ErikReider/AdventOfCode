use std::fs;

fn main() {
	run("./test_input.txt"); // Part 1: 4277556, Part 2: 3263827
	run("./input.txt"); //Part 1: 8108520669952, Part 2: 11708563470209
}

fn run(path: &'static str) {
	let binding = fs::read_to_string(path).expect("Could not open file");
	let rows: Vec<&str> = binding.split_inclusive("\n").collect::<Vec<&str>>();
	assert!(rows.len() > 2);
	let (operators, rows) = rows.split_last().expect("Could not split_last");

	let (mut part1_sum, mut part2_sum): (u64, u64) = (0, 0);
	let rows: Vec<Vec<char>> = rows
		.iter()
		.map(|l| l.chars().collect::<Vec<char>>())
		.collect();
	let operators: Vec<char> = operators.chars().collect();
	let mut previous_i = 0;
	for chunk in operators.chunk_by(|&_a, &b| b != '*' && b != '+') {
		let len = chunk.len();
		let operator = chunk[0];
		assert!(operator == '*' || operator == '+');

		let mut part1_result: Option<u64> = None;
		for row in &rows {
			let row: String = row.iter().skip(previous_i).take(len).collect();
			let row = row.trim();
			match row.parse::<u64>() {
				Ok(value) => match operator {
					'*' => part1_result = Some(part1_result.map_or(value, |v| v * value)),
					'+' => part1_result = Some(part1_result.map_or(value, |v| v + value)),
					e => panic!("Operator error: {e}"),
				},
				e => panic!("{e:?}: {row}: {previous_i}, {len} {operator}"),
			}
		}

		let mut part2_result: Option<u64> = None;
		for i in 0..(len - 1) {
			let number: String = rows.iter().map(|row| row[previous_i + i]).collect();
			match number.trim().parse::<u64>() {
				Ok(value) => match operator {
					'*' => part2_result = Some(part2_result.map_or(value, |v| v * value)),
					'+' => part2_result = Some(part2_result.map_or(value, |v| v + value)),
					e => panic!("Operator error: {e}"),
				},
				e => panic!("{e:?}: {number}: {previous_i}, {len} {operator}"),
			}
		}

		part1_sum += part1_result.unwrap_or(0);
		part2_sum += part2_result.unwrap_or(0);

		previous_i += len;
	}

	println!("Part 1 Sum: {part1_sum}");
	println!("Part 2 Sum: {part2_sum}");
}
