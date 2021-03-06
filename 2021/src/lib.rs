use core::fmt;
use std::{
    error::Error,
    fs::read_to_string,
    io::{self},
    path::PathBuf,
    str::FromStr,
};

#[macro_export]
macro_rules! day_input {
    () => {
        input(from_source_file(file!())).unwrap()
    };
}

#[macro_export]
macro_rules! day_parse {
    () => {
        aoc2021::parse_input(aoc2021::from_source_file(file!())).unwrap()
    };
}

#[macro_export]
macro_rules! day_parse_lines {
    () => {
        aoc2021::parse_input_lines(aoc2021::from_source_file(file!())).unwrap()
    };
}

/// There are three unwraps here, tread lightly. Prefer to call one of the
/// macros in this crate rather than this function directly.
///
/// This function expects a path with a file stem of `day$num` where `$num` is
/// the day number.
///
/// # Panics
/// - If the provided path has no file stem
/// - If the file stem doesn't start `day`
/// - If the stem - the `day` prefix does not parse into a number
pub fn from_source_file(fname: &'static str) -> usize {
    PathBuf::from(fname)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .strip_prefix("day")
        .unwrap()
        .parse()
        .unwrap()
}

fn input_path(day: usize) -> Result<String, AdventError> {
    if day == 0 || day > 25 {
        Err(AdventError::InvalidDay { day })
    } else {
        Ok(format!("input/day{}", day))
    }
}

pub fn input(day: usize) -> Result<String, AdventError> {
    let path = input_path(day)?;
    read_to_string(&path).map_err(|e| AdventError::io(e, path))
}

pub fn parse_input<T: FromStr>(day: usize) -> Result<T, AdventError>
where
    <T as FromStr>::Err: fmt::Display,
{
    let path = input_path(day)?;
    let iput = input(day)?;

    iput.parse()
        .map_err(|e| AdventError::input_parse::<T, String>(e, path, None))
}

pub fn parse_input_lines<T: FromStr>(day: usize) -> Result<Vec<T>, AdventError>
where
    <T as FromStr>::Err: fmt::Display,
{
    let path = input_path(day)?;
    let iput = input(day)?;

    let mut parsed = vec![];
    for (ln, line) in iput.lines().enumerate() {
        parsed.push(
            line.parse()
                .map_err(|e| AdventError::input_parse::<T, &str>(e, path.as_str(), Some(ln)))?,
        )
    }

    Ok(parsed)
}

#[derive(Debug)]
pub enum AdventError {
    InvalidDay {
        day: usize,
    },
    IoError {
        inner: io::Error,
        file: PathBuf,
    },
    InputParseError {
        inner: String,
        file: PathBuf,
        line: Option<usize>,
    },
}

impl AdventError {
    pub fn io<P: Into<PathBuf>>(inner: io::Error, file: P) -> Self {
        Self::IoError {
            inner,
            file: file.into(),
        }
    }

    pub fn input_parse<T: FromStr, P: Into<PathBuf>>(
        inner: <T as FromStr>::Err,
        file: P,
        line: Option<usize>,
    ) -> Self
    where
        <T as FromStr>::Err: fmt::Display,
    {
        Self::InputParseError {
            inner: format!("{}", inner),
            file: file.into(),
            line,
        }
    }
}

impl Error for AdventError {}
impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdventError::InvalidDay { day } => write!(
                f,
                "Advent of Code runs for 25 days but you requested day {}",
                day
            ),
            AdventError::IoError { inner, file } => {
                write!(f, "Failed to read '{}': {}", file.to_string_lossy(), inner)
            }
            AdventError::InputParseError { inner, file, line } => {
                if let Some(ln) = line {
                    write!(
                        f,
                        "Failed parse for '{}' on line {}: {}",
                        file.to_string_lossy(),
                        ln,
                        inner
                    )
                } else {
                    write!(
                        f,
                        "Failed parse for '{}': {}",
                        file.to_string_lossy(),
                        inner
                    )
                }
            }
        }
    }
}

/// A struct to parse day two's input into.
pub struct MovementCommand {
    pub dir: MovementDirection,
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

pub enum MovementDirection {
    Forward,
    Down,
    Up,
}

impl FromStr for MovementDirection {
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
