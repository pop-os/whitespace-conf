//! Parses configuration files which are key-value pairs delimited by whitespace.
//!
//! ### Note
//! Lines that start with `#` are ignored.
//!
//! ### Example
//! ```rust
//! use std::fs;
//!
//! fn main() {
//!     let string = fs::read_to_string("/etc/login.defs").unwrap();
//!
//!     let defs = linux_login_defs::parse(&string);
//!
//!     println!("UID_MIN = {:?}", defs.get("UID_MIN"));
//!     println!("UID_MAX = {:?}", defs.get("UID_MAX"));
//! }
//! ```

use std::collections::HashMap;

/// Parses configuration files which are key-value pairs delimited by whitespace.
pub fn parse<'a>(input: &'a str) -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if let Some(pos) = line.find(char::is_whitespace) {
            map.insert(&line[..pos], line[pos + 1..].trim_start());
        }
    }

    map
}
