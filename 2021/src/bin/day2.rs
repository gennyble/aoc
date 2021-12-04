use aoc2021::{day_parse_lines, MovementCommand, MovementDirection};

fn main() {
    let cmds: Vec<MovementCommand> = day_parse_lines!();

    let (horizontal, depth) = part1(&cmds);

    println!(
        "Part One:\nDepth is {} and horizontal position {}. The product of those is {}",
        horizontal,
        depth,
        horizontal * depth
    );

    let (horizontal, depth, aim) = part2(&cmds);

    println!("Part Two:\nDepth is {}, horizontal {}, and aim {}. The product of the horizontal and depth is {}", depth, horizontal, aim, depth * horizontal)
}

fn part1(cmds: &[MovementCommand]) -> (isize, isize) {
    cmds.iter()
        .fold((0, 0), |(horizontal, depth), command| match command.dir {
            MovementDirection::Forward => (horizontal + command.units, depth),
            MovementDirection::Up => (horizontal, depth - command.units),
            MovementDirection::Down => (horizontal, depth + command.units),
        })
}

fn part2(cmds: &[MovementCommand]) -> (isize, isize, isize) {
    cmds.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), command| match command.dir {
            MovementDirection::Forward => {
                (horizontal + command.units, command.units * aim + depth, aim)
            }
            MovementDirection::Up => (horizontal, depth, aim - command.units),
            MovementDirection::Down => (horizontal, depth, aim + command.units),
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_returns_incorrect_value() {
        let cmds: Vec<MovementCommand> = day_parse_lines!();
        let tup = part1(&cmds);

        assert_eq!((1970, 916), tup)
    }

    #[test]
    fn part_two_returns_incorrect_value() {
        let cmds: Vec<MovementCommand> = day_parse_lines!();
        let tup = part2(&cmds);

        assert_eq!((1970, 1000556, 916), tup)
    }
}
