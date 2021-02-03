use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = mdc, js_name = autoInit)]
    pub fn auto_init();
}
