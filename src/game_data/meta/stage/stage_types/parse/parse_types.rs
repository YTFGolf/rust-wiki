//! Contains types used in the `parse` module.

#[derive(Debug, PartialEq)]
/// Error when parsing the stage type.
pub enum StageTypeParseError {
    /// Invalid "matcher" (variant code, e.g. `"main"`).
    UnknownMatcher,
    /// No map number provided when necessary.
    NoMapNumber,
    /// No stage number provided when necessary.
    NoStageNumber,
    /// Map or stage number is invalid (e.g. negative, contains letters).
    InvalidNumber,
    /// Selector is not in a valid format for the given function (e.g. is a file
    /// name when the function is db refs).
    InvalidFormat,
}
