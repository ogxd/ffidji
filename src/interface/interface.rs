use crate::interface::*;
use crate::base::Writer;
use std::collections::{HashMap, HashSet};

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
            self.types.push(Type { name: "int8".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "uint8".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true});
            self.types.push(Type { name: "int16".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "uint16".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "int32".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "uint32".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "int64".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "uint64".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "float16".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "float32".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "float64".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });
            self.types.push(Type { name: "char16".to_string(), description: None, fields: Vec::new(), base_type: true, is_system: true });

            self.types.push(Type { name: "string".to_string(), description: None, fields: vec![ Parameter { name: String::from("utf8bytes"), r#type: String::from("uint8"), description: None, array: Some(true) } ], base_type: false, is_system: true });

            // Add custom types
            for r#type in &self.types {
                if new_types_map.insert(r#type.name.clone(), r#type.clone()).is_some() {
                    panic!("Type '{}' already exists.", r#type.name);
                }
            }

            self.types_map = Some(new_types_map);
        }
    }

    pub fn check_valid(&self)
    {
        match &self.types_map {
            None => panic!("Interface must be initialized first!"),
            Some(types_map) => {

                // Verify type fields
                for r#type in &self.types {
                    for field in &r#type.fields {
                        if !types_map.contains_key(&field.r#type) {
                            panic!("Type '{}' for {}.{} is undefined.", &field.r#type, r#type.name, &field.name);
                        }
                    }
                }

                let mut method_names: HashSet<String> = HashSet::new();

                // Verify method parameters & returns
                for method in &self.methods {
                    let mut parameter_names: HashSet<String> = HashSet::new();
                    
                    for parameter in &method.parameters {
                        if !types_map.contains_key(&parameter.r#type) {
                            panic!("Type '{}' for {}(..{}..) is undefined.", &parameter.r#type, method.name, &parameter.name);
                        }
                        // Ensures that method names are unique.
                        if !parameter_names.insert(parameter.name.clone()) {
                            panic!("A parameter with name '{}' already exists for method '{}'.", &parameter.name, &method.name);
                        }
                        if parameter.array.unwrap_or(false) {
                            panic!("Parameter '{}' for method {}(...) is an array, which is not supported at the moment. Please change the field type to a type with an array field instead.", &parameter.name, method.name);
                        }
                    }
                    for r#return in &method.returns {
                        if !types_map.contains_key(&r#return.r#type) {
                            panic!("Type '{}' for {}(..) -> {} is undefined.", &r#return.r#type, method.name, &r#return.name);
                        }
                        if r#return.array.unwrap_or(false) {
                            panic!("Return '{}' for method {}(...) is an array, which is not supported at the moment. Please change the field type to a type with an array field instead.", &r#return.name, method.name);
                        }
                    }

                    // Ensures that method names are unique.
                    if !method_names.insert(method.name.clone()) {
                        panic!("A method with name '{}' already exists.", &method.name);
                    }
                }
            }
        }
    }

    pub fn check_reserved(&self, writer: &dyn Writer) {
        match &self.types_map {
            None => panic!("Interface must be initialized first!"),
            Some(_) => {

                // Verify name in type and fields
                for r#type in &self.types {
                    if r#type.is_system {
                        continue;
                    }
                    if writer.is_name_reserved(&r#type.name) {
                        panic!("Type '{}''s name is reserved", &r#type.name);
                    }
                    for field in &r#type.fields {
                        if writer.is_name_reserved(&field.name) {
                            panic!("Type '{}''s field '{}''s name is reserved", &r#type.name, &field.name);
                        }
                    }
                }

                // Verify names in methods and parameters & returns
                for method in &self.methods {
                    if writer.is_name_reserved(&method.name) {
                        panic!("Method '{}''s name is reserved", &method.name);
                    }
                    for parameter in &method.parameters {
                        if writer.is_name_reserved(&parameter.name) {
                            panic!("Method '{}''s parameter '{}''s name is reserved", &method.name, &parameter.name);
                        }
                    }
                    for r#return in &method.returns {
                        if writer.is_name_reserved(&r#return.name) {
                            panic!("Method '{}''s return '{}''s name is reserved", &method.name, &r#return.name);
                        }
                    }
                }
            }
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
        } else {
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