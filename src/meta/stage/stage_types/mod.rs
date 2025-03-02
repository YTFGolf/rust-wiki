//! Defines information that can be used to obtain or transform pure data from
//! or into usable formats.

#![allow(dead_code)]
// Note: this module is intended to just be fragile spaghetti code that only
// depends on itself and the ID types in super, and provides an interface that
// just works from the outside.

pub(self) mod data;
pub(self) mod types;
