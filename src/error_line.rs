use anyhow::anyhow;
use itertools::Itertools;
use std::{
    convert::TryFrom,
    path::PathBuf,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum Error {
    #[error("missing file_path")]
    MissingFilePath,

    #[error("invalid file_path: {0}")]
    InvalidFilePath(std::convert::Infallible),

    #[error("missing line")]
    MissingLine,

    #[error("invalid line: {0}")]
    InvalidLine(std::num::ParseIntError),

    #[error("missing column")]
    MissingColumn,

    #[error("invalid column: {0}")]
    InvalidColumn(std::num::ParseIntError),

    #[error("missing message")]
    MissingMessage,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct ErrorLine {
    pub(super) file_path: PathBuf,
    pub(super) line: usize,
    pub(super) column: usize,
    pub(super) message: String,
}

impl TryFrom<&str> for ErrorLine {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = input.to_owned();
        let mut split = input.split(':');

        let file_path = split
            .next()
            .ok_or(Error::MissingFilePath)?
            .parse::<PathBuf>()
            .map_err(Error::InvalidFilePath)?;

        let line = split
            .next()
            .ok_or(Error::MissingLine)?
            .parse::<usize>()
            .map_err(Error::InvalidLine)?;

        let column = split
            .next()
            .ok_or(Error::MissingLine)?
            .parse::<usize>()
            .map_err(Error::InvalidLine)?;

        let message = split.join(":").trim().to_string();

        Ok(Self {
            file_path,
            line,
            column,
            message,
        })
    }
}

impl From<ErrorLine> for format_serde_error::ErrorTypes {
    fn from(error_line: ErrorLine) -> Self {
        let error = anyhow!(error_line.message).into();

        Self::Custom {
            error,
            line: Some(error_line.line),
            column: Some(error_line.column),
        }
    }
}

pub(super) fn parse(input: &str) -> Result<Vec<ErrorLine>, Error> {
    input
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| ErrorLine::try_from(line))
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod test {
    use super::{
        parse,
        ErrorLine,
    };

    mod parse {
        use pretty_assertions::assert_eq;
        use std::path::PathBuf;

        #[test]
        fn example() -> Result<(), anyhow::Error> {
            let input = include_str!("../resources/example");

            let expected = vec![super::ErrorLine {
                file_path: PathBuf::from("./main.go"),
                line: 3,
                column: 1,
                message: "syntax error: non-declaration statement outside function body"
                    .to_string(),
            }];

            let got = super::parse(input)?;

            assert_eq!(expected, got);

            Ok(())
        }
    }

    mod print {
        use format_serde_error::SerdeError;
        use pretty_assertions::assert_eq;
        use std::path::PathBuf;

        #[test]
        fn example() -> Result<(), anyhow::Error> {
            let input = include_str!("../resources/example");
            let input_main = include_str!("../resources/main.go").to_string();

            let got = super::parse(input)?;

            for err in got {
                println!("{}", SerdeError::new(input_main.clone(), err));
            }

            Ok(())
        }
    }
}
