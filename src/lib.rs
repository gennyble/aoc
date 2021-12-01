use core::fmt;
use std::{
    error::Error,
    fs::read_to_string,
    io::{self},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Copy, Clone)]
pub enum Challenge {
    First,
    Second,
}

impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Challenge::First => write!(f, "first"),
            Challenge::Second => write!(f, "second"),
        }
    }
}

fn input_path(ch: Challenge, day: usize) -> Result<String, AdventError> {
    if day == 0 || day > 25 {
        Err(AdventError::InvalidDay { day })
    } else {
        Ok(format!("input/day{}_{}", day, ch))
    }
}

pub fn input(ch: Challenge, day: usize) -> Result<String, AdventError> {
    let path = input_path(ch, day)?;
    read_to_string(&path).map_err(|e| AdventError::io(e, path))
}

pub fn parse_input<T: FromStr>(ch: Challenge, day: usize) -> Result<T, AdventError>
where
    <T as FromStr>::Err: fmt::Display,
{
    let path = input_path(ch, day)?;
    let iput = input(ch, day)?;

    iput.parse()
        .map_err(|e| AdventError::input_parse::<T, String>(e, path, None))
}

pub fn parse_input_lines<T: FromStr>(ch: Challenge, day: usize) -> Result<Vec<T>, AdventError>
where
    <T as FromStr>::Err: fmt::Display,
{
    let path = input_path(ch, day)?;
    let iput = input(ch, day)?;

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
