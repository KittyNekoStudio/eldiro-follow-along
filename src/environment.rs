use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Enviroment {
    bindings: HashMap<String, Value>,
}

impl Enviroment {
    pub fn store_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }
    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Value, String> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or_else(|| format!("binding with name '{name}' does not exist"))
    }
}
