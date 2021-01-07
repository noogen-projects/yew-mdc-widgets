use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{
    utils::{MdcWidget, VTagExt},
    List, AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct Menu {
    html: Html,
    list: List,
}

impl Menu {
    pub const VAR_NAME: &'static str = "menu";
    pub const ANCHOR_CLASS: &'static str = "mdc-menu-surface--anchor";

    pub fn from_list(list: List) -> Self {
        let list = list.markup_only().attr("role", "menu");
        let mut menu = Self {
            html: html! {
                <div class = "mdc-menu mdc-menu-surface"></div>
            },
            list,
        };
        menu.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCMenu");
        menu
    }

    pub fn new() -> Self {
        Self::from_list(List::ul())
    }

    pub fn open_existing(id: impl AsRef<str>) {
        js_sys::eval(&format!(
            "document.getElementById('{}').MDCMenu.open = true;",
            id.as_ref()
        ))
        .expect("JavaScript evaluation error");
    }

    pub fn open(self) -> Self {
        let statement = format!("{}.open = true;", Self::VAR_NAME,);
        self.add_script_statement(statement)
    }

    pub fn root_id(&self) -> &str {
        self.root_tag()
            .attributes
            .get("id")
            .expect("The Menu widget must have ID")
    }

    pub fn add_script_statement(mut self, statement: String) -> Self {
        if self.html.find_child_tag("script").is_some() {
            self.html.add_child_script_statement(statement);
        } else {
            let id = self.root_id();
            let script = format!(
                r"{{
                    const {menu} = document.getElementById('{id}');
                    if ({menu}.MDCMenu === undefined) {{
                        window.mdc.autoInit({menu}.parentElement);
                    }}
                    {statement}
                }}",
                menu = Self::VAR_NAME,
                id = id,
                statement = statement,
            );

            let Self { html, list } = self;
            self = Self {
                html: html! {
                    <>
                        { html }
                        <script>{ script }</script>
                    </>
                },
                list,
            };
        }
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
