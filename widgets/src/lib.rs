#![recursion_limit = "256"]

pub use {gloo, gloo_console as console, js_sys, wasm_bindgen, wasm_dom as dom, web_sys, yew};

pub use self::button::*;
pub use self::card::*;
pub use self::checkbox::*;
pub use self::chip::*;
pub use self::data_table::*;
pub use self::dialog::*;
pub use self::drawer::*;
pub use self::fab::*;
pub use self::floating_label::*;
pub use self::icon_button::*;
pub use self::linear_progress::*;
pub use self::list::*;
pub use self::listeners::*;
pub use self::mdc::auto_init;
pub use self::menu::*;
pub use self::radio::*;
pub use self::snackbar::*;
pub use self::switch::*;
pub use self::tab::*;
pub use self::text_field::*;
pub use self::top_app_bar::*;
pub use self::widget::{MdcObject, MdcWidget};

pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod data_table;
pub mod dialog;
pub mod drawer;
pub mod fab;
pub mod floating_label;
pub mod icon_button;
pub mod line_ripple;
pub mod linear_progress;
pub mod list;
pub mod listeners;
pub mod menu;
pub mod notched_outline;
pub mod radio;
pub mod ripple;
pub mod snackbar;
pub mod switch;
pub mod tab;
pub mod text_field;
pub mod top_app_bar;
pub mod utils;
pub mod widget;

pub mod mdc {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = mdc, js_name = autoInit)]
        pub fn auto_init();
    }
}

pub const AUTO_INIT_ATTR: &str = "data-mdc-auto-init";

pub const MATERIAL_ICONS_CLASS: &str = "material-icons";
