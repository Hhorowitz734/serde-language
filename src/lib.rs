
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "Delimiter")]
    pub delim: String,

    #[serde(rename = "Subfields")]
    pub subfields: Option<Vec<Field>>, // Optional subfields
}


impl Field {

    pub fn pretty_print(&self, indent: usize) {
        /*
        * Pretty print fields as a tree. Maybe we want a package to do this in the future.
        * This is just a testing function, as it is performance intensive
        */

        let indentation = " ".repeat(indent);
        println!("{}- {} (Delimiter: \"{}\" | Identifier: \"{}\")", indentation, self.name, self.delim, self.identifier);

        // Recursive case
        if let Some(ref subfields) = self.subfields {
            for sf in subfields {
                sf.pretty_print(indent + 2); // Recursive call
            }
        }

        // The base case is just that it falls through here.

    }

}



#[derive(Debug)]
pub enum RecursiveParsingErrors {
    Io(std::io::Error)
    //Implement additional parsing errors here
}


