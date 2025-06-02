//! Utilities for dealing with numbers.

use num_format::{Locale, ToFormattedString, WriteFormatted};
use std::fmt::Write;

/// Get precision to use when formatting as float. Optimised for frames to use
/// as their seconds conversion.
/// ```
/// # use rust_wiki::wikitext::number_utils::get_precision_f;
/// assert_eq!(get_precision_f(30), 0);
/// assert_eq!(get_precision_f(3), 1);
/// assert_eq!(get_precision_f(1), 2);
/// ```
// potentially deprecate?
pub fn get_precision_f(frames: u32) -> usize {
    if frames % 30 == 0 {
        0
    } else if frames % 3 == 0 {
        1
    } else {
        2
    }
}

/// Get the plural of the word based on the amount.
/// ```
/// # use rust_wiki::wikitext::number_utils::plural;
/// assert_eq!(plural(1, "thing", "things"), "thing");
/// assert_eq!(plural(2, "thing", "things"), "things");
/// ```
pub fn plural<'a>(amt: u16, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 1 { single } else { plural }
}

/// Get the plural of the word based on the amount, using 30 as the singular
/// value instead of 1.
/// ```
/// # use rust_wiki::wikitext::number_utils::plural_f;
/// assert_eq!(plural_f(30, "thing", "things"), "thing");
/// assert_eq!(plural_f(60, "thing", "things"), "things");
/// ```
pub fn plural_f<'a>(amt: u32, single: &'a str, plural: &'a str) -> &'a str {
    if amt == 30 { single } else { plural }
}

/// Write time in seconds to buffer.
pub fn write_seconds(buf: &mut String, time_f: u32) {
    let time_s = f64::from(time_f) / 30.0;
    let precision = get_precision_f(time_f);
    // technically precision could just be 2 but might as well get the proper
    // precision.
    buf.write_str(&get_formatted_float(time_s, precision))
        .unwrap();
}

/// Get string representation of the time in seconds.
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

/// Get a formatted decimal representation of the number. Due to how rounding
/// works, a .5 value can give unexpected behaviour.
/// ```
/// # use rust_wiki::wikitext::number_utils::get_formatted_float;
/// assert_eq!(get_formatted_float(3.0,   2), "3");
/// assert_eq!(get_formatted_float(3.1,   2), "3.1");
/// assert_eq!(get_formatted_float(3.11,  2), "3.11");
/// assert_eq!(get_formatted_float(3.111, 2), "3.11");
/// assert_eq!(get_formatted_float(3.199, 2), "3.2");
/// assert_eq!(get_formatted_float(3.000001, 2), "3");
/// assert_eq!(get_formatted_float(1959.9999999999998, 2), "1,960");
/// // These might be unexpected
/// assert_eq!(get_formatted_float(3.195, 2), "3.19");
/// assert_eq!(get_formatted_float(3.185, 2), "3.19");
/// // This still works
/// assert_eq!(get_formatted_float(3.1951, 2), "3.2");
/// ```
pub fn get_formatted_float(num: f64, max_precision: usize) -> String {
    let mut buf = String::new();

    // e.g. for 3300.3300, will do `buf.write("3,300")` at the top, then below
    // will do `buf.write(".33")`

    let int_part = num.floor() as i64;
    buf.write_formatted(&int_part, &Locale::en).unwrap();

    let float_part = num.fract();
    if max_precision == 0 || float_part == 0.0 {
        return buf;
    }

    let formatted_float_untrimmed = format!("{float_part:.max_precision$}");
    let formatted_float = formatted_float_untrimmed.trim_matches('0');
    // need to remove both the 0 at the start (0.xxx -> .xxx) and any trailing
    // zeros (.xx0000 -> .xx); while doing those separately might semantically
    // make more sense it's a better idea to do it in one operation
    if formatted_float == "." {
        return buf;
    }

    write!(buf, "{formatted_float}").unwrap();

    buf
}
