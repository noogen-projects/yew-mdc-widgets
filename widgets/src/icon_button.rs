use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use const_format::concatcp;
use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    ripple,
    utils::{
        dom::{self, JsCast, JsObjectAccess},
        VTagExt,
    },
    CustomEvent, Element, MdcWidget, AUTO_INIT_ATTR, MATERIAL_ICONS_CLASS,
};

pub mod mdc {
    use wasm_bindgen::prelude::*;

    pub const TYPE_NAME: &str = "MDCIconButtonToggle";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCIconButtonToggle)]
        pub type IconButtonToggle;

        #[wasm_bindgen(method, getter)]
        pub fn on(this: &IconButtonToggle) -> bool;

        #[wasm_bindgen(method, setter)]
        pub fn set_on(this: &IconButtonToggle, is_on: bool);
    }
}

#[derive(Debug, Clone)]
pub struct IconButton {
    html: Html,
    is_toggle: bool,
}

impl Default for IconButton {
    fn default() -> Self {
        Self::new()
    }
}

impl IconButton {
    /// Defaults to an icon button
    pub const CLASS: &'static str = "mdc-icon-button";

    /// Indicates the element which shows the ripple styling.
    pub const RIPPLE_CLASS: &'static str = "mdc-icon-button__ripple";

    /// This class is applied to the root element and is used to indicate if the icon button toggle
    /// is in the "on" state.
    pub const ON_CLASS: &'static str = "mdc-icon-button--on";

    /// This class is applied to each icon element for the icon button toggle.
    pub const ICON_CLASS: &'static str = "mdc-icon-button__icon";

    /// This class is applied to a icon element and is used to indicate the toggle button icon that
    /// is represents the "on" icon.
    pub const ICON_ON_CLASS: &'static str = "mdc-icon-button__icon--on";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <button class = { Self::CLASS } data-mdc-ripple-is-unbounded = "">
                    <div class = { Self::RIPPLE_CLASS }></div>
                </button>
            },
            is_toggle: false,
        }
    }

    pub fn new() -> Self {
        Self::simple().attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME)
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        let root = self.root_tag_mut();
        root.add_class(MATERIAL_ICONS_CLASS);
        root.add_child(name.into().into());
        self
    }

    pub fn item(self, item: impl Into<Html>) -> Self {
        self.add_item_with_class(item, Self::ICON_CLASS)
    }

    pub fn toggle(self, icon_on: impl Into<String>, icon_off: impl Into<String>) -> Self {
        self.toggle_on(html! { <i class = { MATERIAL_ICONS_CLASS }>{ icon_on.into() }</i> })
            .toggle_off(html! { <i class = { MATERIAL_ICONS_CLASS }>{ icon_off.into() }</i> })
    }

    pub fn toggle_on(mut self, item: impl Into<Html>) -> Self {
        self.enable_toggle();
        self.add_item_with_class(item, concatcp!(IconButton::ICON_CLASS, " ", IconButton::ICON_ON_CLASS))
    }

    pub fn toggle_off(mut self, item: impl Into<Html>) -> Self {
        self.enable_toggle();
        self.add_item_with_class(item, Self::ICON_CLASS)
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if !self.is_toggle {
            let root = self.root_tag_mut();
            if enabled {
                root.set_attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME);
            } else {
                root.remove_attr(AUTO_INIT_ATTR);
            }
        }
        self
    }

    pub fn on(self) -> Self {
        self.class(Self::ON_CLASS)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.root_tag_mut().set_attr("disabled", "");
        } else {
            self.root_tag_mut().remove_attr("disabled");
        }
        self
    }

    pub fn set_on_by_id(id: impl AsRef<str>, is_on: bool) {
        let toggle_button = dom::get_exist_element_by_id::<Element>(id.as_ref())
            .get(mdc::TYPE_NAME)
            .unchecked_into::<mdc::IconButtonToggle>();
        toggle_button.set_on(is_on);
    }

    /// Emits when the icon is toggled
    /// event.detail: `{"isOn": boolean}`
    pub fn on_change(self, callback: impl Into<Callback<CustomEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":change"), callback)
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }

    fn enable_toggle(&mut self) {
        if !self.is_toggle {
            let root = self.root_tag_mut();
            root.set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
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
    const NAME: &'static str = stringify!(IconButton);

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
