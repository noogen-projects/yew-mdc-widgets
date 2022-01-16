use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::MdcWidget;

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCNotchedOutline";
}
#[derive(Debug, Clone)]
pub struct NotchedOutline {
    html: Html,
}

impl NotchedOutline {
    pub const CLASS: &'static str = "mdc-notched-outline";
    pub const LEADING_CLASS: &'static str = "mdc-notched-outline__leading";
    pub const NOTCH_CLASS: &'static str = "mdc-notched-outline__notch";
    pub const TRAILING_CLASS: &'static str = "mdc-notched-outline__trailing";
    pub const NOTCHED_CLASS: &'static str = "mdc-notched-outline--notched";

    pub fn new() -> Self {
        Self {
            html: html! {
                <span class = { Self::CLASS }>
                    <span class = { Self::LEADING_CLASS }></span>
                    <span class = { Self::NOTCH_CLASS }></span>
                    <span class = { Self::TRAILING_CLASS }></span>
                </span>
            },
        }
    }
}

impl Default for NotchedOutline {
    fn default() -> Self {
        Self::new()
    }
}

impl MdcWidget for NotchedOutline {
    const NAME: &'static str = stringify!(FloatingLabel);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for NotchedOutline {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for NotchedOutline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<NotchedOutline> for Html {
    fn from(widget: NotchedOutline) -> Self {
        widget.html
    }
}
