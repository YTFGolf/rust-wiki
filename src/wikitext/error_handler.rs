//! Module for simplifying runtime errors.

/// Trait to simplify error handling.
pub trait InfallibleWrite {
/// Infallibly write a string to a buffer.
    fn infallible_write(self);
}

impl InfallibleWrite for std::fmt::Result {
    fn infallible_write(self) {
        self.expect("Writing to string failed");
    }
}
