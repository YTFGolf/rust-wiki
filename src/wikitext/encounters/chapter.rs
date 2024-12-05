//! Deals with singular chapters of encounters.

/// Represents a stage for encounters.
pub struct Stage<'a> {
    /// Name of stage.
    pub stage_name: &'a str,
    /// Enemy magnifications in stage.
    pub mags: &'a str,
}

/// Represents a contiguous block of stages of the same type with the same map
/// num.
pub struct Chapter<'a> {
    /// Name of chapter.
    pub chapter_name: &'a str,
    /// Stages in chapter.
    pub stages: &'a [Stage<'a>],
}
