use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = mdc, js_name = autoInit)]
    pub fn auto_init();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = MDCDialog)]
    pub type Dialog;

    #[wasm_bindgen(method)]
    pub fn open(this: &Dialog);

    #[wasm_bindgen(method)]
    pub fn close(this: &Dialog);

    #[wasm_bindgen(method)]
    pub fn layout(this: &Dialog);
}
