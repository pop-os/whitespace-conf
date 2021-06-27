use std::fs;

fn main() {
    let string = fs::read_to_string("/etc/login.defs").unwrap();

    let defs = whitespace_conf::parse(&string);

    println!("UID_MIN = {:?}", defs.get("UID_MIN"));
    println!("UID_MAX = {:?}", defs.get("UID_MAX"));
}
