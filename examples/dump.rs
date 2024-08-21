// examples/dump.rs

extern crate serdelang;

use serdelang::Field;
use serde_json;


fn main() {
    // Create a sample Field instance
    let field = Field {
        r#name: String::from("root"),
        r#subfields: vec![
            Field {
                r#name: String::from("child1"),
                r#subfields: vec![],
            },
            Field {
                r#name: String::from("child2"),
                r#subfields: vec![],
            },
        ],
    };

    // Serialize the Field instance to JSON
    let serialized = serde_json::to_string(&field).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize the JSON back to a Field instance
    let deserialized: Field = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
