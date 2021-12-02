use aoc2021::parse_input_lines;

fn main() {
    let values: Vec<usize> = parse_input_lines(1).unwrap();

    let p1 = part1(&values);
    let p2 = part2(&values);

    println!(
        "Depth increased {} times\nDenoised depth increased {} times",
        p1, p2
    )
}

pub fn part1(values: &[usize]) -> usize {
    let mut previous = *values.first().unwrap();
    let mut increase = 0;
    for value in values.iter().skip(1) {
        if *value > previous {
            increase += 1
        }

        previous = *value;
    }

    increase
}

pub fn part2(values: &[usize]) -> usize {
    let mut slidesum = SlidingSum::new(values.iter().take(3).map(|v| *v).collect());
    let mut slide_previous = slidesum.sum();
    let mut slide_increased = 0;

    for value in values.iter().skip(3) {
        slidesum.push(*value);
        let sum = slidesum.sum();

        if sum > slide_previous {
            slide_increased += 1;
        }

        slide_previous = sum;
    }

    slide_increased
}

// Tried to make this generic but it got too weird.
pub struct SlidingSum {
    values: Vec<usize>,
    idx: usize,
}

impl SlidingSum {
    pub fn new(initial: Vec<usize>) -> Self {
        Self {
            values: initial,
            idx: 0,
        }
    }

    pub fn push(&mut self, value: usize) {
        self.values[self.idx] = value;
        self.idx += 1;

        if self.idx == self.values.len() {
            self.idx = 0;
        }
    }

    pub fn sum(&self) -> usize {
        self.values.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_returns_incorrect_value() {
        let values: Vec<usize> = parse_input_lines(1).unwrap();

        assert_eq!(part1(&values), 1557)
    }

    #[test]
    fn part_two_returns_incorrect_value() {
        let values: Vec<usize> = parse_input_lines(1).unwrap();

        assert_eq!(part2(&values), 1608)
    }
}
