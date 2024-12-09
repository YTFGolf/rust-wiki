//! Deals with singular chapters of encounters.

use crate::data::stage::raw::stage_metadata::StageMeta;

/// Represents a stage for encounters.
pub struct Stage<'a> {
    /// Name of stage.
    pub stage_name: &'a str,
    /// Enemy magnifications in stage.
    pub mags: &'a str,
    /// Stage metadata.
    pub meta: &'a StageMeta,
}
impl<'a> Stage<'a> {
    /// Create new Stage.
    pub fn new(stage_name: &'a str, mags: &'a str, meta: &'a StageMeta) -> Self {
        Self {
            stage_name,
            mags,
            meta,
        }
    }
}

/// Represents a contiguous block of stages of the same display type with the
/// same map num.
pub struct Chapter<'a> {
    /// Name of chapter.
    pub chapter_name: &'a str,
    /// Stages in chapter.
    pub stages: &'a [Stage<'a>],
}
impl<'a> Chapter<'a> {
    /// Create new Chapter.
    pub fn new(chapter_name: &'a str, stages: &'a [Stage<'a>]) -> Self {
        Self {
            chapter_name,
            stages,
        }
    }
}

/// Block of chapters grouped under 1 heading.
pub struct Group<'a> {
    /// Heading.
    pub heading: &'a str,
    /// Chapters under heading.
    pub chapters: &'a [Chapter<'a>],
}
impl<'a> Group<'a> {
    /// Create new Group.
    pub fn new(heading: &'a str, chapters: &'a [Chapter<'a>]) -> Self {
        Self { heading, chapters }
    }
}
