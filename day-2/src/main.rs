use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;




fn main() {
	let pat = Regex::new(r"(\d+) (red|green|blue)").unwrap();
	let possible: Vec<(&str, u32)> = vec![("red", 12), ("green", 13), ("blue", 14)];
	let mut total = 0;

	if let Ok(puzzle) = File::open("sample.txt") {
		let reader = BufReader::new(puzzle);
		for (i, line) in reader.lines().enumerate() {
			// println!("{:?} {:?}", i, line);
			if let Ok(line) = line {
				let mut valid = true;
				for cap in pat.captures_iter(&line) {
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
	println!("Total of puzzle part 1 => {}", total);
	part_two();
}

fn part_two() {
	let pat = Regex::new(r"(\d+) (red|green|blue)").unwrap();
	let mut total = 0;

	if let Ok(puzzle) = File::open("puzzle.txt") {
		let reader = BufReader::new(puzzle);

		for (i, line) in reader.lines().enumerate() {
			if let Ok(line) = line {
				let mut max_vals: Vec<(&str, u32)> = vec![("red", 0), ("green", 0), ("blue", 0)];
				for cap in pat.captures_iter(&line) {
					if let (Some(num_str), Some(color)) = (cap.get(1), cap.get(2)) {
						if let Ok(num) = num_str.as_str().parse::<u32>() {
							for (max_color, max_val) in &mut max_vals {
								if color.as_str() == *max_color && num > *max_val {
									*max_val = num;
								} 
							}
						}
					}
				}
				let mut prod: u32 = 1;
				for (_, max_val) in &max_vals {
					prod *= *max_val;
				}
				total += prod;
			}

		}
	}
	println!("Total of puzzle part 2 => {}", total);
}
