use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{
    utils::{MdcWidget, VTagExt},
    Text,
};

#[derive(Debug, Clone)]
pub struct TopAppBar {
    html: Html,
}

impl TopAppBar {
    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        Self {
            html: html! {
                <header id = id class = "mdc-top-app-bar">
                    <div class = "mdc-top-app-bar__row">
                        <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                            <span class = "mdc-top-app-bar__title"></span>
                        </section>
                        <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-end" role = "toolbar">
                        </section>
                    </div>
                    <script>{ format!("mdc.topAppBar.MDCTopAppBar.attachTo(document.getElementById('{}'));", id) }</script>
                </header>
            },
        }
    }

    pub fn title(mut self, title: impl Into<Html>) -> Self {
        if let Some(Html::VTag(row)) = self.root_tag_mut().children.first_mut() {
            if let Some(Html::VTag(start_section)) = row.children.first_mut() {
                if let Some(title_tag) = start_section.find_child_contains_class_mut("mdc-top-app-bar__title") {
                    title_tag.add_child(title.into());
                }
            }
        }
        self
    }

    pub fn navigation_item(mut self, item: impl Into<Html>) -> Self {
        let mut item = item.into();
        item.add_class("mdc-top-app-bar__navigation-icon");

        if let Some(Html::VTag(row)) = self.root_tag_mut().children.first_mut() {
            if let Some(Html::VTag(start_section)) = row.children.first_mut() {
                let idx = start_section.children.len() - 1;
                start_section.children.insert(idx, item);
            }
        }
        self
    }

    pub fn action_item(mut self, item: impl Into<Html>) -> Self {
        let mut item = item.into();
        item.add_class("mdc-top-app-bar__action-item");

        if let Some(Html::VTag(row)) = self.root_tag_mut().children.first_mut() {
            if let Some(Html::VTag(end_section)) = row.children.last_mut() {
                end_section.children.push(item);
            }
        }
        self
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }
}

impl MdcWidget for TopAppBar {
    const NAME: &'static str = "TopAppBar";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for TopAppBar {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for TopAppBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<TopAppBar> for Html {
    fn from(bar: TopAppBar) -> Self {
        bar.html
    }
}
