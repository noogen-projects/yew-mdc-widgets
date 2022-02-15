use std::ops::{Deref, DerefMut};

use const_format::concatcp;
use yew::{html, Callback, Html, MouseEvent};

use crate::{
    dom::{self, existing::JsObjectAccess, JsCast},
    utils::{ManageChildren, VTagExt},
    Element, MdcObject, MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    use crate::Element;
    use wasm_bindgen::prelude::*;

    pub const TYPE_NAME: &str = "MDCSnackbar";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCSnackbar)]
        pub type Snackbar;

        #[wasm_bindgen(constructor, js_class = MDCSnackbar, js_namespace = ["mdc", "snackbar"])]
        pub fn new(element: Element) -> Snackbar;

        /// Returns whether the snackbar is currently open.
        #[wasm_bindgen(method, getter)]
        pub fn is_open(this: &Snackbar) -> bool;

        /// Sets whether the snackbar is currently open.
        #[wasm_bindgen(method, setter)]
        pub fn set_is_open(this: &Snackbar, is_open: bool);

        /// Returns the automatic dismiss timeout in milliseconds
        #[wasm_bindgen(method, getter)]
        pub fn timeout_ms(this: &Snackbar) -> i32;

        /// Sets the automatic dismiss timeout in milliseconds
        #[wasm_bindgen(method, setter)]
        pub fn set_timeout_ms(this: &Snackbar, timeout_ms: i32);

        /// Returns whether the snackbar closes when it is focused and the user presses the ESC key.
        #[wasm_bindgen(method, getter)]
        pub fn close_on_escape(this: &Snackbar) -> bool;

        /// Sets whether the snackbar closes when it is focused and the user presses the ESC key.
        #[wasm_bindgen(method, setter)]
        pub fn set_close_on_escape(this: &Snackbar, close_on_escape: bool);

        /// Returns the textContent of the label element.
        #[wasm_bindgen(method, getter)]
        pub fn label_text(this: &Snackbar) -> String;

        /// Sets the textContent of the label element.
        #[wasm_bindgen(method, setter)]
        pub fn set_label_text(this: &Snackbar, label_text: &str);

        /// Returns the textContent of the action button element.
        #[wasm_bindgen(method, getter)]
        pub fn action_button_text(this: &Snackbar) -> String;

        /// Sets the textContent of the action button element.
        #[wasm_bindgen(method, setter)]
        pub fn set_action_button_text(this: &Snackbar, action_button_text: &str);

        #[wasm_bindgen(method)]
        pub fn open(this: &Snackbar);

        #[wasm_bindgen(method)]
        pub fn close(this: &Snackbar);

        #[wasm_bindgen(method)]
        pub fn layout(this: &Snackbar);
    }
}

#[derive(Debug, Clone)]
pub struct Snackbar {
    html: Html,
}

impl Default for Snackbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Snackbar {
    pub const VAR_NAME: &'static str = "snackbar";

    pub const CLASS: &'static str = "mdc-snackbar";

    /// Mandatory. Message text.
    pub const LABEL_CLASS: &'static str = "mdc-snackbar__label";

    /// Optional. Wraps the action button/icon elements, if present.
    pub const ACTIONS_CLASS: &'static str = "mdc-snackbar__actions";

    /// Optional. The action button.
    pub const ACTION_CLASS: &'static str = "mdc-snackbar__action";

    pub const SURFACE_CLASS: &'static str = "mdc-snackbar__surface";

    /// Optional. The dismiss ("X") icon.
    pub const DISMISS_CLASS: &'static str = "mdc-snackbar__dismiss";

    /// Optional. Positions the snackbar on the leading edge of the screen (left in LTR, right in RTL) instead of
    /// centered.
    pub const LEADING_CLASS: &'static str = "mdc-snackbar--leading";

    /// Optional. Positions the action button/icon below the label instead of alongside it.
    pub const STACKED_CLASS: &'static str = "mdc-snackbar--stacked";

    pub const OPEN_CLASS: &'static str = "mdc-snackbar--open";

    pub const DEMO_BUTTON_CLASS: &'static str = "snackbar-demo-button";

