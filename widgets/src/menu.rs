use std::ops::{Deref, DerefMut};

use yew::{html, virtual_dom::AttrValue, Html, ToHtml};

use crate::{
    dom::{self, existing::JsObjectAccess},
    utils::{ManageChildren, VTagExt},
    Element, List, MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCMenu";
}

#[derive(Debug, Clone)]
pub struct Menu {
    html: Html,
    list: List,
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
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
        menu.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        menu
    }

    pub fn new() -> Self {
        Self::from_list(List::ul())
    }

    pub fn open_existing(id: impl AsRef<str>) {
        let menu = dom::existing::get_element_by_id::<Element>(id.as_ref()).get(mdc::TYPE_NAME);
        menu.set("open", true);
    }

    pub fn open(self) -> Self {
        let statement = format!("{}.open = true;", Self::VAR_NAME);
        self.add_script_statement(statement)
    }

    pub fn root_id(&self) -> AttrValue {
        self.root_tag().attr("id").expect("The Menu widget must have ID")
    }

    pub fn add_script_statement(mut self, statement: String) -> Self {
        if self.html.find_child_tag("script").is_some() {
            self.html.add_child_script_statement(statement);
        } else {
            let id = self.root_id();
            let script = format!(
                r"{{
                    const {menu} = document.getElementById('{id}');
                    if ({menu}.{mdc_type} === undefined) {{
                        window.mdc.autoInit({menu}.parentElement);
                    }}
                    {statement}
                }}",
                menu = Self::VAR_NAME,
                mdc_type = mdc::TYPE_NAME,
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
    const NAME: &'static str = stringify!(Menu);

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
            tag.insert_child(0, Html::from(list));
        }
        html
    }
}

impl ToHtml for Menu {
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    fn into_html(self) -> Html {
        self.into()
    }
}