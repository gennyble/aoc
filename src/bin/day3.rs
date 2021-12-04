#![feature(drain_filter)]

use core::str::FromStr;
use std::cmp::Ordering;

use aoc2021::day_parse;

fn main() {
    let mut dr: DiagnosticReport = day_parse!();
    let (g, e) = dr.gamma_epsilon();

    println!("Gamma {}, Epsilon {}. Product {}", g, e, g * e);
}

#[derive(Debug, Clone)]
struct DiagnosticReport {
    /// Number stored as Least Significant Bit first! Challenge speaks in
    /// Most Significant first.
    bin: Vec<Vec<usize>>,
    bin_len: usize,
}

impl DiagnosticReport {
    pub fn zeros_ones(&self, position: usize) -> (usize, usize) {
        let mut ones = 0;
        let mut zeros = 0;

        for value in &self.bin {
            match value[position] {
                0 => zeros += 1,
                1 => ones += 1,
                _ => unreachable!(),
            }
        }

        (zeros, ones)
    }

    pub fn common(&self, position: usize) -> (usize, usize) {
        let (zeros, ones) = self.zeros_ones(position);

        match zeros.cmp(&ones) {
            Ordering::Greater => (0, 1),
            Ordering::Less => (1, 0),
            Ordering::Equal => (1, 0),
        }
    }

    pub fn gamma_epsilon(&self) -> (usize, usize) {
        let mut gamma = 0;
        let mut epsilon = 0;

        for index in 0..self.bin_len {
            let (common, uncommon) = self.common(index);
            let position = 2usize.pow(index as u32);

            gamma += common * position;
            epsilon += uncommon * position;
        }

        (gamma, epsilon)
    }
}

impl FromStr for DiagnosticReport {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let len = raw.lines().next().unwrap().len();

        Ok(Self {
            /// Beware: From MSB (least on right) to LSB (least on left)
            bin: raw
                .lines()
                .map(|s| {
                    s.chars()
                        .rev()
                        .map(|c| {
                            if c == '0' {
                                0
                            } else if c == '1' {
                                1
                            } else {
                                panic!()
                            }
                        })
                        .collect()
                })
                .collect(),
            bin_len: len,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn dr_demo() -> DiagnosticReport {
        #[rustfmt::skip]
        let test = "1000\n1100\n1110\n1111";

        DiagnosticReport::from_str(test).unwrap()
    }

    #[test]
    fn gamma_epsilon_are_wrong() {
        let dr: DiagnosticReport = day_parse!();
        let (g, e) = dr.gamma_epsilon();

        assert_eq!(g, 2987);
        assert_eq!(e, 1108);
        assert_eq!(g * e, 3309596);
    }

    #[test]
    fn diagnostic_report_has_wrong_zeros_ones_count() {
        let demo = dr_demo();

        assert_eq!(demo.zeros_ones(3), (0, 4));
        assert_eq!(demo.zeros_ones(2), (1, 3));
        assert_eq!(demo.zeros_ones(1), (2, 2));
        assert_eq!(demo.zeros_ones(0), (3, 1));
    }

    #[test]
    fn diagnostic_report_has_wrong_common() {
        let demo = dr_demo();

        assert_eq!(demo.common(3), (1, 0));
        assert_eq!(demo.common(2), (1, 0));
        assert_eq!(demo.common(1), (1, 0));
        assert_eq!(demo.common(0), (0, 1));
    }
}
