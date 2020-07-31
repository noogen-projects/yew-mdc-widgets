use std::{
    rc::Rc, ops::{Deref, DerefMut},
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    Text,
    utils::{VTagExt, MdcWidget},
};

pub struct ListItem {
    html: Html,
}

impl ListItem {
    const RIPPLE_CLASS: &'static str = "mdc-list-item__ripple";
    const SELECTION_CLASS: &'static str = "mdc-list-item--selected";
    const FIRST_TILE_CLASS: &'static str = "mdc-list-item__graphic";
    const LAST_TILE_CLASS: &'static str = "mdc-list-item__meta";

    pub fn new<'a>(id: impl Into<Text<'a>>, text: impl Into<Html>) -> Self {
        let item = Self {
            html: html! {
                <li id = id.into() class = "mdc-list-item">
                    <span class = "mdc-list-item__text">{ text }</span>
                </li>
            },
        };
        item.ripple(true)
    }

    pub fn two_line<'a>(id: impl Into<Text<'a>>, primary: impl Into<Html>, secondary: impl Into<Html>) -> Self {
        Self::new(id, html! {
            <>
                <span class = "mdc-list-item__primary-text">{ primary }</span>
                <span class = "mdc-list-item__secondary-text">{ secondary }</span>
            </>
        })
    }

    pub fn selected(mut self, selected: bool) -> Self {
        let is_already_selected = self.html.is_contains_class(Self::SELECTION_CLASS);

        if is_already_selected && !selected {
            self.html.remove_any_class(&[Self::SELECTION_CLASS]);
        } else if !is_already_selected && selected {
            self.html.add_class(Self::SELECTION_CLASS);
        }
        self
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        let root_tag = self.root_tag_mut();
        if enabled {
            if !root_tag.is_some_child_contains_class(Self::RIPPLE_CLASS) {
                root_tag.children.insert(0, html! {
                    <span class = Self::RIPPLE_CLASS></span>
                });
            }
            if !root_tag.is_last_child("script") {
                if let Some(id) = root_tag.attributes.get("id") {
                    root_tag.children.push(html! {
                        <script>{ format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id) }</script>
                    });
                }
            }
        } else if !enabled {
            if let Some(idx) = root_tag.find_child_contains_class_idx(Self::RIPPLE_CLASS) {
                root_tag.children.remove(idx);
            }
            if let Some(idx) = root_tag.find_child_tag_idx("script") {
                root_tag.children.remove(idx);
            }
        }
        self
    }

    pub fn tile(mut self, tile: impl Into<Html>) -> Self {
        let root_tag = self.root_tag_mut();
        let (idx, class) = if root_tag.is_some_child_contains_class(Self::FIRST_TILE_CLASS) {
            let idx = root_tag.find_child_tag_idx("script").unwrap_or_else(|| root_tag.children.len());
            (idx, Self::LAST_TILE_CLASS)
        } else {
            let idx = root_tag
                .find_child_contains_class_idx(Self::RIPPLE_CLASS)
                .map(|idx| idx + 1)
                .unwrap_or(0);
            (idx, Self::FIRST_TILE_CLASS)
        };
        root_tag.children.insert(idx, html! {
            <span class = class>{ tile }</span>
        });
        self
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        let mut label = html! {
            <label class = "mdc-list-item__text">{ label }</label>
        };
        let root_tag = self.root_tag_mut();

        if let Some(id) = root_tag
            .find_child_tag_recursively("input")
            .and_then(|input| input.attributes.get("id"))
        {
            label.set_attr("for", id);
        }

        let idx = root_tag
            .find_child_contains_class_idx(Self::LAST_TILE_CLASS)
            .unwrap_or_else(|| root_tag.children.len());

        root_tag.children.insert(idx, label);
        self
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        self.root_tag_mut().add_listener(Rc::new(onclick::Wrapper::new(callback)));
        self
    }
}

impl MdcWidget for ListItem {
    const NAME: &'static str = "ListItem";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl From<ListItem> for Html {
    fn from(item: ListItem) -> Self {
        item.html
    }
}

pub struct List {
    html: Html,
}

impl List {
    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        Self {
            html: html! {
                <>
                    <ul id = id class = "mdc-list" tabindex = "-1">
                    </ul>
                    <script>{ format!("mdc.list.MDCList.attachTo(document.getElementById('{}'));", id) }</script>
                </>
            },
        }
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    pub fn item(mut self, item: impl Into<Html>) -> Self {
        self.root_tag_mut().children.push(item.into());
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<Html>>) -> Self {
        for item in items.into_iter() {
            self = self.item(item);
        }
        self
    }

    pub fn divider(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = "mdc-list-divider"></li>
        });
        self
    }
}

impl MdcWidget for List {
    const NAME: &'static str = "List";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for List {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<List> for Html {
    fn from(list: List) -> Self {
        list.html
    }
}