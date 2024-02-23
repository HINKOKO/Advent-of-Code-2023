use std::fs::File;
use std::io::{self, BufRead};



// if is_symbol(&grid, i, start.saturating_sub(1))

fn main() -> io::Result<()> {
	let file = File::open("puzzle_real.txt")?;
	let reader = io::BufReader::new(file);
	let mut lines = Vec::new();

	for line in reader.lines() {
		lines.push(line?);
	}

	let n = lines.len();
	let m = lines[0].len();

	let mut ans = 0;

	let mut goods: Vec<Vec<Vec<i32>>> = vec![vec![Vec::new(); m]; n];
	
	
	fn is_symbol(lines: &Vec<String>, goods: &mut Vec<Vec<Vec<i32>>>, i: usize, j: usize, num: i32) -> bool {
		let n = lines.len();
		let m = lines[0].len();

		if !(i < n && j < m) {
			return false;
		}
		if lines[i].chars().nth(j).unwrap() == '*' {
			goods[i][j].push(num);
		}
		lines[i].chars().nth(j).unwrap() != '.' && !lines[i].chars().nth(j).unwrap().is_digit(10)
	}

	for (i, line) in lines.iter().enumerate() {
		let mut start = 0;
		let mut j = 0;

		while j < m {
			start = j;
			let mut num = String::new();

			while j < m && line.chars().nth(j).unwrap().is_digit(10) {
				num.push(line.chars().nth(j).unwrap());
				j += 1;
			}

			if num.is_empty() {
				j += 1;
				continue;
			}

			// 'Shadowing' variable 'num' by parsing it into num
			let num: i32 = num.parse().unwrap();

			let _ = is_symbol(&lines, &mut goods, i, start.checked_sub(1).unwrap_or(0), num)
				|| is_symbol(&lines, &mut goods, i, j, num);

			// saturating_sub ensures that if start - 1 < 0 , it 'saturates' or block at 0
			for k in (start.checked_sub(1).unwrap_or(0))..(j + 1) {
				// i - 1 prev line, i + 1 next line, checking "around" the digits or group of digit
				let _ = is_symbol(&lines, &mut goods, i.checked_sub(1).unwrap_or(0), k, num) ||
					is_symbol(&lines, &mut goods, i + 1, k, num);

			}
		}
	}
	for i in 0..n {
		for j in 0..m {
			let nums = &goods[i][j];
			if lines[i].chars().nth(j).unwrap() == '*' && nums.len() == 2 {
				ans += nums[0] * nums[1];
			}
		}
	}
	println!("answer of puzzle => {}", ans);
	Ok(())
	
}
