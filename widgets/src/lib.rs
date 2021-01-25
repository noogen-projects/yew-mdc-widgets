#![recursion_limit = "256"]

pub use gloo;
pub use js_sys;
pub use wasm_bindgen;
pub use web_sys;
pub use yew;

pub use self::{
    button::*, card::*, checkbox::*, chip::*, data_table::*, drawer::*, fab::*, icon_button::*, list::*, listeners::*,
    menu::*, radio::*, switch::*, text_field::*, top_app_bar::*, utils::MdcWidget,
};

pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod data_table;
pub mod dialog;
pub mod drawer;
pub mod fab;
pub mod icon_button;
pub mod list;
pub mod listeners;
pub mod menu;
pub mod radio;
pub mod switch;
pub mod text_field;
pub mod top_app_bar;
pub mod utils;

const AUTO_INIT_ATTR: &str = "data-mdc-auto-init";

pub fn auto_init() {
    js_sys::eval("window.mdc.autoInit()").expect("JavaScript evaluation error");
}
