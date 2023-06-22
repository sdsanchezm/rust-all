use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn saludar(name: &str) {
    alert(&format!("Hola, {}, how u doing?", name))
}