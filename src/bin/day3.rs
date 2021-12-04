use core::str::FromStr;

use aoc2021::day_parse;

fn main() {
    let dr: DiagnosticReport = day_parse!();
    let (g, e) = dr.gamma_epsilon();

    println!("Gamma {}, Epsilon {}. Product {}", g, e, g * e);
}

#[derive(Debug, Clone)]
struct DiagnosticReport {
    /// Number stored as Least Significant Bit first! Challenge speaks in
    /// Most Significant first.
    bin: Vec<String>,
    bin_len: usize,
}

impl DiagnosticReport {
    pub fn zeros_ones(&self, position: usize) -> (usize, usize) {
        let mut ones = 0;
        let mut zeros = 0;

        for value in &self.bin {
            match value.chars().skip(position).next().unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => unreachable!(),
            }
        }

        (zeros, ones)
    }

    pub fn common(&self, position: usize) -> (usize, usize) {
        let (zeros, ones) = self.zeros_ones(position);

        if ones > zeros {
            (1, 0)
        } else {
            (0, 1)
        }
    }

    pub fn gamma_epsilon(&self) -> (usize, usize) {
        let mut gamma_str = String::new();
        let mut gamma = 0;
        let mut epsilon_str = String::new();
        let mut epsilon = 0;

        for index in 0..self.bin_len {
            let (common, uncommon) = self.common(index);
            let position = 2usize.pow(index as u32);

            if common == 0 {
                gamma_str.push('0');
            } else {
                gamma_str.push('1');
            }
            if uncommon == 0 {
                epsilon_str.push('0');
            } else {
                epsilon_str.push('1');
            }

            gamma += common * position;
            epsilon += uncommon * position;
        }

        println!("dbg: {} | {}", gamma_str, epsilon_str);

        (gamma, epsilon)
    }
}

impl FromStr for DiagnosticReport {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let len = raw.lines().next().unwrap().len();

        Ok(Self {
            /// Beware: From MSB (least on right) to LSB (least on left)
            bin: raw.lines().map(|s| s.chars().rev().collect()).collect(),
            bin_len: len,
        })
    }
}
