use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{classes, html, html::onclick, Callback, Html, MouseEvent, ToHtml};

use crate::{
    ripple,
    utils::{ManageChildren, VTagExt},
    MdcWidget, AUTO_INIT_ATTR, MATERIAL_ICONS_CLASS,
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

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    /// Defaults to a text button that is flush with the surface.
    pub const CLASS: &'static str = "mdc-button";

    /// Indicates the element which shows the ripple styling.
    pub const RIPPLE_CLASS: &'static str = "mdc-button__ripple";

    /// Indicates the element containing the button's text label.
    pub const LABEL_CLASS: &'static str = "mdc-button__label";

    /// Indicates the element containing the button's icon.
    pub const ICON_CLASS: &'static str = "mdc-button__icon";

    pub fn simple() -> Self {
        Self {
            html: html! { <button class = { Self::CLASS }><div class = { Self::RIPPLE_CLASS }></div></button> },
        }
    }

    pub fn new() -> Self {
        let mut button = Self::simple();
        button.root_tag_mut().set_attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME);
        button
    }

    pub fn outlined() -> Self {
        Self::new().style(ButtonStyle::Outlined)
    }

    pub fn raised() -> Self {
        Self::new().style(ButtonStyle::Raised)
    }

    pub fn unelevated() -> Self {
        Self::new().style(ButtonStyle::Unelevated)
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        self
            .root_tag_mut()
            .add_child(html! {
                <span class = { Self::LABEL_CLASS }>{ label.into() }</span>
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
            root.set_attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME);
        } else {
            root.remove_attr_or_prop(AUTO_INIT_ATTR);
        }
        self
    }

    pub fn disable(mut self, disabled: bool) -> Self {
        if disabled {
            self.root_tag_mut().set_attr("disabled", "");
        } else {
            self.root_tag_mut().remove_attr_or_prop("disabled");
        }
        self
    }

    pub fn disabled(self) -> Self {
        self.disable(true)
    }

    pub fn add_before_label(mut self, item: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let idx = root
            .find_child_contains_class_idx(Self::LABEL_CLASS)
            .unwrap_or_else(|| {
                if root.is_last_child("script") {
                    root.children_count() - 1
                } else {
                    root.children_count()
                }
            });
        root
            .children_mut()
            .unwrap(/* root tag of button always has children */)
            .to_vlist_mut()
            .insert(idx, item.into());
        self
    }

    pub fn add_after_label(mut self, item: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let idx = root
            .find_child_contains_class_idx(Self::LABEL_CLASS)
            .map(|idx| idx + 1)
            .unwrap_or_else(|| {
                if root.is_last_child("script") {
                    root.children_count() - 1
                } else {
                    root.children_count()
                }
            });
        root
            .children_mut()
            .unwrap(/* root tag of button always has children */)
            .to_vlist_mut()
            .insert(idx, item.into());
        self
    }

    pub fn icon(self, name: impl Into<String>) -> Self {
        self.add_after_label(html! {
            <i class = { classes!(MATERIAL_ICONS_CLASS, Self::ICON_CLASS) } aria-hidden = "true">{ name.into() }</i>
        })
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }
}

impl MdcWidget for Button {
    const NAME: &'static str = stringify!(Button);

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

impl ToHtml for Button {
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    fn into_html(self) -> Html {
        self.into()
    }
}