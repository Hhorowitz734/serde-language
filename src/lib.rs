use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    
    pub r#name: String,
    pub r#subfields: Vec<Field>

}

#[derive(Debug)]
pub enum RecursiveParsingErrors {
    Io(std::io::Error)
    //Implement additional parsing errors here
}


