use crate::interface::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Method {

    #[serde(rename = "name", default)]
    pub name: String,
    
    #[serde(rename = "description", default)]
    pub description: Option<String>,

    #[serde(rename = "Parameter", default)]
    pub parameters: Vec<Parameter>,

    #[serde(rename = "Return", default)]
    pub returns: Vec<Parameter>,
}