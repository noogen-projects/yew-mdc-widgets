use std::ops::{Deref, DerefMut};

use yew::{html, virtual_dom::AttrValue, Html, ToHtml};

use crate::{
    utils::{ManageChildren, VTagExt},
    MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCDrawer";
}

#[derive(Debug, Clone)]
pub struct Drawer {
    html: Html,
}

impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawer {
    pub const VAR_NAME: &'static str = "drawer";
    pub const APP_CONTENT_CLASS: &'static str = "mdc-drawer-app-content";
    pub const CONTENT_CLASS: &'static str = "mdc-drawer__content";
    pub const DISMISSIBLE_CLASS: &'static str = "mdc-drawer--dismissible";
    pub const HEADER_CLASS: &'static str = "mdc-drawer__header";
    pub const MODAL_CLASS: &'static str = "mdc-drawer--modal";
    pub const SUBTITLE_CLASS: &'static str = "mdc-drawer__subtitle";
    pub const TITLE_CLASS: &'static str = "mdc-drawer__title";

    pub fn new() -> Self {
        let mut drawer = Self {
            html: html! {
                <aside class = "mdc-drawer"></aside>
            },
        };
        drawer.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        drawer
    }

    pub fn root_id(&self) -> AttrValue {
        self.root_tag().attr("id").expect("The Drawer widget must have ID")
    }

    pub fn add_script_statement(mut self, statement: String) -> Self {
        if self.html.find_child_tag("script").is_some() {
            self.html.add_child_script_statement(statement);
        } else {
            let id = self.root_id();
            let script = format!(
                r"{{
                    const {drawer} = document.getElementById('{id}');
                    if ({drawer}.{mdc_type} === undefined) {{
                        window.mdc.autoInit({drawer}.parentElement);
                    }}
                    {statement}
                }}",
                drawer = Self::VAR_NAME,
                mdc_type = mdc::TYPE_NAME,
                id = id,
                statement = statement,
            );

            let Self { html } = self;
            self = Self {
                html: html! {
                    <>
                        { html }
                        <script>{ script }</script>
                    </>
                },
            };
        }
        self
    }

    pub fn standard(mut self) -> Self {
        self.root_tag_mut().remove_class(Self::MODAL_CLASS);
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
        root.insert_child(0, html! {
            <div class = { Self::HEADER_CLASS }>{ header.into() }</div>
        });
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
            header.remove_child_contains_class(Self::TITLE_CLASS);
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
            header.remove_child_contains_class(Self::SUBTITLE_CLASS);
            header.add_child(subtitle);
            self
        } else {
            self.header(subtitle)
        }
    }

    pub fn content(mut self, content: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        root.remove_child_contains_class(Self::CONTENT_CLASS);
        root.add_child(html! {
            <div class = { Self::CONTENT_CLASS }>{ content.into() }</div>
        });
        self
    }
}

impl MdcWidget for Drawer {
    const NAME: &'static str = stringify!(Drawer);

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
    fn from(widget: Drawer) -> Self {
        widget.html
    }
}

impl ToHtml for Drawer {
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    fn into_html(self) -> Html {
        self.into()
    }
}
