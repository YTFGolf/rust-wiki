//! Defines information that can be used to obtain or transform pure data from
//! or into usable formats.

// Note: this module is intended to just be fragile spaghetti code that only
// depends on itself and the ID types in super, and provides an interface that
// just works from the outside.

mod data;
pub mod parse;
pub mod transform;
mod types;

pub use data::{get_stage_type, iter_stage_types};
