use std::str::FromStr;

use aoc2021::day_parse_lines;

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
            Direction::Forward => (horizontal + command.units, depth),
            Direction::Up => (horizontal, depth - command.units),
            Direction::Down => (horizontal, depth + command.units),
        })
}

fn part2(cmds: &[MovementCommand]) -> (isize, isize, isize) {
    cmds.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), command| match command.dir {
            Direction::Forward => (horizontal + command.units, command.units * aim + depth, aim),
            Direction::Up => (horizontal, depth, aim - command.units),
            Direction::Down => (horizontal, depth, aim + command.units),
        },
    )
}

struct MovementCommand {
    pub dir: Direction,
    pub units: isize,
}

impl FromStr for MovementCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((direction, units)) => {
                let dir = direction.parse()?;
                let units = units
                    .parse()
                    .map_err(|e| format!("Faield to parse '{}' as units: {}", units, e))?;

                Ok(Self { dir, units })
            }
            None => Err(format!("String '{}' does not contain a space", s)),
        }
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(format!("'{}' is not a valid direction", s)),
        }
    }
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
