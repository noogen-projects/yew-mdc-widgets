use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, virtual_dom::VTag, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct Dialog {
    html: Html,
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
        dialog.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCDialog");
        dialog
    }

    pub fn open(self) -> Self {
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
            self = self.content(html! { <div class = Self::CONTENT_CLASS></div> });
            self.surface_mut()
                .find_child_contains_class_idx(Self::CONTENT_CLASS)
                .unwrap()
        };
        self.surface_mut().insert_child(content_idx, item.into());
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
        surface.insert_child(actions_idx, action.into());
        self
    }

    pub fn on_click(self, callback: Callback<MouseEvent>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback)))
    }

    fn surface_mut(&mut self) -> &mut VTag {
        self.root_tag_mut().children.children[0]
            .find_child_contains_class_mut(Self::SURFACE_CLASS)
            .expect("Can't get dialog surface")
    }
}

impl MdcWidget for Dialog {
    const NAME: &'static str = "Dialog";

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
