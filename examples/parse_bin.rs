// examples/parse_bin.rs
// This should be written as unit tests instead

use serdelang::Field; 
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Paths to the input format (JSON schema) and binary file
    let input_format_path = "/Users/bhorowitz/Documents/serde-language/examples/sample-files/input-schemas/rowlf.json"; 
    let binary_file_path = "/Users/bhorowitz/Documents/serde-language/examples/sample-files/test-binaries/binary.bin"; 

    // Open and read the JSON file (input format)
    let mut input_format_file = File::open(input_format_path).expect("Unable to open input format file");
    let mut input_format_contents = String::new();
    input_format_file.read_to_string(&mut input_format_contents).expect("Unable to read input format file");

    // Deserialize the JSON content to a Field instance
    let field: Field = serde_json::from_str(&input_format_contents).expect("Unable to parse JSON input format");

    // Print the deserialized Field instance (optional)
    field.pretty_print(0);

    // Open the binary file
    let mut binary_file = File::open(binary_file_path).expect("Unable to open binary file");

    // Parse the binary file according to the schema
    field.parse_binary_file(&mut binary_file, 0)?;

    Ok(())
}
