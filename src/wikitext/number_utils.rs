use num_format::{Locale, ToFormattedString};
use std::fmt::Write;

/// Get precision to use when formatting as float.
///
/// Due to floats being silly this has a maximum precision of 1. Anything else,
/// do it manually.
/// ```
/// # use rust_wiki::wikitext::wiki_utils::get_float_precision;
/// assert_eq!(get_float_precision(1.0), 0);
/// assert_eq!(get_float_precision(1.1), 1);
/// ```
pub fn get_float_precision(num: f64) -> usize {
    if num % 1.0 == 0.0 { 0 } else { 1 }
}

/// Get precision to use when formatting as float. Optimised for frames to use
/// as their seconds conversion.
/// ```
/// # use rust_wiki::wikitext::wiki_utils::get_precision_f;
/// assert_eq!(get_precision_f(30), 0);
/// assert_eq!(get_precision_f(3), 1);
/// assert_eq!(get_precision_f(1), 2);
/// ```
pub fn get_precision_f(frames: u32) -> usize {
    if frames % 30 == 0 {
        0
    } else if frames % 3 == 0 {
        1
    } else {
        2
    }
}

pub fn plural<'a>(amt: u16, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 1 { single } else { plural }
}

pub fn plural_f<'a>(amt: u32, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 30 { single } else { plural }
}

pub fn write_seconds(buf: &mut String, time_f: u32) {
    // TODO refactor to `write_formatted_float`
    let time_s = f64::from(time_f) / 30.0;
    assert!(time_s < 1_000.0, "Amount of seconds is above 1,000!");
    let precision = get_precision_f(time_f);
    write!(buf, "{time_s:.precision$}").unwrap();
}

pub fn seconds_repr(time_f: u32) -> String {
    let mut buf = String::new();
    write_seconds(&mut buf, time_f);
    buf
}

/// `(frames, seconds)`
pub fn time_repr(time_f: u32) -> (String, String) {
    let f = time_f.to_formatted_string(&Locale::en);
    let s = seconds_repr(time_f);
    (f, s)
}

pub fn write_formatted_float(buf: &mut String, num: f64, precision: usize) {
    let int_part = num.floor() as i64;
    let formatted_int = int_part.to_formatted_string(&Locale::en);

    let float_part = num.fract();
    let formatted_float = format!("{float_part:.precision$}")
        .trim_start_matches('0')
        .to_string();

    write!(buf, "{formatted_int}{formatted_float}").unwrap();
}

pub fn get_formatted_float(num: f64, precision: usize) -> String {
    let mut buf = String::new();
    write_formatted_float(&mut buf, num, precision);
    buf
}
