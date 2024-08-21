
#[derive(Serialize, Deserialize)]
pub struct Field {
    
    r#name: String,
    r#subfields: Vec<Field>

}
