use std::fs;

fn main() {
	run("./test_input.txt"); // Part 1: 3, Part 2: 14
	run("./input.txt"); //Part 1: 798, Part 2: 366181852921027
}

fn run(path: &'static str) {
	let binding = fs::read_to_string(path).expect("Could not open file");
	let (ranges, ids) = binding.trim().split_once("\n\n").expect("Could not split");

	let mut ranges_list: Vec<(u64, u64)> = ranges
		.trim()
		.split("\n")
		.filter_map(|range| match range.split_once("-") {
			Some((start, end)) => match (start.parse::<u64>(), end.parse::<u64>()) {
				(Ok(start), Ok(end)) => Some((start, end)),
				_ => None,
			},
			_ => None,
		})
		.collect();
	ranges_list.sort_by_key(|&range| range.0);

	// Merge the ranges
	let mut ranges: Vec<(u64, u64)> = vec![];
	let mut begin: Option<(u64, u64)> = None;
	for (i, &(start, end)) in ranges_list.iter().enumerate() {
		begin = if let Some((begin_start, begin_end)) = begin {
			if (begin_start..=begin_end).contains(&start) {
				// Connected, merge
				Some((begin_start, begin_end.max(end)))
			} else {
				// Not connected, add the previous marged range to the list
				ranges.push((begin_start, begin_end));
				Some((start, end))
			}
		} else {
			Some((start, end))
		};
		if i + 1 == ranges_list.len()
			&& let Some(begin_range) = begin
		{
			ranges.push(begin_range);
		}
	}

	// Calculate the number of fresh ingredients
	let mut n_fresh: u64 = 0;
	for id in ids.trim().split("\n") {
		if let Ok(id) = id.parse::<u64>()
			&& ranges
				.iter()
				.any(|&(start, end)| (start..=end).contains(&id))
		{
			n_fresh += 1;
		}
	}

	println!("Part 1: {}", n_fresh);
	println!(
		"Part 2: {}",
		ranges
			.iter()
			.map(|(start, end)| end + 1 - start)
			.sum::<u64>()
	);
}
