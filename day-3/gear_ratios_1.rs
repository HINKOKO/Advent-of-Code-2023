use std::fs;

fn is_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
	let n = grid.len();
	let m = grid[0].len();

	if i >= n || j >= m {
		return false;
	}
	grid[i][j] != '.' && !grid[i][j].is_digit(10)
}

// if is_symbol(&grid, i, start.saturating_sub(1))

fn main() {
	let puzzle = "puzzle.txt";
	let data = fs::read_to_string(puzzle).expect("Unable to open your puzzle");
	let lines: Vec<String> = data.trim().split("\n").map(|s| s.to_string()).collect();
	
	let m = lines[0].len(); // All lines are same len

	let mut ans = 0;

	let mut grid: Vec<Vec<char>> = Vec::new();

	for line in &lines {
		let chars: Vec<char> = line.trim().chars().collect();
		grid.push(chars);
	}

	for (i, line) in grid.iter().enumerate() {
		let mut j = 0;
		let mut start;

		while j < m {
			start = j; // 'start' will keep track of the start position of an encountered digit
			let mut num = String::new();

			while j < m && line[j].is_digit(10) {
				num.push(line[j]);
				j += 1;
			}

			if num.is_empty() {
				j += 1;
				continue;
			}

			// 'Shadowing' variable 'num' by parsing it into num
			let num: usize = num.parse().expect("Unable to parse the string'ed number");

			if is_symbol(&grid, i, start.saturating_sub(1)) || is_symbol(&grid, i, j) {
				ans += num;
				j += 1;
				continue;
			}

			// saturating_sub ensures that if start - 1 < 0 , it 'saturates' or block at 0
			for k in (start.saturating_sub(1))..(j + 1) {
				// i - 1 prev line, i + 1 next line, checking "around" the digits or group of digit
				if is_symbol(&grid, i.saturating_sub(1), k) || is_symbol(&grid, i + 1, k) {
					ans += num;
					break;
				}

			}
		}
	}
	println!("Puzzle answer is => {}", ans);
}
