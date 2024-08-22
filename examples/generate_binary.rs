// examples/generate_binary.rs

/*
* This file generates binaries for testing the parser
*/

use std::fs::File;
use std::io::Write;
use std::io::Result;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).expect("Invalid hex digit"))
        .collect()
}

fn main() -> Result<()> {
    let file_path = "/Users/bhorowitz/Documents/serde-language/examples/sample-files/test-binaries/binary.bin";
    let mut file = File::create(file_path)?;

    let hex_string = "EB 90 10 05 10 10";
    let bytes = hex_to_bytes(hex_string);

    file.write_all(&bytes)?;

    println!("Binary file '{}' has been generated.", file_path);

    Ok(())
}
