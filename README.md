# whitespace-conf

Rust library for parsing configuration files which are key-value pairs delimited by whitespace. Lines that start with `#` are ignored.

```rust
use std::fs;

fn main() {
    let string = fs::read_to_string("/etc/login.defs").unwrap();

    let defs = linux_login_defs::parse(&string);

    println!("UID_MIN = {:?}", defs.get("UID_MIN"));
    println!("UID_MAX = {:?}", defs.get("UID_MAX"));
}
```