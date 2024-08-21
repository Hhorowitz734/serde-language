extern crate serdelang;

use serdelang::Field;
use serde_json;
use std::fs::File;
use std::io::Read;

fn main() {

    // Open the JSON file
    let mut file = File::open("/Users/bhorowitz/Documents/serde-language/examples/sample-files/input-schemas/csv.json").expect("Unable to open file");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    // Deserialize the JSON content to a Field instance
    let field: Field = serde_json::from_str(&contents).expect("Unable to parse JSON");

    // Print the deserialized Field instance
    println!("Deserialized (Debug format): {:#?}", field);



}
