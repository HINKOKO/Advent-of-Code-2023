use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn main() -> io::Result<()> {
	let mut input: Vec<String> = Vec::new();
	let file = File::open("sample.txt")?;
	let reader = BufReader::new(file);

	for line in reader.lines() {
		input.push(line?);
	}
	let res = part_1(&input);
	println!("Part 1 => {}", res);
	Ok(())
}

fn part_1(puzzle: &[String]) -> usize {
	let mut total = 0;

	for line in puzzle {
		let mut card_score = 0;
		let cards: Vec<&str> = line.split(" | ").collect();
		if let Some(winners_part) = cards[0].split(":").nth(1) {
			let winners: Vec<&str> = winners_part.trim().split_whitespace().collect();
			let tickets: Vec<&str> = cards[1].trim().split_whitespace().collect();

			for &num in &tickets {
				if winners.contains(&num) {
					if card_score == 0 {
						card_score += 1;
					} else {
						card_score *= 2;
					}
				}
			}
			total += card_score;
		}
	}
	total
}
