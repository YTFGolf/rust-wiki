//! Get info about a cat.

use super::template_parameter::TemplateParameter;
use crate::config::Config;
use std::borrow::Cow;

struct Template {
    name: Cow<'static, str>,
    params: [TemplateParameter],
}

/// Do thing.
pub fn do_thing(wiki_id: u32, config: &Config) {
    println!("{wiki_id:?} {config:?}")
}
