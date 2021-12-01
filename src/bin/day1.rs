use aoc2021::{parse_input_lines, Challenge};

fn main() {
    let values: Vec<usize> = parse_input_lines(Challenge::First, 1).unwrap();

    let mut previous = *values.first().unwrap();
    let mut increase = 0;
    for value in values.iter().skip(1) {
        if *value > previous {
            increase += 1
        }

        previous = *value;
    }

    println!("Depth increased: {} times", increase)
}
