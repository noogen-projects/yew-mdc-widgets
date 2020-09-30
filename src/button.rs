use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, services::ConsoleService, Callback, Html, MouseEvent};

use crate::{
    utils::{ripple_element, MdcWidget, VTagExt},
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
    auto_init: bool,
    ripple: bool,
    html: Html,
}

impl Button {
    pub fn new() -> Self {
        Self {
            auto_init: false,
            ripple: false,
            html: html! { <button class = "mdc-button"></button> },
        }
    }

    pub fn auto() -> Self {
        let mut button = Self::new();
        button.auto_init = true;
        button.ripple(true)
    }

    pub fn manual<'a>(id: impl Into<Text<'a>>) -> Self {
        Self::new().id(id).ripple(true)
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
        ripple_element(&mut self, "mdc-button__ripple", enabled);
        let auto_init = self.auto_init;
        let root = self.root_tag_mut();

        if root.attr(AUTO_INIT_ATTR).is_some() {
            root.remove_attr(AUTO_INIT_ATTR);
        }
        root.remove_child_tag("script");

        if enabled && auto_init {
            root.set_attr(AUTO_INIT_ATTR, "MDCRipple");
        } else if enabled {
            if let Some(id) = root.attributes.get("id") {
                root.children.push(html! {
                    <script>{ format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id) }</script>
                });
            } else {
                ConsoleService::error("Can not enable ripple for button without id");
                return self;
            }
        }
        self.ripple = enabled;
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

    fn auto_init(mut self, enabled: bool) -> Self {
        self.auto_init = enabled;
        let ripple = self.ripple;
        self.ripple(ripple)
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