    pub fn new() -> Self {
        let mut snackbar = Self {
            html: html! {
                <aside class = { Self::CLASS }>
                    <div class = { Self::SURFACE_CLASS } role = "status">
                        <div class = { Self::ACTIONS_CLASS }>
                        </div>
                    </div>
                </aside>
            },
        };
        snackbar.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        snackbar
    }

    pub fn leading() -> Self {
        Self::new().class(Self::LEADING_CLASS)
    }

    pub fn stacked() -> Self {
        Self::new().class(Self::STACKED_CLASS)
    }

    pub fn mdc_object(id: impl AsRef<str>) -> mdc::Snackbar {
        dom::existing::get_element_by_id::<Element>(id.as_ref())
            .get(mdc::TYPE_NAME)
            .unchecked_into::<mdc::Snackbar>()
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        self.root_tag_mut()
            .find_child_contains_class_mut(Self::SURFACE_CLASS)
            .unwrap()
            .insert_child(0, html! {
                <div class = { Self::LABEL_CLASS }>{ label }</div>
            });
        self
    }

    pub fn action(mut self, action: impl Into<Html>) -> Self {
        let mut action = action.into();
        action.add_class(Self::ACTION_CLASS);

        self.root_tag_mut()
            .find_child_contains_class_recursively_mut(Self::ACTIONS_CLASS)
            .unwrap()
            .add_child(action);
        self
    }

    pub fn dismiss(mut self, action: impl Into<Html>) -> Self {
        let mut action = action.into();
        action.add_class(Self::DISMISS_CLASS);

        self.root_tag_mut()
            .find_child_contains_class_recursively_mut(Self::ACTIONS_CLASS)
            .unwrap()
            .add_child(action);
        self
    }

    pub fn opened(self) -> Self {
        self.class(Self::OPEN_CLASS)
    }

    /// Returns whether the snackbar is currently open.
    pub fn is_open(id: impl AsRef<str>) -> bool {
        Self::get_mdc_object(id).is_open()
    }

    /// Sets whether the snackbar is currently open.
    pub fn set_is_open(id: impl AsRef<str>, is_open: bool) {
        Self::get_mdc_object(id).set_is_open(is_open);
    }

    /// Returns the automatic dismiss timeout in milliseconds.
    /// Value must be between 4000 and 10000 (or -1 to disable the timeout completely) or an error will be thrown.
    /// Defaults to 5000 (5 seconds).
    pub fn timeout_ms(id: impl AsRef<str>) -> i32 {
        Self::get_mdc_object(id).timeout_ms()
    }

    /// Sets the automatic dismiss timeout in milliseconds.
    /// Value must be between 4000 and 10000 (or -1 to disable the timeout completely) or an error will be thrown.
    /// Defaults to 5000 (5 seconds).
    pub fn set_timeout_ms(id: impl AsRef<str>, timeout_ms: i32) {
        Self::get_mdc_object(id).set_timeout_ms(timeout_ms);
    }

    pub fn open_existing(id: impl AsRef<str>) {
        Self::get_mdc_object(id).open();
    }

    pub fn close_existing(id: impl AsRef<str>) {
        Self::get_mdc_object(id).close();
    }

    /// Indicates when the snackbar begins its opening animation.
    /// event.detail: `{}`
    pub fn on_opening(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":opening"), callback)
    }

    /// Indicates when the snackbar finishes its opening animation.
    /// event.detail: `{}`
    pub fn on_opened(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":opened"), callback)
    }

    /// Indicates when the snackbar begins its closing animation.
    /// reason contains the reason why the snackbar closed ('dismiss', 'action', or undefined).
    /// event.detail: `{reason?: string}`
    pub fn on_closing(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":closing"), callback)
    }

    /// Indicates when the snackbar finishes its closing animation.
    /// reason contains the reason why the snackbar closed ('dismiss', 'action', or undefined).
    /// event.detail: `{reason?: string}`
    pub fn on_closed(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":closed"), callback)
    }
}

impl MdcWidget for Snackbar {
    const NAME: &'static str = stringify!(Snackbar);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl MdcObject for Snackbar {
    const MDC_TYPE_NAME: &'static str = mdc::TYPE_NAME;
    type MdcType = mdc::Snackbar;
}

impl Deref for Snackbar {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Snackbar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Snackbar> for Html {
    fn from(widget: Snackbar) -> Self {
        widget.html
    }
}
