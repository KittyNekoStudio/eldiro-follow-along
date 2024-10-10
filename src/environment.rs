use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Enviroment {
    bindings: HashMap<String, Value>,
}

impl Enviroment {
    pub fn store_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }
}
