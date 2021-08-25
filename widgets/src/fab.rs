use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{ripple, utils::VTagExt, MdcWidget, AUTO_INIT_ATTR};

#[derive(Debug, Clone)]
pub struct Fab {
    html: Html,
}

impl Default for Fab {
    fn default() -> Self {
        Self::new()
    }
}

impl Fab {
    pub const ICON_CLASS: &'static str = "mdc-fab__icon";
    pub const MINI_CLASS: &'static str = "mdc-fab--mini";
    pub const EXTENDED_CLASS: &'static str = "mdc-fab--extended";

    /// Animates the FAB out of view. When this class is removed, the FAB will return to view.
    pub const EXITED_CLASS: &'static str = "mdc-fab--exited";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <button class = "mdc-fab">
                    <div class = "mdc-fab__ripple"></div>
                </button>
            },
        }
    }

    pub fn new() -> Self {
        let mut icon_button = Self::simple();
        icon_button
            .root_tag_mut()
            .set_attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME);
        icon_button
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.root_tag_mut().add_child(html! {
            <span class = vec![Self::ICON_CLASS, "material-icons"]>{ name.into() }</span>
        });
        self
    }

    pub fn item(mut self, item: impl Into<Html>) -> Self {
        let mut item = item.into();
        item.add_class(Self::ICON_CLASS);

        self.root_tag_mut().add_child(item);
        self
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if enabled {
            self.root_tag_mut().set_attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME);
        } else {
            self.root_tag_mut().remove_attr(AUTO_INIT_ATTR);
        }
        self
    }

    /// Modifies the FAB to a smaller size
    pub fn mini(self) -> Self {
        self.class(Self::MINI_CLASS)
    }

    /// Modifies the FAB to wider size which includes a text label
    pub fn label(mut self, label: impl Into<Html>) -> Self {
        self = self.class(Self::EXTENDED_CLASS);
        self.root_tag_mut().add_child(html! {
            <span class = "mdc-fab__label">{ label.into() }</span>
        });
        self
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }
}

impl MdcWidget for Fab {
    const NAME: &'static str = stringify!(Fab);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Fab {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Fab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Fab> for Html {
    fn from(widget: Fab) -> Self {
        widget.html
    }
}
