use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use const_format::concatcp;
use yew::{html, html::onclick, virtual_dom::VTag, Callback, Html, MouseEvent};

use crate::{
    utils::{
        dom::{self, JsCast, JsObjectAccess},
        VTagExt,
    },
    Element, MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    use wasm_bindgen::prelude::*;

    pub const TYPE_NAME: &str = "MDCDialog";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCDialog)]
        pub type Dialog;

        #[wasm_bindgen(method)]
        pub fn open(this: &Dialog);

        #[wasm_bindgen(method)]
        pub fn close(this: &Dialog);

        #[wasm_bindgen(method)]
        pub fn layout(this: &Dialog);
    }
}

#[derive(Debug, Clone)]
pub struct Dialog {
    html: Html,
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog {
    /// The bounding box for the dialog's content.
    pub const SURFACE_CLASS: &'static str = "mdc-dialog__surface";

    /// Brief summary of the dialog's purpose.
    pub const TITLE_CLASS: &'static str = "mdc-dialog__title";

    /// Primary content area. May contain a list, a form, or prose.
    pub const CONTENT_CLASS: &'static str = "mdc-dialog__content";

    /// Footer area containing the dialog's action buttons.
    pub const ACTIONS_CLASS: &'static str = "mdc-dialog__actions";

    /// Individual action button. Typically paired with `mdc-button`.
    pub const BUTTON_CLASS: &'static str = "mdc-dialog__button";

    /// Indicates that the dialog is open and visible.
    pub const OPEN_CLASS: &'static str = "mdc-dialog--open";

    /// Any element within a dialog may include this attribute to indicate that interacting with it
    /// should close the dialog with the specified action.
    pub const ACTION_ATTR: &'static str = "data-mdc-dialog-action";

    /// This indicate that a button represents the default action.
    pub const BUTTON_DEFAULT_ATTR: &'static str = "data-mdc-dialog-button-default";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class = "mdc-dialog">
                    <div class = "mdc-dialog__container">
                        <div class = Self::SURFACE_CLASS role = "alertdialog" aria-modal = "true">
                        </div>
                    </div>
                    <div class = "mdc-dialog__scrim"></div>
                </div>
            },
        }
    }

    pub fn new() -> Self {
        let mut dialog = Self::simple();
        dialog.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        dialog
    }

    pub fn opened(self) -> Self {
        self.class(Self::OPEN_CLASS)
    }

    pub fn title(mut self, title: impl Into<Html>) -> Self {
        let mut title = title.into();
        title.add_class(Self::TITLE_CLASS);

        let surface = self.surface_mut();
        surface.remove_child_contains_class(Self::TITLE_CLASS);
        surface.insert_child(0, title);
        self
    }

    pub fn content(mut self, content: impl Into<Html>) -> Self {
        let mut content = content.into();
        content.add_class(Self::CONTENT_CLASS);

        let surface = self.surface_mut();
        surface.remove_child_contains_class(Self::CONTENT_CLASS);
        let content_idx = surface
            .find_child_contains_class_idx(Self::TITLE_CLASS)
            .map(|idx| idx + 1)
            .unwrap_or(0);
        surface.insert_child(content_idx, content);
        self
    }

    pub fn content_item(mut self, item: impl Into<Html>) -> Self {
        let content_idx = if let Some(idx) = self.surface_mut().find_child_contains_class_idx(Self::CONTENT_CLASS) {
            idx
        } else {
            self = self.content(html! { <div></div> });
            self.surface_mut()
                .find_child_contains_class_idx(Self::CONTENT_CLASS)
                .unwrap()
        };
        self.surface_mut().children[content_idx].add_child(item.into());
        self
    }

    pub fn actions(mut self, actions: impl Into<Html>) -> Self {
        let mut actions = actions.into();
        actions.add_class(Self::ACTIONS_CLASS);

        let surface = self.surface_mut();
        surface.remove_child_contains_class(Self::ACTIONS_CLASS);
        surface.add_child(actions);
        self
    }

    pub fn action(mut self, action: impl Into<Html>) -> Self {
        let surface = self.surface_mut();
        let actions_idx = surface
            .find_child_contains_class_idx(Self::ACTIONS_CLASS)
            .unwrap_or_else(|| {
                surface.add_child(html! { <div class = Self::ACTIONS_CLASS></div> });
                surface.children.len() - 1
            });
        surface.children[actions_idx].add_child(action.into());
        self
    }

    fn surface_mut(&mut self) -> &mut VTag {
        self.root_tag_mut().children.children[0]
            .find_child_contains_class_mut(Self::SURFACE_CLASS)
            .expect("Can't get dialog surface")
    }

    pub fn open_existing(id: impl AsRef<str>) {
        let dialog = dom::get_exist_element_by_id::<Element>(id.as_ref())
            .get(mdc::TYPE_NAME)
            .unchecked_into::<mdc::Dialog>();
        dialog.open();
    }

    pub fn close_existing(id: impl AsRef<str>) {
        let dialog = dom::get_exist_element_by_id::<Element>(id.as_ref())
            .get(mdc::TYPE_NAME)
            .unchecked_into::<mdc::Dialog>();
        dialog.close();
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }

    /// Indicates when the dialog begins its opening animation.
    /// event.detail: `{}`
    pub fn on_opening(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":opening"), callback)
    }

    /// Indicates when the dialog finishes its opening animation.
    /// event.detail: `{}`
    pub fn on_opened(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":opened"), callback)
    }

    /// Indicates when the dialog begins its closing animation. action represents the action which
    /// closed the dialog. event.detail: `{action: string?}`
    pub fn on_closing(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":closing"), callback)
    }

    /// Indicates when the dialog finishes its closing animation. action represents the action which
    /// closed the dialog. event.detail: `{action: string?}`
    pub fn on_closed(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":closed"), callback)
    }
}

impl MdcWidget for Dialog {
    const NAME: &'static str = stringify!(Dialog);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Dialog {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Dialog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Dialog> for Html {
    fn from(widget: Dialog) -> Self {
        widget.html
    }
}
