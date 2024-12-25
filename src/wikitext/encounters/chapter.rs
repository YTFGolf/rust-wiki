//! Deals with singular chapters of encounters.

use super::section::EncountersSection;
use crate::data::stage::raw::stage_metadata::StageMeta;
use std::borrow::Cow;

#[derive(Debug)]
/// Represents a stage for encounters.
pub struct Stage<'a> {
    /// Name of stage.
    pub stage_name: &'a str,
    /// Enemy magnifications in stage.
    pub mags: String,
    /// Stage metadata.
    pub meta: &'a StageMeta,
}
impl<'a> Stage<'a> {
    /// Create new Stage.
    pub fn new(stage_name: &'a str, mags: String, meta: &'a StageMeta) -> Self {
        Self {
            stage_name,
            mags,
            meta,
        }
    }
}

#[derive(Debug)]
/// Represents a contiguous block of stages of the same display type with the
/// same map num.
pub struct Chapter<'a> {
    /// Name of chapter.
    pub chapter_name: Cow<'a, str>,

    /// Stages in chapter.
    pub stages: Vec<Stage<'a>>,
}
impl<'a> Chapter<'a> {
    /// Create new Chapter.
    pub fn new(chapter_name: Cow<'a, str>, stages: Vec<Stage<'a>>) -> Self {
        Self {
            chapter_name,
            stages,
        }
    }
}

#[derive(Debug)]
/// Represents a section of the encounters.
pub struct Group<'a> {
    /// Section
    pub section: &'a EncountersSection,
    /// Chapters under heading.
    pub chapters: Vec<Chapter<'a>>,
}
impl<'a> Group<'a> {
    /// Create new Group.
    pub fn new(section: &'a EncountersSection, chapters: Vec<Chapter<'a>>) -> Self {
        Self { section, chapters }
    }
}
