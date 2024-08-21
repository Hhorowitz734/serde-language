
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Delimiter")]
    pub delim: String,
    #[serde(rename = "Subfields")]
    pub subfields: Option<Vec<Field>>, // Optional subfields
}


#[derive(Debug)]
pub enum RecursiveParsingErrors {
    Io(std::io::Error)
    //Implement additional parsing errors here
}


