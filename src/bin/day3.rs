#![feature(drain_filter)]

use core::str::FromStr;
use std::cmp::Ordering;

use aoc2021::day_parse;

fn main() {
    let dr: DiagnosticReport = day_parse!();
    let (g, e) = dr.gamma_epsilon();

    println!("Gamma {}, Epsilon {}. Product {}", g, e, g * e);

    let (o2r, co2r) = dr.o2_co2();
    let o2 = msb_into_dec(&o2r);
    let co2 = msb_into_dec(&co2r);

    println!("o2: {}, co2: {}. Product {}", o2, co2, o2 * co2)
}

fn lsb_into_dec(bin: &[u8]) -> usize {
    let mut num = 0;
    for idx in 0..bin.len() {
        num += 2usize.pow(idx as u32) * bin[idx] as usize;
    }

    num
}

fn msb_into_dec(bin: &[u8]) -> usize {
    let bin: Vec<u8> = bin.iter().rev().map(|v| *v).collect();
    lsb_into_dec(&bin)
}

#[derive(Debug, Clone)]
struct DiagnosticReport {
    /// Number stored as Least Significant Bit first! Challenge speaks in
    /// Most Significant first.
    bin: Vec<Vec<u8>>,
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

    pub fn common(&self, position: usize) -> (u8, u8) {
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
            let index = self.bin_len - 1 - index;
            let (common, uncommon) = self.common(index);
            let position = 2usize.pow(index as u32);

            gamma += common as usize * position;
            epsilon += uncommon as usize * position;
        }

        (gamma, epsilon)
    }

    pub fn filter_common_uncommon(self, position: usize) -> (DiagnosticReport, DiagnosticReport) {
        let mut commons = vec![];
        let mut uncommons = vec![];

        let (common, uncommon) = self.common(position);
        for v in self.bin {
            if v[position] == common {
                commons.push(v);
            } else if v[position] == uncommon {
                uncommons.push(v);
            } else {
                panic!("WHY")
            }
        }

        (
            Self {
                bin: commons,
                bin_len: self.bin_len,
            },
            Self {
                bin: uncommons,
                bin_len: self.bin_len,
            },
        )
    }

    pub fn o2_co2(self) -> (Vec<u8>, Vec<u8>) {
        let (mut dr_o2, mut dr_co2) = self.filter_common_uncommon(0);
        let (mut o2, mut co2) = (None, None);
        for idx in 1..dr_o2.bin_len {
            if o2.is_none() {
                dr_o2 = dr_o2.filter_common_uncommon(idx).0;

                if dr_o2.bin.len() == 1 {
                    o2 = dr_o2.bin.get(0).map(|v| v.to_owned());
                }
            }

            if co2.is_none() {
                dr_co2 = dr_co2.filter_common_uncommon(idx).1;

                if dr_co2.bin.len() == 1 {
                    co2 = dr_co2.bin.get(0).map(|v| v.to_owned());
                }
            }
        }

        (o2.unwrap(), co2.unwrap())
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

    fn dr_test() -> DiagnosticReport {
        let test =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

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

        assert_eq!(demo.zeros_ones(0), (0, 4));
        assert_eq!(demo.zeros_ones(1), (1, 3));
        assert_eq!(demo.zeros_ones(2), (2, 2));
        assert_eq!(demo.zeros_ones(3), (3, 1));
    }

    #[test]
    fn diagnostic_report_has_wrong_common() {
        let demo = dr_demo();

        assert_eq!(demo.common(0), (1, 0));
        assert_eq!(demo.common(1), (1, 0));
        assert_eq!(demo.common(2), (1, 0));
        assert_eq!(demo.common(3), (0, 1));
    }

    #[test]
    fn diagnostic_report_test_has_wrong_common_0() {
        let test = dr_test();

        assert_eq!(test.common(0), (1, 0));
    }

    #[test]
    fn diagnostic_report_test_has_wrong_o2_co2() {
        let test = dr_test();
        let (test_o2, test_co2) = test.o2_co2();

        let mut o2 = vec![1, 0, 1, 1, 1];
        let o2_dec = 23;
        let o2_dec_test = msb_into_dec(&o2);

        assert_eq!(o2, test_o2);
        assert_eq!(o2_dec, o2_dec_test);
    }
}
