#[derive(Debug, Deserialize)]

pub struct Interface {
    pub namespace: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "Method", default)]
    pub methods: Vec<Method>
}

#[derive(Debug, Deserialize)]
pub struct Method {
    pub name: String,
    
    pub description: Option<String>,

    #[serde(rename = "Parameter", default)]
    pub parameters: Vec<Parameter>,

    #[serde(rename = "Return", default)]
    pub returns: Vec<Parameter>
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,

    pub description: Option<String>,

    pub r#type: String
}