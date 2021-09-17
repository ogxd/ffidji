#[derive(Debug, Deserialize, Clone)]
pub struct Parameter {

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "description", default)]
    pub description: Option<String>,

    #[serde(rename = "type", default)]
    pub r#type: String,

    #[serde(rename = "array", default)]
    pub array: Option<bool>,
}