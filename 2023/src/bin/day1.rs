use std::str::FromStr;

use aoc2023::{ParseFile, RuntimeError};

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

fn main() -> Result<(), RuntimeError> {
	let cal = Calibration::parse_file("input/day1")?;

	let mut sum = 0;
	for (ln, line) in cal.lines.iter().enumerate() {
		let (mut tens, mut ones) = (None, None);

		for char in line.chars() {
			if let Some(digit) = char.to_digit(10) {
				if tens.is_some() {
					ones = Some(digit);
				} else {
					tens = Some(digit);
				}
			}
		}

		match (tens, ones) {
			(Some(tens), None) => {
				sum += tens * 10 + tens;
			}
			(Some(tens), Some(ones)) => {
				sum += tens * 10 + ones;
			}
			(None, _) => {
				eprintln!("{ln}: didn't have a digit!");
				std::process::exit(1);
			}
		}
	}

	println!("sum = {sum}");

	Ok(())
}
