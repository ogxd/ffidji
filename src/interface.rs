use std::collections::HashMap;

pub fn enrich(interface: &mut Interface) {
    interface.types.push(Type { name: "int8".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "uint8".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "int16".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "uint16".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "int32".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "uint32".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "int64".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "uint64".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "float32".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
    interface.types.push(Type { name: "float64".to_string(), description: None, fields: Vec::new(), blittable: Some(true) });
}

pub fn get_types_map(map: &mut HashMap<String, Type>, interface: &Interface) {

    // Add custom types
    for r#type in &interface.types {
        if map.contains_key(&r#type.name) {
            panic!("Type '{}' already exists.", r#type.name);
        }
        map.insert(r#type.name.clone(), r#type.clone());
    }

    // Verify type fields
    for r#type in &interface.types {
        for field in &r#type.fields {
            if !map.contains_key(&field.r#type) {
                panic!("Type '{}' for {}.{} is undefined.", &field.r#type, r#type.name, &field.name);
            }
        }
    }

    // Verify method parameters & returns
    for methods in &interface.methods {
        for parameter in &methods.parameters {
            if !map.contains_key(&parameter.r#type) {
                panic!("Type '{}' for {}(..{}..) is undefined.", &parameter.r#type, methods.name, &parameter.name);
            }
        }
        for r#return in &methods.returns {
            if !map.contains_key(&r#return.r#type) {
                panic!("Type '{}' for {}(..) -> {} is undefined.", &r#return.r#type, methods.name, &r#return.name);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Interface {

    #[serde(rename = "namespace", default)]
    pub namespace: Option<String>,

    #[serde(rename = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "Type", default)]
    pub types: Vec<Type>,

    #[serde(rename = "Method", default)]
    pub methods: Vec<Method>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Type {

    #[serde(rename = "name", default)]
    pub name: String,
    
    #[serde(rename = "description", default)]
    pub description: Option<String>,

    #[serde(rename = "Field", default)]
    pub fields: Vec<Field>,

    #[serde(skip_serializing)]
    pub blittable: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Field {

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "description", default)]
    pub description: Option<String>,

    #[serde(rename = "type", default)]
    pub r#type: String,
}


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

#[derive(Debug, Deserialize, Clone)]
pub struct Parameter {

    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "description", default)]
    pub description: Option<String>,

    #[serde(rename = "type", default)]
    pub r#type: String,
}