//! Utilities related to CSV parsing.

use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
/// Error when trying to open and parse csv file.
pub enum FullCSVError {
    /// Error opening file given.
    #[error("Couldn't open file {file_name}. {source}")]
    FileOpenError {
        /// Name of file that couldn't be opened.
        file_name: String,
        /// What went wrong when trying to open.
        source: std::io::Error,
    },
    /// Error when parsing CSV.
    #[error("couldn't parse CSV")]
    ParseError(#[from] CSVParseError),
}

#[derive(Debug, thiserror::Error)]
/// Type of error encountered on certain line.
pub enum CSVParseErrorKind {
    /// CSV file doesn't have enough lines.
    #[error("tried to get a line that didn't exist")]
    NotEnoughLines,
    /// Error when converting to CSV Record.
    #[error("Couldn't convert to CSV Record. (TODO: fix this message) {0}")]
    CSVRecordError(csv::Error),
    /// Error when deserialising CSV Record.
    #[error(transparent)]
    DeserialiseError(csv::Error),
}

/// CSV parse error and line it occurred on.
pub type CSVParseErrorLine = (CSVParseErrorKind, usize);

#[derive(Debug)]
/// Error when parsing stage data CSV files.
pub struct CSVParseError {
    /// Kind of error encountered.
    pub kind: CSVParseErrorKind,
    /// File where error was encountered.
    pub file_name: String,
    /// Line of file where error was encountered.
    pub line: usize,
}
impl Display for CSVParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error on line {line} in file {file_name:?}: {kind}",
            line = self.line,
            file_name = self.file_name,
            kind = self.kind
        )
    }
}
impl std::error::Error for CSVParseError {}
