
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub r#Name: String,
    pub r#Delimiter: String,
    pub r#subfields: Option<Vec<Field>>, // Optional subfields
}


#[derive(Debug)]
pub enum RecursiveParsingErrors {
    Io(std::io::Error)
    //Implement additional parsing errors here
}


