use crate::interface::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Type {

    #[serde(rename = "name", default)]
    pub name: String,
    
    #[serde(rename = "description", default)]
    pub description: Option<String>,

    #[serde(rename = "Field", default)]
    pub fields: Vec<Parameter>,

    #[serde(skip_deserializing)]
    pub base_type: bool,

    #[serde(skip_deserializing)]
    pub is_system: bool,
}