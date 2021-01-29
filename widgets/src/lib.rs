#![recursion_limit = "256"]

pub use gloo;
pub use js_sys;
pub use wasm_bindgen;
pub use web_sys;
pub use yew;

pub use self::{
    bind::mdc::auto_init, button::*, card::*, checkbox::*, chip::*, data_table::*, drawer::*, fab::*, icon_button::*,
    list::*, listeners::*, menu::*, radio::*, switch::*, text_field::*, top_app_bar::*, widget::MdcWidget,
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
pub mod ripple;
pub mod switch;
pub mod text_field;
pub mod top_app_bar;
pub mod utils;
pub mod widget;
pub mod bind {
    pub mod mdc;
}

pub const AUTO_INIT_ATTR: &str = "data-mdc-auto-init";
