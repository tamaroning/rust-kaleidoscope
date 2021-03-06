const VERSION_STR: &'static str = "0.1.0";

pub fn show_version() {
    println!("Rust-Kaleidocope version {}", VERSION_STR);
}

pub fn show_usage() {
    println!("Usage: rust-kaleidoscope [options] <filepath>");
}