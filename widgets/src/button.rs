use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    Text, AUTO_INIT_ATTR,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonStyle {
    Outlined,
    Raised,
    Unelevated,
}

impl ButtonStyle {
    pub fn classes() -> [&'static str; 3] {
        [
            ButtonStyle::Outlined.class(),
            ButtonStyle::Raised.class(),
            ButtonStyle::Unelevated.class(),
        ]
    }

    pub fn class(&self) -> &'static str {
        match self {
            ButtonStyle::Outlined => "mdc-button--outlined",
            ButtonStyle::Raised => "mdc-button--raised",
            ButtonStyle::Unelevated => "mdc-button--unelevated",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    html: Html,
}

impl Button {
    pub fn simple() -> Self {
        Self {
            html: html! { <button class = "mdc-button"><div class = "mdc-button__ripple"></div></button> },
        }
    }

    pub fn new() -> Self {
        let mut button = Self::simple();
        button.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCRipple");
        button
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        self.root_tag_mut().children.push(html! {
            <span class = "mdc-button__label">{ label }</span>
        });
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        if self.root_tag().is_contains_any_class(&ButtonStyle::classes()) {
            self.root_tag_mut().remove_any_class(&ButtonStyle::classes());
        }
        self.class(style.class())
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        let root = self.root_tag_mut();

        if enabled {
            root.set_attr(AUTO_INIT_ATTR, "MDCRipple");
        } else {
            root.remove_attr(AUTO_INIT_ATTR);
        }
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.root_tag_mut().set_attr("disabled", "");
        } else {
            self.root_tag_mut().remove_attr("disabled");
        }
        self
    }

    pub fn add_before_label(mut self, item: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let idx = root
            .find_child_contains_class_idx("mdc-button__label")
            .unwrap_or_else(|| {
                if root.is_last_child("script") {
                    root.children.len() - 1
                } else {
                    root.children.len()
                }
            });
        root.children.insert(idx, item.into());
        self
    }

    pub fn add_after_label(mut self, item: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let idx = root
            .find_child_contains_class_idx("mdc-button__label")
            .map(|idx| idx + 1)
            .unwrap_or_else(|| {
                if root.is_last_child("script") {
                    root.children.len() - 1
                } else {
                    root.children.len()
                }
            });
        root.children.insert(idx, item.into());
        self
    }

    pub fn icon<'a>(self, name: impl Into<Text<'a>>) -> Self {
        self.add_after_label(html! {
            <i class = "material-icons mdc-button__icon" aria-hidden = "true">{ name.into() }</i>
        })
    }

    pub fn on_click(self, callback: Callback<MouseEvent>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback)))
    }
}

impl MdcWidget for Button {
    const NAME: &'static str = "Button";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Button {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Button> for Html {
    fn from(widget: Button) -> Self {
        widget.html
    }
}
