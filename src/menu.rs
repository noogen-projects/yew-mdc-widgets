use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{
    Text, List,
    utils::{VTagExt, MdcWidget},
};

pub struct Menu {
    html: Html,
    list: List,
}

impl Menu {
    pub const ANCHOR_CLASS: &'static str = "mdc-menu-surface--anchor";

    pub fn from_list<'a>(id: impl Into<Text<'a>>, list: List) -> Self {
        let id = id.into();
        let list = list.markup_only();
        Self {
            html: html! {
                <div id = id class = "mdc-menu mdc-menu-surface">
                    <script>{ format!("mdc.menu.MDCMenu.attachTo(document.getElementById('{}'));", id) }</script>
                </div>
            },
            list,
        }
    }

    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        let list = List::new(format!("{}-list", id));
        Self::from_list(id, list)
    }

    pub fn open_existing(id: impl AsRef<str>) {
        js_sys::eval(&format!("mdc.menu.MDCMenu.attachTo(document.getElementById('{}')).open = true;", id.as_ref()))
            .expect("JavaScript evaluation error");
    }

    pub fn open(mut self) -> Self {
        let root = self.root_tag_mut();
        if let Some(id) = root.attr("id") {
            let statement = format!("mdc.menu.MDCMenu.attachTo(document.getElementById('{}')).open = true;", id);
            root.add_child_script_statement(statement);
        }
        self
    }

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    pub fn item(mut self, item: impl Into<Html>) -> Self {
        self.list = self.list.item(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<Html>>) -> Self {
        self.list = self.list.items(items);
        self
    }

    pub fn divider(mut self) -> Self {
        self.list = self.list.divider();
        self
    }

    /// Increases the leading margin of the divider so that it does not intersect
    /// the graphics column.
    pub fn divider_inset_leading(mut self) -> Self {
        self.list = self.list.divider_inset_leading();
        self
    }

    /// Increases the trailing margin of the divider so that it coincides with the
    /// item's padding.
    pub fn divider_inset_trailing(mut self) -> Self {
        self.list = self.list.divider_inset_trailing();
        self
    }

    /// Alters the inset to correspond to the item's padding rather than the leading
    /// graphics column.
    pub fn divider_inset_padding(mut self) -> Self {
        self.list = self.list.divider_inset_padding();
        self
    }
}

impl MdcWidget for Menu {
    const NAME: &'static str = "Menu";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Menu {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Menu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Menu> for Html {
    fn from(menu: Menu) -> Self {
        let Menu { mut html, list } = menu;
        if let Html::VTag(tag) = &mut html {
            tag.children.insert(0, Html::from(list));
        }
        html
    }
}