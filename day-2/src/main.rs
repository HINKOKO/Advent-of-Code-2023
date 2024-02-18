use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


fn main() {

	let pattern = Regex::new(r"(\d+) (red|green|blue)").unwrap();
	let possible: Vec<(&str, u32)> = vec![("red", 12), ("green", 13), ("blue", 14)];
	let mut total = 0;

	if let Ok(puzzle) = File::open("sample.txt") {
		let reader = BufReader::new(puzzle);

		for (i, line) in reader.lines().enumerate() {
			if let Ok(line) = line {
				let mut valid = true;

				for cap in pattern.captures_iter(&line) {
					let num: u32 = cap[1].parse().unwrap();
					let color = &cap[2];

					if let Some(&(_, max_count)) = possible.iter().find(|&&(c, _)| c == color) {
						if num > max_count {
							valid = false;
							break;
						}
					}
				}
				if valid {
					total += i + 1;
				}
			}
		}
	}
	println!("{}", total)
}
