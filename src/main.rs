#![feature(slice_patterns)]

mod imgavg;

use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let path = env::args().skip(1).next().expect("usage: cargo run -- <path>");
    let mut file = File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    println!("{}", imgavg::calculate_background(&buf));
}
