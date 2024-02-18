// For part 1
use std::fs::File;
use std::io::{self, BufRead};


// =========== PART 2 =====================
// const NUMS: [&[u8]; 9] = [
//     b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
// ];

// pub fn main() {
//     println!(
//         "{}",
//         include_bytes!("../puzzle.txt")
//             .split(|b| b == &b'\n')
//             .map(|line| {
//                 (0..line.len()).find_map(|i| num(line, i)).unwrap_or(0) * 10
//                     + (0..line.len()).rev().find_map(|i| num(line, i)).unwrap_or(0)
//             })
//             .sum::<usize>()
//     );
// }

// #[inline(always)]
// fn num(line: &[u8], i: usize) -> Option<usize> {
//     line[i]
//         .is_ascii_digit()
//         .then_some((line[i] - b'0') as usize)
//         .or(NUMS
//             .iter()
//             .enumerate()
//             .find(|(_, name)| line[i..].starts_with(name))
//             .map(|(num, _)| num + 1))
// }

fn first_and_last(s: &str) -> u32 {
    let mut concat: u32 = 0;
    let mut first = None;
    let mut last = None;

    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            if first == None {
                first = Some(digit);
            }
            last = Some(digit);
        }
    }

    if let (Some(first), Some(last)) = (first, last) {
       let concat_str = format!("{}{}", first.to_string(), last.to_string());
       concat = concat_str.parse().unwrap_or(0);
    }
    concat
}

fn main() -> io::Result<()> {
    let mut total: u32 = 0;
    let file = File::open("puzzle.txt")?;
    let reader = io::BufReader::new(file);

    for l in reader.lines() {
        if let Ok(l) = l {
            let twins = first_and_last(&l);
            total += twins;
        }
    }
    println!("{total}");
    
    Ok(())
}


// ============ PART 1 ============

// fn main() -> io::Result<()> {
//     let mut total = 0;
//     let file = File::open("puzzle.txt")?;
//     let reader = io::BufReader::new(file);

//     for l in reader.lines() {
//         if let Ok(l) = l {
//             let twins = my_first_my_last(&l);
//             total += twins;
//         }
//     }
//     println!("{total}");
//     Ok(())
// }

// fn my_first_my_last(s: &str) -> u32 {
//     let mut concat: u32 = 0;
//     let mut first = None;
//     let mut last = None;

//     for c in s.chars() {
//         if let Some(digit) = c.to_digit(10) {
//             if first.is_none() {
//                 first = Some(digit);
//             }
//             last = Some(digit)
//         }
//     }

//     if let (Some(first), Some(last)) = (first, last) {
//         let first_str = first.to_string(); 
//         let last_str = last.to_string();
//         let concat_str = format!("{}{}", first_str, last_str);
//         concat = concat_str.parse().unwrap_or(0);
//     }
//     concat
// }


