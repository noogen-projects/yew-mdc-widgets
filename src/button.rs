use std::{
    rc::Rc, ops::{Deref, DerefMut},
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    Text,
    utils::{VTagExt, MdcWidget, ripple},
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
    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let button = Self {
            html: html! { <button id = id.into() class = "mdc-button"></button> },
        };
        button.ripple(true)
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
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
        ripple(&mut self, "mdc-button__ripple", enabled);
        if enabled {
            if !self.root_tag().is_last_child("script") {
                let button_tag = self.root_tag_mut();
                if let Some(id) = button_tag.attributes.get("id") {
                    button_tag.children.push(html! {
                        <script>{ format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id) }</script>
                    });
                }
            }
        } else {
            if self.root_tag().is_last_child("script") {
                let idx = self.root_tag_mut().children.len() - 1;
                self.root_tag_mut().children.remove(idx);
            }
        }
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.root_tag_mut().attributes.insert("disabled".to_string(), "".to_string());
        } else {
            self.root_tag_mut().attributes.remove("disabled");
        }
        self
    }

    pub fn add_before_label(mut self, item: impl Into<Html>) -> Self {
        let idx = self.root_tag().find_child_contains_class_idx("mdc-button__label")
            .unwrap_or_else(|| if self.root_tag().is_last_child("script") {
                self.root_tag().children.len() - 1
            } else {
                self.root_tag().children.len()
            });
        self.root_tag_mut().children.insert(idx, item.into());
        self
    }

    pub fn add_after_label(mut self, item: impl Into<Html>) -> Self {
        let idx = self.root_tag().find_child_contains_class_idx("mdc-button__label")
            .map(|idx| idx + 1)
            .unwrap_or_else(|| if self.root_tag().is_last_child("script") {
                self.root_tag().children.len() - 1
            } else {
                self.root_tag().children.len()
            });
        self.root_tag_mut().children.insert(idx, item.into());
        self
    }

    pub fn icon<'a>(self, icon: impl Into<Text<'a>>) -> Self {
        self.add_after_label(html! {
            <i class = "material-icons mdc-button__icon" aria-hidden = "true">{ icon.into() }</i>
        })
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        self.root_tag_mut().add_listener(Rc::new(onclick::Wrapper::new(callback)));
        self
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
    fn from(button: Button) -> Self {
        button.html
    }
}