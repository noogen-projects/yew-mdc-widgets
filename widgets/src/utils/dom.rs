use js_sys::Reflect;
pub use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, Window};

#[track_caller]
pub fn window() -> Window {
    web_sys::window().expect("Can't get Window")
}

#[track_caller]
pub fn document() -> Document {
    window().document().expect("Can't get Document")
}

#[track_caller]
pub fn select_element<T: JsCast>(selector: &str) -> Option<T> {
    document()
        .query_selector(selector)
        .unwrap_or_else(|err| panic!("Can't select element by selector {:?}: {:?}", selector, err))
        .map(|element| element.dyn_into::<T>().expect("Can't cast to element"))
}

#[track_caller]
pub fn select_exist_element<T: JsCast>(selector: &str) -> T {
    select_element(selector).unwrap_or_else(|| panic!("Element not found by selector {:?}", selector))
}

pub trait JsObjectAccess {
    fn get(&self, property: impl Into<JsValue>) -> JsValue;
    fn set(&self, property: impl Into<JsValue>, value: impl Into<JsValue>) -> bool;
}

impl JsObjectAccess for JsValue {
    #[track_caller]
    fn get(&self, property: impl Into<JsValue>) -> JsValue {
        Reflect::get(self, &property.into()).expect("Target is not an Object")
    }

    #[track_caller]
    fn set(&self, property: impl Into<JsValue>, value: impl Into<JsValue>) -> bool {
        Reflect::set(self, &property.into(), &value.into()).expect("Target is not an Object")
    }
}
