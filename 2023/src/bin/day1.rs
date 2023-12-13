use std::str::FromStr;

use aoc2023::{ParseFile, RuntimeError};

fn main() -> Result<(), RuntimeError> {
	let cal = Calibration::parse_file("input/day1")?;

	let mut sum = 0;
	for (ln, line) in cal.lines.iter().enumerate() {
		// The digits (0-9) in the line with their positions
		let mut digits: Vec<(usize, u32)> = line
			.chars()
			.enumerate()
			.filter_map(|(idx, c)| c.to_digit(10).map(|d| (idx, d)))
			.collect();

		let first_digit = digits.first();
		let first_word_digit = find_word_digit(line);

		// these might be the same as the firsts's but that's fine
		let last_digit = digits.last();
		let last_word_digit = rfind_word_digit(line);

		let (tens_idx, tens) = match (first_digit, first_word_digit) {
			(Some((digit_idx, digit_num)), Some((word_idx, word_num))) => {
				if *digit_idx > word_idx {
					(word_idx, word_num)
				} else {
					(*digit_idx, *digit_num as usize)
				}
			}
			(Some((digit_idx, digit_num)), None) => (*digit_idx, *digit_num as usize),
			(None, Some(word)) => word,
			(None, None) => {
				eprintln!("{ln}: line has no numbers of any form");
				std::process::exit(1);
			}
		};

		let (ones_idx, ones) =
			match (last_digit, last_word_digit) {
				(Some((digit_idx, digit_num)), Some((word_idx, word_num))) => {
					println!("{ln}: [{digit_idx},{digit_num}] [{word_idx},{word_num}]");

					if *digit_idx < word_idx {
						(word_idx, word_num)
					} else {
						(*digit_idx, *digit_num as usize)
					}
				}
				(Some((digit_idx, digit_num)), None) => (*digit_idx, *digit_num as usize),
				(None, Some(word)) => word,
				(None, None) => {
					eprintln!("{ln}: line has no numbers of any form. we're at last, how did we get here?");
					std::process::exit(1);
				}
			};

		sum += tens * 10 + ones;

		println!("{line}");
		println!(
			"{:>firstpad$}^{:>secondpad$}^",
			"",
			"",
			firstpad = tens_idx,
			secondpad = (ones_idx - tens_idx).saturating_sub(1)
		);
	}

	println!("sum = {sum}");

	Ok(())
}

const NUMBERS: [&'static str; 10] = [
	"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_word_digit(line: &str) -> Option<(usize, usize)> {
	let mut lowest_idx = None;
	let mut lowest_digit = None;

	for (digit, number) in NUMBERS.iter().enumerate() {
		match (line.find(number), lowest_idx) {
			(Some(found_idx), Some(low_idx)) if found_idx < low_idx => {
				lowest_idx = Some(found_idx);
				lowest_digit = Some(digit);
			}
			(Some(found_idx), None) => {
				lowest_idx = Some(found_idx);
				lowest_digit = Some(digit);
			}
			_ => (),
		}
	}

	// we never set _idx without _digits, but it's still
	// not good to unwrap here. we should be using one varaible
	// as Option<(usize, usize)> but i want to get to day2 and this
	// is advent of code not boeing 747 max *(i don't need to care)*
	lowest_idx.map(|idx| (idx, lowest_digit.unwrap()))
}

// copy pasted 'cause i'm wasted (i'm not drunk i just liked the rhyme)
fn rfind_word_digit(line: &str) -> Option<(usize, usize)> {
	let mut highest_idx = None;
	let mut highest_digit = None;

	for (digit, number) in NUMBERS.iter().enumerate() {
		match (line.rfind(number), highest_idx) {
			(Some(found_idx), Some(high_idx)) if found_idx > high_idx => {
				highest_idx = Some(found_idx);
				highest_digit = Some(digit);
			}
			(Some(found_idx), None) => {
				highest_idx = Some(found_idx);
				highest_digit = Some(digit);
			}
			_ => (),
		}
	}

	// we never set _idx without _digits, but it's still
	// not good to unwrap here. we should be using one varaible
	// as Option<(usize, usize)> but i want to get to day2 and this
	// is advent of code not boeing 747 max *(i don't need to care)*
	highest_idx.map(|idx| (idx, highest_digit.unwrap()))
}

pub struct Calibration {
	pub lines: Vec<String>,
}

impl FromStr for Calibration {
	type Err = RuntimeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			lines: s.lines().map(|s| s.to_owned()).collect(),
		})
	}
}
