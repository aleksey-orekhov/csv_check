

use std::fs::File;
use std::io::prelude::*;  // for `Read::read_to_string`
use std::io::Result;
use std::string::String;

fn main() {
    let file_contents = read_file();
    println!("{:?}", file_contents);

}


fn read_file() -> String {
    let filename = "/Users/cruser42/Coding/Rust/csv_check/Cargo.toml";
    let x = File::open(filename);
    let mut f = try!(x);
    let mut text = String::new();
    try!(f.read_to_string(&mut text));
    println!("{:?}", text);
    text
}
