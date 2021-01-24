use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct ListItem {
    html: Html,
}

impl ListItem {
    pub const FIRST_TILE_CLASS: &'static str = "mdc-list-item__graphic";
    pub const LAST_TILE_CLASS: &'static str = "mdc-list-item__meta";
    pub const PRIMARY_TEXT_ITEM_CLASS: &'static str = "mdc-list-item__primary-text";
    pub const RIPPLE_CLASS: &'static str = "mdc-list-item__ripple";
    pub const SECONDARY_TEXT_ITEM_CLASS: &'static str = "mdc-list-item__secondary-text";
    pub const SELECTION_CLASS: &'static str = "mdc-list-item--selected";
    pub const TEXT_ITEM_CLASS: &'static str = "mdc-list-item__text";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <li class = "mdc-list-item">
                    <span class = Self::RIPPLE_CLASS></span>
                </li>
            },
        }
    }

    pub fn new() -> Self {
        Self::simple().ripple(true)
    }

    pub fn link(href: impl AsRef<str>) -> Self {
        let item = Self {
            html: html! {
                <a class = "mdc-list-item" href = href.as_ref()>
                    <span class = Self::RIPPLE_CLASS></span>
                </a>
            },
        };
        item.ripple(true)
    }

    pub fn text(mut self, text: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();

        if let Some(idx) = root.find_child_contains_class_idx(Self::TEXT_ITEM_CLASS) {
            let mut primary = root.children.remove(idx);
            primary.remove_any_class(&[Self::TEXT_ITEM_CLASS]);
            primary.add_class(Self::PRIMARY_TEXT_ITEM_CLASS);

            root.children.insert(
                idx,
                html! {
                    <span class = Self::TEXT_ITEM_CLASS>
                        { primary }
                        <span class = Self::SECONDARY_TEXT_ITEM_CLASS>
                            { text }
                        </span>
                    </span>
                },
            );
        } else {
            let idx = root
                .find_child_contains_class_idx(Self::LAST_TILE_CLASS)
                .unwrap_or_else(|| root.children.len());
            root.children.insert(
                idx,
                html! {
                    <span class = Self::TEXT_ITEM_CLASS>{ text }</span>
                },
            );
        }
        self
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
        let root = self.root_tag_mut();

        if enabled {
            root.set_attr(AUTO_INIT_ATTR, "MDCRipple");
        } else {
            root.remove_attr(AUTO_INIT_ATTR);
        }
        self
    }

    pub fn tile(mut self, tile: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let (idx, class) = if root.is_some_child_contains_class(Self::FIRST_TILE_CLASS)
            || root.is_some_child_contains_class(Self::TEXT_ITEM_CLASS)
        {
            (root.children.len(), Self::LAST_TILE_CLASS)
        } else {
            let idx = root
                .find_child_contains_class_idx(Self::RIPPLE_CLASS)
                .map(|idx| idx + 1)
                .unwrap_or(0);
            (idx, Self::FIRST_TILE_CLASS)
        };
        root.children.insert(
            idx,
            html! {
                <span class = class>{ tile }</span>
            },
        );
        self
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self = self.tile(name.into());

        let root_tag = self.root_tag_mut();
        let tile_idx = root_tag
            .find_child_contains_class_idx(Self::LAST_TILE_CLASS)
            .or_else(|| root_tag.find_child_contains_class_idx(Self::FIRST_TILE_CLASS))
            .expect("The widget must have tile!");

        root_tag.children[tile_idx].add_class("material-icons");
        root_tag.children[tile_idx].set_attr("aria-hidden", "true");
        self
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        let mut label = html! {
            <label class = "mdc-list-item__text">{ label }</label>
        };
        let root = self.root_tag_mut();

        if let Some(id) = root
            .find_child_tag_recursively("input")
            .and_then(|input| input.attributes.get("id"))
        {
            label.set_attr("for", id);
        }

        let idx = root
            .find_child_contains_class_idx(Self::LAST_TILE_CLASS)
            .unwrap_or_else(|| root.children.len());

        root.children.insert(idx, label);
        self
    }

    pub fn on_click(self, callback: Callback<MouseEvent>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback)))
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
    fn from(widget: ListItem) -> Self {
        widget.html
    }
}

