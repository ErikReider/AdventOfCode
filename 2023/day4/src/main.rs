use std::collections::HashMap;

fn main() {
	let (sum, cards) = solve();
	println!("Part 1: {:?}", sum);

	println!("Part 2: {:?}", cards);
}

fn solve() -> (u32, u32) {
	let input = include_str!("../input");

	let mut matches: HashMap<u32, u32> = HashMap::new();
	let mut cards: Vec<u32> = vec![];
	let mut sum: u32 = 0;
	for line in input.split("\n") {
		if line.len() == 0 {
			continue;
		}
		let (card_num, values) = line.split_once(": ").unwrap();
		let card_num = card_num
			.trim_start_matches("Card ")
			.trim()
			.parse::<u32>()
			.unwrap();

		let (win_values, my_values) = values.split_once(" | ").unwrap();
		let win_values: Vec<u32> = win_values
			.split(" ")
			.filter_map(|v| v.trim().parse::<u32>().ok())
			.collect();
		let my_values: Vec<u32> = my_values
			.split(" ")
			.filter_map(|v| v.trim().parse::<u32>().ok())
			.collect();

		let num_matches =
			my_values.iter().filter(|v| win_values.contains(v)).count();
		matches.insert(card_num, num_matches as u32);
		cards.push(card_num);
		sum += 2_u32.pow(num_matches.checked_sub(1).unwrap_or(0) as u32);
	}

	let mut i = 0;
	while i < cards.len() {
		let card = cards[i];
		let num_matches = matches[&card];
		i += 1;
		for card in (card + 1)..(card + num_matches + 1) {
			cards.push(card);
		}
	}

	(sum, cards.len() as u32)
}
