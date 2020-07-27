#![recursion_limit = "256"]

use std::borrow::Cow;

pub use self::{button::*, checkbox::*, data_table::*, text_field::*};

pub mod button;
pub mod checkbox;
pub mod data_table;
pub mod text_field;
pub mod utils;

type Text<'a> = Cow<'a, str>;
