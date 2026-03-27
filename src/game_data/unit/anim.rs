//! Deals with unit animation data.

use crate::game_data::version::Version;
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
/// Error when getting animation data.
pub enum AnimDataError {
    /// Specific form not found.
    AnimNotFound,
    /// Animation is found but has no frames.
    EmptyAnimation,
    /// Some weird error.
    ReadFileError(usize, std::io::Error),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Data about a single animation.
pub struct Anim {
    /// Length of the animation.
    pub length: u16, // right now all that's needed
}
impl Anim {
    /// Construct new [`Anim`] object.
    pub const fn new(length: u16) -> Self {
        Self { length }
    }

    /// Get length of unit's animations.
    pub const fn length(&self) -> u16 {
        // not called `len` to avoid setting off `clippy::len_without_is_empty`
        self.length
    }
}

/// Get data from a `.maanim` file.
pub fn get_maanim_data(path: &str, version: &Version) -> Result<Anim, AnimDataError> {
    use AnimDataError as E;
    let qualified = version.get_file_path("ImageDataLocal").join(path);

    let file = BufReader::new(File::open(&qualified).map_err(|_| E::AnimNotFound)?);

    let mut anim_lines = vec![];
    for (i, line) in file.lines().enumerate() {
        let line = line.map_err(|e| AnimDataError::ReadFileError(i, e))?;
        let split = line
            .split(',')
            .filter_map(|c| c.parse::<i32>().ok())
            // .ok will ignore the text parts but keep the rest
            // perhaps is better to do trimming and then check is numeric before
            // checking but that's effort tbh
            .collect::<Vec<_>>();
        anim_lines.push(split);
    }
    let anim_lines = anim_lines;

    /*
    lines will look like:

    9,12,4,0,0,下半身
    4
    0,0,1,0
    6,255,0,0
    7,0,1,0
    11,0,1,0
    10,12,4,0,0,下半身

    Roughly that's:
    - Control line
    - Amount of anim lines
    - Anim lines
    */

    /*
    Control line: idk, idk, (something to do with repeating), idk, idk
    Anim line: frame, idk, idk, idk
    */

    let mut max_frame = 0;
    for (i, line) in anim_lines.iter().enumerate() {
        const CONTROL_LINE_LEN: usize = 5;
        if line.len() < CONTROL_LINE_LEN {
            continue;
        }

        let following_lines_amt = (&anim_lines[i + 1])[0] as usize;
        if following_lines_amt == 0 {
            continue;
        }

        let last_anim_frame = &anim_lines[i + following_lines_amt + 1][0];
        let first_anim_frame = &anim_lines[i + 2][0];

        let duration = last_anim_frame - first_anim_frame;
        let repeats = max(line[2], 1);
        // make sure is at least 1, clamp would semantically mean more but
        // doesn't exist here...

        let last_frame_used = duration * repeats + first_anim_frame;
        max_frame = max(last_frame_used, max_frame);
    }

    if max_frame == 0 {
        return Err(AnimDataError::EmptyAnimation);
    }
    // can be assumed that 0f wouldn't appear naturally so must only appear if
    // the animation is empty

    Ok(Anim::new(max_frame as u16 + 1))
}
