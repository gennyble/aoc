#![feature(drain_filter)]

use core::str::FromStr;
use std::cmp::Ordering;

use aoc2021::day_parse;

fn main() {
    let mut dr: DiagnosticReport = day_parse!();
    let (g, e) = dr.gamma_epsilon();

    println!("Gamma {}, Epsilon {}. Product {}", g, e, g * e);

    let (o2, co2) = dr.o2_co2_rating();

    println!("O2 Rating {}, Co2 Rating {}. Product {}", o2, co2, o2 * co2)
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

    fn o2_co2_rating_raw(&mut self) -> (String, String) {
        let mut co2_report = self.filter_oxygen(0);

        // Filter for o2 in the current report
        for idx in 1..self.bin_len {
            if self.bin.len() == 1 {
                println!("Found o2");
                break;
            }

            self.filter_oxygen(idx);
        }

        // Filter for co2 using position = 0 report and it's relevant throwaways
        for idx in 1..self.bin_len {
            if co2_report.bin.len() == 1 {
                println!("Found co2");
                break;
            }

            co2_report = co2_report.filter_oxygen(idx);
        }

        println!(
            "{} | {}",
            u32::from_str_radix(
                &self.bin.get(0).unwrap().chars().rev().collect::<String>(),
                2
            )
            .unwrap(),
            u32::from_str_radix(
                &co2_report
                    .bin
                    .get(0)
                    .unwrap()
                    .chars()
                    .rev()
                    .collect::<String>(),
                2
            )
            .unwrap()
        );

        (
            self.bin.get(0).unwrap().to_owned(),
            co2_report.bin.get(0).unwrap().to_owned(),
        )
    }

    pub fn o2_co2_rating(&mut self) -> (usize, usize) {
        let (o2_raw, co2_raw) = self.o2_co2_rating_raw();

        let mut o2 = 0;
        let mut co2 = 0;
        for (idx, (o2_ch, co2_ch)) in o2_raw.chars().zip(co2_raw.chars()).enumerate() {
            let mult = 2usize.pow(idx as u32);

            if o2_ch == '1' {
                o2 += mult
            }

            if co2_ch == '1' {
                co2 += mult
            }
        }

        (o2, co2)
    }

    /// Filter this diagnostic report for the Oxygen Generator Rating at the
    /// provided position.
    ///
    /// **The removed elements are returned as another Diagnostic Report**.
    /// The removed elements make up the correct set for the Co2 Scrubber Rating.
    pub fn filter_oxygen(&mut self, position: usize) -> DiagnosticReport {
        let (zeros, ones) = self.zeros_ones(position);
        let common = match zeros.cmp(&ones) {
            Ordering::Equal => '1',
            Ordering::Greater => '0',
            Ordering::Less => '1',
        };

        DiagnosticReport {
            bin: self
                .bin
                .drain_filter(|s| s.chars().skip(position).next().unwrap() != common)
                .collect(),
            bin_len: self.bin_len,
        }
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
