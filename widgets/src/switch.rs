use std::ops::{Deref, DerefMut};

use yew::{classes, html, virtual_dom::VTag, Callback, Html, MouseEvent};

use crate::{
    utils::{labeled_on_click, IntoWidgetWithVList, ManageChildren, VTagExt},
    MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCSwitch";
}

#[derive(Debug, Clone)]
pub struct Switch {
    html: Html,
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Switch {
    pub const CLASS: &'static str = "mdc-switch";

    /// Styles the switch as unselected ("off")
    pub const UNSELECTED_CLASS: &'static str = "mdc-switch--unselected";

    /// Styles the switch as selected ("on")
    pub const SELECTED_CLASS: &'static str = "mdc-switch--selected";

    pub const TRACK_CLASS: &'static str = "mdc-switch__track";

    pub const HANDLE_TRACK_CLASS: &'static str = "mdc-switch__handle-track";

    pub const HANDLE_CLASS: &'static str = "mdc-switch__handle";

    pub const SHADOW_CLASS: &'static str = "mdc-switch__shadow";

    pub const ELEVATION_OVERLAY_CLASS: &'static str = "mdc-elevation-overlay";

    pub const RIPPLE_CLASS: &'static str = "mdc-switch__ripple";

    pub const ICONS_CLASS: &'static str = "mdc-switch__icons";

    pub const ICON_CLASS: &'static str = "mdc-switch__icon";

    pub const ICON_ON_CLASS: &'static str = "mdc-switch__icon--on";

    pub const ICON_OFF_CLASS: &'static str = "mdc-switch__icon--off";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <button class = { classes!(Self::CLASS, Self::UNSELECTED_CLASS) } aria-checked = "false">
                    <div class = { Self::TRACK_CLASS }></div>
                    <div class = { Self::HANDLE_TRACK_CLASS }>
                        <div class = { Self::HANDLE_CLASS }>
                            <div class = { Self::SHADOW_CLASS }>
                                <div class = { Self::ELEVATION_OVERLAY_CLASS }></div>
                            </div>
                            <div class = { Self::RIPPLE_CLASS }></div>
                        </div>
                    </div>
                </button>
            },
        }
    }

    pub fn new() -> Self {
        let mut switch = Self::simple();
        switch.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        switch
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        let id = self.root_tag().attr("id").expect("Cannot find root tag id");
        self = self.into_widget_with_v_list();
        self.html_mut().add_child(html! {
            <label for = { id }>{ label }</label>
        });
        self
    }

    pub fn disabled(self) -> Self {
        self.disable(true)
    }

    pub fn disable(mut self, disable: bool) -> Self {
        if disable {
            self.root_tag_mut().set_attr("disabled", "true");
        } else {
            self.root_tag_mut().remove_attr_or_prop("disabled");
        }
        self
    }

    pub fn on(self) -> Self {
        self.turn(true)
    }

    pub fn turn(mut self, on: bool) -> Self {
        let root = self.root_tag_mut();

        if on {
            root.remove_class(Self::UNSELECTED_CLASS);
            root.add_class_if_needed(Self::SELECTED_CLASS);
        } else {
            root.remove_class(Self::SELECTED_CLASS);
            root.add_class_if_needed(Self::UNSELECTED_CLASS);
        }

        root.set_attr("aria-checked", if on { "true" } else { "false" });
        self
    }

    pub fn icon_on(mut self, icon: impl Into<Html>) -> Self {
        let mut icon = icon.into();
        icon.add_class_if_needed(Self::ICON_CLASS);
        icon.add_class_if_needed(Self::ICON_ON_CLASS);

        self.icons().insert_child(0, icon);
        self
    }

    pub fn icon_on_default(self) -> Self {
        self.icon_on(html! {
            <svg viewBox = "0 0 24 24">
                <path d = "M19.69,5.23L8.96,15.96l-4.23-4.23L2.96,13.5l6,6L21.46,7L19.69,5.23z" />
            </svg>
        })
    }

    pub fn icon_off(mut self, icon: impl Into<Html>) -> Self {
        let mut icon = icon.into();
        icon.add_class_if_needed(Self::ICON_CLASS);
        icon.add_class_if_needed(Self::ICON_OFF_CLASS);

        self.icons().add_child(icon);
        self
    }

    pub fn icon_off_default(self) -> Self {
        self.icon_off(html! {
            <svg viewBox = "0 0 24 24">
                <path d = "M20 13H4v-2h16v2z" />
            </svg>
        })
    }

    pub fn on_click(mut self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        labeled_on_click(&mut self, callback.into());
        self
    }

    fn icons(&mut self) -> &mut VTag {
        let handle = self
            .root_tag_mut()
            .find_child_contains_class_recursively_mut(Self::HANDLE_CLASS)
            .unwrap();
        if handle.find_child_contains_class_mut(Self::ICONS_CLASS).is_none() {
            handle.add_child(html! { <div class = { Self::ICONS_CLASS }></div> });
        }
        handle.find_child_contains_class_mut(Self::ICONS_CLASS).unwrap()
    }
}

impl MdcWidget for Switch {
    const NAME: &'static str = stringify!(Switch);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }

    fn id(mut self, id: impl Into<String>) -> Self {
        self.root_tag_mut().set_attr("id", id.into());
        self
    }
}

impl IntoWidgetWithVList for Switch {
    fn into_widget_with_v_list(mut self) -> Self {
        if !matches!(self.html, Html::VList(_)) {
            self.html = html! { <>{ self.html }</> }
        }
        self
    }
}

impl Deref for Switch {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Switch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Switch> for Html {
    fn from(widget: Switch) -> Self {
        widget.html
    }
}
