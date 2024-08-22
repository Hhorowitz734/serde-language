
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

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

    pub fn parse_binary_file(&self, file: &mut File, depth: usize) -> std::io::Result<()> {

        let mut buffer = [0; 2];

        file.read_exact(&mut buffer)?;

        // Format bytes as a string (in the future, this should just be a byte comparison)
        let identifier = format!("{:02X}{:02X}", buffer[0], buffer[1]);

        if identifier == self.identifier {
            println!("{:indent$}- Found: {} (Identifier: {})", "", self.name, self.identifier, indent = depth * 2);
            
            if let Some(ref subfields) = self.subfields {
                
                for subfield in subfields {
                    subfield.parse_binary_file(file, depth + 1)?; //Recursive call
                }
            }
        } else {
            // Here, we should just continue if we don't find the right message. It needs to be more optional
            println!("{:indent$}- Expected: {} (Identifier: {}), but found: {}", "", self.name, self.identifier, identifier, indent = depth * 2);
        
        }

        Ok(())
    }

}




#[derive(Debug)]
pub enum RecursiveParsingErrors {
    Io(std::io::Error)
    //Implement additional parsing errors here
}


