#![recursion_limit = "256"]

pub use gloo;
pub use gloo_console as console;
pub use js_sys;
pub use wasm_bindgen;
pub use web_sys;
pub use yew;

pub use self::{
    button::*, card::*, checkbox::*, chip::*, data_table::*, dialog::*, drawer::*, fab::*, floating_label::*,
    icon_button::*, list::*, listeners::*, mdc::auto_init, menu::*, radio::*, switch::*, tab::*, text_field::*,
    top_app_bar::*, widget::MdcWidget,
};

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
pub mod list;
pub mod listeners;
pub mod menu;
pub mod notched_outline;
pub mod radio;
pub mod ripple;
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
