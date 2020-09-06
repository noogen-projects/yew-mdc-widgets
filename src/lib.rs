#![recursion_limit = "256"]

use std::borrow::Cow;

pub use self::{
    button::*, checkbox::*, data_table::*, drawer::*, icon_button::*, list::*, menu::*, radio::*, text_field::*,
    top_app_bar::*, utils::MdcWidget,
};

pub mod button;
pub mod checkbox;
pub mod data_table;
pub mod drawer;
pub mod icon_button;
pub mod list;
pub mod menu;
pub mod radio;
pub mod text_field;
pub mod top_app_bar;
pub mod utils;

type Text<'a> = Cow<'a, str>;
