use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    Text,
};

#[derive(Debug, Clone)]
pub struct IconButton {
    html: Html,
    is_toggle: bool,
}

impl IconButton {
    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let button = Self {
            html: html! { <button id = id.into() class = "mdc-icon-button"></button> },
            is_toggle: false,
        };
        button.ripple(true)
    }

    pub fn icon<'a>(mut self, name: impl Into<Text<'a>>) -> Self {
        let root = self.root_tag_mut();
        root.add_class("material-icons");
        root.add_child(html! { { name.into() } });
        self
    }

    pub fn item(self, item: impl Into<Html>) -> Self {
        self.add_item_with_class(item, "mdc-icon-button__icon")
    }

    pub fn toggle<'a, 'b>(self, icon_on: impl Into<Text<'a>>, icon_off: impl Into<Text<'b>>) -> Self {
        self.toggle_on(html! { <i class = "material-icons">{ icon_on.into() }</i> })
            .toggle_off(html! { <i class = "material-icons">{ icon_off.into() }</i> })
    }

    pub fn toggle_on(mut self, item: impl Into<Html>) -> Self {
        self.enable_toggle();
        self.add_item_with_class(item, "mdc-icon-button__icon mdc-icon-button__icon--on")
    }

    pub fn toggle_off(mut self, item: impl Into<Html>) -> Self {
        self.enable_toggle();
        self.add_item_with_class(item, "mdc-icon-button__icon")
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if !self.is_toggle {
            let root = self.root_tag_mut();
            if enabled {
                if !root.is_last_child("script") {
                    if let Some(id) = root.attributes.get("id") {
                        root.children.push(html! {
                        <script>{ format!("(new mdc.ripple.MDCRipple(document.getElementById('{}'))).unbounded = true", id) }</script>
                    });
                    }
                }
            } else {
                root.remove_child_tag("script");
            }
        }
        self
    }

    pub fn on(mut self) -> Self {
        let root = self.root_tag_mut();
        if !root.is_contains_class("mdc-icon-button--on") {
            root.add_class("mdc-icon-button--on");
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

    pub fn on_click(self, callback: Callback<MouseEvent>) -> Self {
        self.add_listener(Rc::new(onclick::Wrapper::new(callback)))
    }

    fn enable_toggle(&mut self) {
        if !self.is_toggle {
            let root = self.root_tag_mut();
            root.remove_child_tag("script");
            if let Some(id) = root.attributes.get("id") {
                root.children.push(html! {
                    <script>{ format!("mdc.iconButton.MDCIconButtonToggle.attachTo(document.getElementById('{}'))", id) }</script>
                });
            }
            self.is_toggle = true;
        }
    }

    fn add_item_with_class(mut self, item: impl Into<Html>, class: &str) -> Self {
        let mut item = item.into();
        item.add_class(class);

        let root = self.root_tag_mut();
        let idx = root.find_child_tag_idx("script").unwrap_or_else(|| root.children.len());
        root.children.insert(idx, item);

        self
    }
}

impl MdcWidget for IconButton {
    const NAME: &'static str = "IconButton";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for IconButton {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for IconButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<IconButton> for Html {
    fn from(icon_button: IconButton) -> Self {
        icon_button.html
    }
}
