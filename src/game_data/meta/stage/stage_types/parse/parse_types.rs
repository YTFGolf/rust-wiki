//! Contains types used in the `parse` module.

#[derive(Debug, PartialEq, thiserror::Error)]
/// Error when parsing the stage type.
pub enum StageTypeParseError {
    /// Invalid "matcher" (variant code, e.g. `"main"`).
    #[error("unknown variant name")]
    UnknownMatcher,
    /// No map number provided when necessary.
    #[error("no map number supplied")]
    NoMapNumber,
    /// No stage number provided when necessary.
    #[error("no stage number supplied")]
    NoStageNumber,
    /// Map or stage number is invalid (e.g. negative, contains letters).
    #[error("contains invalid numbers")]
    InvalidNumber,
    /// Selector is not in a valid format for the given function (e.g. is a file
    /// name when the function is db refs).
    #[error("unrecognised selector format")]
    InvalidFormat,
}

// could perhaps give more context for UnkownMatcher and InvalidNumber
