use std::str::FromStr;

use camino::Utf8PathBuf;

pub trait ParseFile: FromStr
where
	<Self as FromStr>::Err: Into<RuntimeError>,
{
	fn parse_file<P: Into<Utf8PathBuf>>(path: P) -> Result<Self, RuntimeError> {
		let path = path.into();
		let content =
			std::fs::read_to_string(&path).map_err(|io| RuntimeError::FileIoError { path, io })?;

		match content.parse() {
			Ok(o) => Ok(o),
			Err(e) => Err(e.into()),
		}
	}
}

impl<T> ParseFile for T
where
	T: FromStr,
	<T as FromStr>::Err: Into<RuntimeError>,
{
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
	#[error("io error on path '{path}': {io}")]
	FileIoError {
		path: Utf8PathBuf,
		io: std::io::Error,
	},
}
