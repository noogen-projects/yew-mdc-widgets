use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{utils::VTagExt, MdcWidget};

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCFloatingLabel";
}

#[derive(Debug, Clone)]
pub struct FloatingLabel {
    html: Html,
}

impl FloatingLabel {
    pub const CLASS: &'static str = "mdc-floating-label";
    pub const FLOAT_ABOVE_CLASS: &'static str = "mdc-floating-label--float-above";
    pub const SHAKE_CLASS: &'static str = "mdc-floating-label--shake";
    pub const REQUIRED_CLASS: &'static str = "mdc-floating-label--required";

    pub fn new(label: impl Into<Html>) -> Self {
        Self {
            html: html! {
                <span class = Self::CLASS>{ label }</span>
            },
        }
    }

    pub fn get_id(&self) -> Option<String> {
        self.root_tag().attr("id").map(ToString::to_string)
    }
}

impl MdcWidget for FloatingLabel {
    const NAME: &'static str = stringify!(FloatingLabel);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for FloatingLabel {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for FloatingLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<FloatingLabel> for Html {
    fn from(widget: FloatingLabel) -> Self {
        widget.html
    }
}
