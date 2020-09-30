use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{MdcWidget, VTagExt},
    Text,
};

#[derive(Debug, Clone)]
pub struct ListItem {
    html: Html,
}

impl Default for ListItem {
    fn default() -> Self {
        let item = Self {
            html: html! {
                <li class = "mdc-list-item">
                </li>
            },
        };
        item.ripple(true)
    }
}

impl ListItem {
    const FIRST_TILE_CLASS: &'static str = "mdc-list-item__graphic";
    const LAST_TILE_CLASS: &'static str = "mdc-list-item__meta";
    const PRIMARY_TEXT_ITEM_CLASS: &'static str = "mdc-list-item__primary-text";
    const RIPPLE_CLASS: &'static str = "mdc-list-item__ripple";
    const SECONDARY_TEXT_ITEM_CLASS: &'static str = "mdc-list-item__secondary-text";
    const SELECTION_CLASS: &'static str = "mdc-list-item--selected";
    const TEXT_ITEM_CLASS: &'static str = "mdc-list-item__text";

    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        Self::default().id(id)
    }

    pub fn link(href: impl AsRef<str>) -> Self {
        let item = Self {
            html: html! {
                <a class = "mdc-list-item" href = href.as_ref()>
                </a>
            },
        };
        item.ripple(true)
    }

    pub fn id<'a>(mut self, id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        let root = self.root_tag_mut();

        root.set_attr("id", id.as_ref());
        if root.is_some_child_contains_class(Self::RIPPLE_CLASS) {
            root.remove_child_tag("script");
            root.children.push(html! {
                <script>{ format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id) }</script>
            });
        }
        self
    }

    pub fn text(mut self, text: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();

        if let Some(idx) = root.find_child_contains_class_idx(Self::TEXT_ITEM_CLASS) {
            let mut primary = root.children.remove(idx);
            primary.remove_any_class(&[Self::TEXT_ITEM_CLASS]);
            primary.add_class(Self::PRIMARY_TEXT_ITEM_CLASS);

            root.children.insert(idx, html! {
                <span class = Self::TEXT_ITEM_CLASS>
                    { primary }
                    <span class = Self::SECONDARY_TEXT_ITEM_CLASS>
                        { text }
                    </span>
                </span>
            });
        } else {
            let idx = root
                .find_child_contains_class_idx(Self::LAST_TILE_CLASS)
                .or_else(|| root.find_child_tag_idx("script"))
                .unwrap_or_else(|| root.children.len());
            root.children.insert(idx, html! {
                <span class = Self::TEXT_ITEM_CLASS>{ text }</span>
            });
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
            if !root.is_some_child_contains_class(Self::RIPPLE_CLASS) {
                root.children.insert(0, html! {
                    <span class = Self::RIPPLE_CLASS></span>
                });
            }
            if !root.is_last_child("script") {
                if let Some(id) = root.attributes.get("id") {
                    root.children.push(html! {
                        <script>{ format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id) }</script>
                    });
                }
            }
        } else if !enabled {
            if let Some(idx) = root.find_child_contains_class_idx(Self::RIPPLE_CLASS) {
                root.children.remove(idx);
            }
            if let Some(idx) = root.find_child_tag_idx("script") {
                root.children.remove(idx);
            }
        }
        self
    }

    pub fn tile(mut self, tile: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let (idx, class) = if root.is_some_child_contains_class(Self::FIRST_TILE_CLASS)
            || root.is_some_child_contains_class(Self::TEXT_ITEM_CLASS)
        {
            let idx = root.find_child_tag_idx("script").unwrap_or_else(|| root.children.len());
            (idx, Self::LAST_TILE_CLASS)
        } else {
            let idx = root
                .find_child_contains_class_idx(Self::RIPPLE_CLASS)
                .map(|idx| idx + 1)
                .unwrap_or(0);
            (idx, Self::FIRST_TILE_CLASS)
        };
        root.children.insert(idx, html! {
            <span class = class>{ tile }</span>
        });
        self
    }

    pub fn icon<'a>(mut self, name: impl Into<Text<'a>>) -> Self {
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
            .or_else(|| root.find_child_tag_idx("script"))
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
    fn from(item: ListItem) -> Self {
        item.html
    }
}

#[derive(Debug, Clone)]
pub struct List {
    html: Html,
}

impl List {
    const LIST_VAR_NAME: &'static str = "list";

    pub fn ul<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        Self {
            html: html! {
                <>
                    <ul id = id class = "mdc-list">
                    </ul>
                    <script>{ format!("{{ const {} = mdc.list.MDCList.attachTo(document.getElementById('{}')); }}", Self::LIST_VAR_NAME, id) }</script>
                </>
            },
        }
    }

    pub fn nav<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        Self {
            html: html! {
                <>
                    <nav id = id class = "mdc-list">
                    </nav>
                    <script>{ format!("{{ const {} = mdc.list.MDCList.attachTo(document.getElementById('{}')); }}", Self::LIST_VAR_NAME, id) }</script>
                </>
            },
        }
    }

    pub fn single_selection(mut self) -> Self {
        self.html
            .add_child_script_statement(format!("{}.singleSelection = true;", Self::LIST_VAR_NAME));
        self
    }

    pub fn wrap_focus(mut self) -> Self {
        self.html
            .add_child_script_statement(format!("{}.wrapFocus = true;", Self::LIST_VAR_NAME));
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
    fn from(list: List) -> Self {
        list.html
    }
}