#[derive(Debug, Clone)]
pub struct List {
    html: Html,
}

impl List {
    const LIST_VAR_NAME: &'static str = "list";

    pub fn ul() -> Self {
        let mut list = Self {
            html: html! { <ul class = "mdc-list"></ul> },
        };
        list.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCList");
        list
    }

    pub fn nav() -> Self {
        let mut list = Self {
            html: html! { <nav class = "mdc-list"></nav> },
        };
        list.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCList");
        list
    }

    pub fn root_id(&self) -> &str {
        self.root_tag()
            .attributes
            .get("id")
            .expect("The List widget must have ID")
    }

    pub fn single_selection(self) -> Self {
        let statement = format!("{}.MDCList.singleSelection = true;", Self::LIST_VAR_NAME);
        self.add_script_statement(statement)
    }

    pub fn wrap_focus(self) -> Self {
        let statement = format!("{}.MDCList.wrapFocus = true;", Self::LIST_VAR_NAME);
        self.add_script_statement(statement)
    }

    pub fn add_script_statement(mut self, statement: String) -> Self {
        if self.html.find_child_tag("script").is_some() {
            self.html.add_child_script_statement(statement);
        } else {
            let id = self.root_id();
            let script = format!(
                r"{{
                    const {list} = document.getElementById('{id}');
                    if ({list}.MDCList === undefined) {{
                        window.mdc.autoInit({list}.parentElement);
                    }}
                    {statement}
                }}",
                list = Self::LIST_VAR_NAME,
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

    /// Modifier to style list with two lines (primary and secondary lines).
    pub fn two_line(self) -> Self {
        self.class("mdc-list--two-line")
    }

    /// Styles the density of the list, making it appear more compact.
    pub fn dense(self) -> Self {
        self.class("mdc-list--dense")
    }

    /// configures the leading tile of each row to display avatars.
    pub fn avatar(self) -> Self {
        self.class("mdc-list--avatar-list")
    }

    /// Configures lists that start with text (e.g., do not have a leading tile).
    pub fn textual(self) -> Self {
        self.class("mdc-list--textual-list")
    }

    /// Configures the leading tile of each row to display icons.
    pub fn icon(self) -> Self {
        self.class("mdc-list--icon-list")
    }

    /// Configures the leading tile of each row to display images.
    pub fn image(self) -> Self {
        self.class("mdc-list--image-list")
    }

    /// Configures the leading tile of each row to display smaller images (this is analogous to
    /// an avatar list but the image will not be rounded).
    pub fn thumbnail(self) -> Self {
        self.class("mdc-list--thumbnail-list")
    }

    /// Configures the leading tile of each row to display videos.
    pub fn video(self) -> Self {
        self.class("mdc-list--video-list")
    }

    pub fn item(mut self, item: impl Into<Html>) -> Self {
        let mut item = item.into();
        let root = self.root_tag_mut();
        let item_number = root.children.len();

        if item.attr("id").is_none() && item.is_some_child_contains_class(ListItem::RIPPLE_CLASS) {
            if let Some(id) = root.attr("id") {
                item = ListItem { html: item }
                    .id(format!("{}-item-{}", id, item_number))
                    .into();
            }
        }
        root.children.push(item);
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

    /// Increases the leading margin of the divider so that it does not intersect
    /// the graphics column.
    pub fn divider_inset_leading(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = "mdc-list-divider mdc-list-divider--inset-leading"></li>
        });
        self
    }

    /// Increases the trailing margin of the divider so that it coincides with the
    /// item's padding.
    pub fn divider_inset_trailing(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = "mdc-list-divider mdc-list-divider--inset-trailing"></li>
        });
        self
    }

    /// Alters the inset to correspond to the item's padding rather than the leading
    /// graphics column.
    pub fn divider_inset_padding(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = "mdc-list-divider mdc-list-divider--inset-padding"></li>
        });
        self
    }

    pub fn subheader(self, subheader: impl Into<Html>) -> Self {
        let mut subheader = subheader.into();
        subheader.add_class("mdc-list-group__subheader");
        self.item(subheader)
    }

    pub fn markup_only(mut self) -> Self {
        if let Html::VList(mut list) = self.html {
            self.html = list.children.remove(0);
        }
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
    fn from(widget: List) -> Self {
        widget.html
    }
}
