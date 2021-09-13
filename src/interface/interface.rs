use crate::interface::*;
use std::collections::HashMap;

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

    #[serde(skip_deserializing)]
    types_map: Option<HashMap<String, Type>>,
}

impl Interface {

    pub fn initialize(&mut self)
    {
        // Initializes types map
        if self.types_map.is_none() {

            let mut new_types_map: HashMap<String, Type> = HashMap::new();

            // Add base types
            self.types.push(Type { name: "int8".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "uint8".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "int16".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "uint16".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "int32".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "uint32".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "int64".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "uint64".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "float32".to_string(), description: None, fields: Vec::new(), base_type: true });
            self.types.push(Type { name: "float64".to_string(), description: None, fields: Vec::new(), base_type: true });

            // Add custom types
            for r#type in &self.types {
                if new_types_map.contains_key(&r#type.name) {
                    panic!("Type '{}' already exists.", r#type.name);
                }
                new_types_map.insert(r#type.name.clone(), r#type.clone());
            }

            // Verify type fields
            for r#type in &self.types {
                for field in &r#type.fields {
                    if !new_types_map.contains_key(&field.r#type) {
                        panic!("Type '{}' for {}.{} is undefined.", &field.r#type, r#type.name, &field.name);
                    }
                }
            }

            // Verify method parameters & returns
            for methods in &self.methods {
                for parameter in &methods.parameters {
                    if !new_types_map.contains_key(&parameter.r#type) {
                        panic!("Type '{}' for {}(..{}..) is undefined.", &parameter.r#type, methods.name, &parameter.name);
                    }
                }
                for r#return in &methods.returns {
                    if !new_types_map.contains_key(&r#return.r#type) {
                        panic!("Type '{}' for {}(..) -> {} is undefined.", &r#return.r#type, methods.name, &r#return.name);
                    }
                }
            }

            self.types_map = Some(new_types_map);
        }
    }

    pub fn get_type(&self, type_name: &String) -> &Type {

        // Initializes types map
        if self.types_map.is_none()
        {
            panic!("interface must be initialized!");
        }

        let r#type = &self.types_map.as_ref().unwrap().get(type_name);
        assert!(r#type.is_some(), "missing type: {}", type_name);

        return r#type.unwrap();
    }

    pub fn is_type_blittable(&self, r#type: &Type) -> bool {
        if r#type.base_type {
            return true;
        }
        else {
            for field in &r#type.fields {
                if !self.is_param_blittable(field) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_param_blittable(&self, param: &Parameter) -> bool {
        if param.array.unwrap_or(false) {
            return false;
        }
        return self.is_type_blittable(self.get_type(&param.r#type));
    }
}