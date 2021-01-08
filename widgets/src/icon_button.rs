use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct IconButton {
    html: Html,
    is_toggle: bool,
}

impl IconButton {
    pub fn simple() -> Self {
        Self {
            html: html! { <button class = "mdc-icon-button" data-mdc-ripple-is-unbounded = ""></button> },
            is_toggle: false,
        }
    }

    pub fn new() -> Self {
        let mut icon_button = Self::simple();
        icon_button.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCRipple");
        icon_button
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        let root = self.root_tag_mut();
        root.add_class("material-icons");
        root.add_child(name.into().into());
        self
    }

    pub fn item(self, item: impl Into<Html>) -> Self {
        self.add_item_with_class(item, "mdc-icon-button__icon")
    }

    pub fn toggle(self, icon_on: impl Into<String>, icon_off: impl Into<String>) -> Self {
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

    pub fn ripple(mut self, enabled: bool) -> Self {
        if !self.is_toggle {
            let root = self.root_tag_mut();
            if enabled {
                root.set_attr(AUTO_INIT_ATTR, "MDCRipple");
            } else {
                root.remove_attr(AUTO_INIT_ATTR);
            }
        }
        self
    }

    pub fn on(self) -> Self {
        self.class("mdc-icon-button--on")
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
        self.listener(Rc::new(onclick::Wrapper::new(callback)))
    }

    fn enable_toggle(&mut self) {
        if !self.is_toggle {
            let root = self.root_tag_mut();
            root.set_attr(AUTO_INIT_ATTR, "MDCIconButtonToggle");
            self.is_toggle = true;
        }
    }

    fn add_item_with_class(mut self, item: impl Into<Html>, class: &str) -> Self {
        let mut item = item.into();
        item.add_class(class);

        self.root_tag_mut().add_child(item);
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
    fn from(widget: IconButton) -> Self {
        widget.html
    }
}