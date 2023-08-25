use std::collections::HashMap;
use wasm_bindgen::JsValue;

#[derive(Default)]
pub struct MultiError {
    errors: HashMap<String, String>,
}

impl MultiError {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, section: &str, msg: String) {
        self.errors.insert(section.into(), msg);
    }
}

impl Into<JsValue> for MultiError {
    fn into(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.errors)
            .expect("map of strings to strings should be serializable")
    }
}

impl From<xword_puz::MultiError> for MultiError {
    fn from(value: xword_puz::MultiError) -> MultiError {
        MultiError { errors: value.into_error_map() }
    }
}
