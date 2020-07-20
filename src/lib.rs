#![recursion_limit="256"]

use std::borrow::Cow;

pub mod button;
pub mod table;
pub mod text_field;

type Text<'a> = Cow<'a, str>;
