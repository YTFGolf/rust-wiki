//! Allows you to obtain pure data from, or transform into, usable formats.

// Note: this module is intended to just be fragile spaghetti code that only
// depends on itself and the ID types in super, and provides an interface that
// just works from the outside.

mod data;
pub mod parse;
pub mod transform;
mod types;

pub use data::{
    get_stage_type, iter_stage_types, MAX_VARIANT_INDEX, MAX_VARIANT_NUMBER, RAW_STAGE_TYPES,
};
