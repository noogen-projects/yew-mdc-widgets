use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{
    utils::{MdcWidget, VTagExt},
    Text,
};

#[derive(Debug, Clone)]
pub struct Drawer {
    html: Html,
}

impl Drawer {
    pub const MODAL_CLASS: &'static str = "mdc-drawer--modal";
    pub const DISMISSIBLE_CLASS: &'static str = "mdc-drawer--dismissible";
    pub const APP_CONTENT_CLASS: &'static str = "mdc-drawer-app-content";
    pub const HEADER_CLASS: &'static str = "mdc-drawer__header";
    pub const TITLE_CLASS: &'static str = "mdc-drawer__title";
    pub const SUBTITLE_CLASS: &'static str = "mdc-drawer__subtitle";
    pub const CONTENT_CLASS: &'static str = "mdc-drawer__content";

    pub fn get_attaching_script(id: impl AsRef<str>) -> String {
        format!(
            "mdc.drawer.MDCDrawer.attachTo(document.getElementById('{}'))",
            id.as_ref()
        )
    }

    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        Self {
            html: html! {
                <aside id = id class = "mdc-drawer">
                    <script>{ Self::get_attaching_script(id) }</script>
                </aside>
            },
        }
    }

    pub fn set_app_content<'a>(mut self, id: impl Into<Text<'a>>) -> Self {
        let statement = format!(
            "document.getElementById('{}').classList.add('{}');",
            id.into(),
            Self::APP_CONTENT_CLASS
        );
        self.root_tag_mut().add_child_script_statement(statement);
        self
    }

    pub fn standard(mut self) -> Self {
        self.root_tag_mut().remove_any_class(&[Self::MODAL_CLASS]);
        self
    }

    pub fn modal(mut self) -> Self {
        let root = self.root_tag_mut();
        if !root.is_contains_class(Self::MODAL_CLASS) {
            self.root_tag_mut().add_class(Self::MODAL_CLASS);
        }
        self
    }

    pub fn dismissible(mut self) -> Self {
        let root = self.root_tag_mut();
        if !root.is_contains_class(Self::DISMISSIBLE_CLASS) {
            self.root_tag_mut().add_class(Self::DISMISSIBLE_CLASS);
        }
        self
    }

    pub fn header(mut self, header: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        root.remove_child_contains_class(Self::HEADER_CLASS);
        root.insert_child(
            0,
            html! {
                <div class = Self::HEADER_CLASS>{ header }</div>
            },
        );
        self
    }

    pub fn title(mut self, title: impl Into<Html>) -> Self {
        let mut title = match title.into() {
            Html::VText(text) => html! { <span>{ text }</span> },
            html => html,
        };
        title
            .root_tag_mut()
            .expect("Title root tag expected")
            .add_class(Self::TITLE_CLASS);

        let root = self.root_tag_mut();
        if let Some(header) = root.find_child_contains_class_mut(Self::HEADER_CLASS) {
            header.insert_child(0, title);
            self
        } else {
            self.header(title)
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<Html>) -> Self {
        let mut subtitle = match subtitle.into() {
            Html::VText(text) => html! { <span>{ text }</span> },
            html => html,
        };
        subtitle
            .root_tag_mut()
            .expect("Subtitle root tag expected")
            .add_class(Self::SUBTITLE_CLASS);

        let root = self.root_tag_mut();
        if let Some(header) = root.find_child_contains_class_mut(Self::HEADER_CLASS) {
            header.add_child(subtitle);
            self
        } else {
            self.header(subtitle)
        }
    }

    pub fn content(mut self, content: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        root.remove_child_contains_class(Self::CONTENT_CLASS);
        let idx = root.children.len().saturating_sub(1);
        root.insert_child(
            idx,
            html! {
                <div class = Self::CONTENT_CLASS>{ content }</div>
            },
        );
        self
    }
}

impl MdcWidget for Drawer {
    const NAME: &'static str = "Drawer";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Drawer {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Drawer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Drawer> for Html {
    fn from(drawer: Drawer) -> Self {
        drawer.html
    }
}
