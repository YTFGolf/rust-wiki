//! Module for simplifying runtime errors.

/// Trait to simplify error handling.
pub trait InfallibleWrite {
    /// Infallibly write a string to a buffer.
    #[track_caller]
    fn infallible_write(self);
}

impl InfallibleWrite for std::fmt::Result {
    fn infallible_write(self) {
        self.expect("Writing to string failed");
    }
}

impl InfallibleWrite for Result<usize, std::io::Error> {
    fn infallible_write(self) {
        self.expect("Writing to string failed");
    }
}
