//! Deals with singular chapters of encounters.

use super::section::SectionRef;
use crate::meta::stage::stage_id::StageID;
use std::{borrow::Cow, collections::HashSet};

#[derive(Debug)]
/// Represents a stage for encounters.
pub struct Stage<'a> {
    /// Name of stage.
    pub stage_name: &'a str,
    /// Enemy magnifications in stage.
    pub mags: String,
    /// Stage ID.
    pub id: StageID,
}
impl<'a> Stage<'a> {
    /// Create new Stage.
    pub fn new(stage_name: &'a str, mags: String, id: &'a StageID) -> Self {
        Self {
            stage_name,
            mags,
            id: id.clone(),
        }
    }

        /// Create new Stage.
        pub fn new2(stage_name: &'a str, mags: String, id:StageID) -> Self {
            Self {
                stage_name,
                mags,
                id: id.clone(),
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

    /// Return the chapter with all duplicate stage names removed.
    #[must_use]
    pub fn dedupped(self) -> Self {
        let mut names = HashSet::new();
        let mut dedupped = self;
        let s = dedupped
            .stages
            .into_iter()
            .filter(|s| names.insert(s.stage_name))
            .collect::<Vec<_>>();
        dedupped.stages = s;

        dedupped
    }
}

#[derive(Debug)]
/// Represents a section of the encounters.
pub struct Group<'a> {
    /// Section reference.
    pub sref: SectionRef,
    /// Chapters under heading.
    pub chapters: Vec<Chapter<'a>>,
}
impl<'a> Group<'a> {
    /// Create new Group.
    pub fn new(sref: SectionRef, chapters: Vec<Chapter<'a>>) -> Self {
        Self { sref, chapters }
    }
}
