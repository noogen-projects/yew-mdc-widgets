use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{classes, html, html::onclick, virtual_dom::AttrValue, Callback, Html, MouseEvent};

use crate::{ripple, utils::VTagExt, MdcWidget, AUTO_INIT_ATTR};

pub mod mdc {
    use wasm_bindgen::prelude::*;
    use web_sys::Element;

    pub const TYPE_NAME: &str = "MDCList";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCList)]
        pub type List;

        #[wasm_bindgen(js_name = MDCListIndex)]
        pub type ListIndex;

        /// Sets the list to an orientation causing the keys used for navigation to change.
        /// `true` results in the Up/Down arrow keys being used. `false` results in the
        /// Left/Right arrow keys being used.
        #[wasm_bindgen(method, setter)]
        pub fn set_vertical(this: &List, value: bool);

        /// Returns all list item elements including disabled list items.
        #[wasm_bindgen(method, getter)]
        pub fn list_elements(this: &List) -> Vec<Element>;

        /// Sets the list to allow the up arrow on the first element to focus the last element of
        /// the list and vice versa.
        #[wasm_bindgen(method, setter)]
        pub fn set_wrap_focus(this: &List, value: bool);

        /// Returns whether typeahead is currently matching a user-specified prefix.
        #[wasm_bindgen(method, getter)]
        pub fn typeahead_in_progress(this: &List) -> bool;

        /// Sets whether typeahead is enabled on the list.
        #[wasm_bindgen(method, setter)]
        pub fn set_has_typeahead(this: &List, value: bool);

        /// Sets the list to be a selection list. Enables the `enter` and `space` keys for
        /// selecting/deselecting a list item.
        #[wasm_bindgen(method, setter)]
        pub fn set_single_selection(this: &List, value: bool);

        /// Gets the current selection state by returning selected index or list of indexes for checkbox based list.
        #[wasm_bindgen(method, getter)]
        pub fn selected_index(this: &List) -> ListIndex;

        /// Sets the selection state to given index or list of indexes if it is checkbox based list.
        #[wasm_bindgen(method, setter)]
        pub fn set_selected_index(this: &List, value: &ListIndex);

        /// Recalculates layout and orientation.
        #[wasm_bindgen(method)]
        pub fn layout(this: &List);

        /// Fetches the primary text in the given element.
        #[wasm_bindgen(method)]
        pub fn get_primary_text(this: &List, item: &Element) -> String;

        /// Initialize `selectedIndex` value based on pre-selected checkbox list items,
        /// single selection or radio.
        #[wasm_bindgen(method)]
        pub fn initialize_list_type(this: &List);

        /// Updates the list item at `itemIndex` to the desired `isEnabled` state.
        #[wasm_bindgen(method)]
        pub fn set_enabled(this: &List, item_index: usize, is_enabled: bool);
    }
}

#[derive(Debug, Clone)]
pub struct ListItem {
    html: Html,
}

impl ListItem {
    pub const CLASS: &'static str = "mdc-deprecated-list-item";
    pub const FIRST_TILE_CLASS: &'static str = "mdc-deprecated-list-item__graphic";
    pub const LAST_TILE_CLASS: &'static str = "mdc-deprecated-list-item__meta";
    pub const PRIMARY_TEXT_ITEM_CLASS: &'static str = "mdc-deprecated-list-item__primary-text";
    pub const RIPPLE_CLASS: &'static str = "mdc-deprecated-list-item__ripple";
    pub const SECONDARY_TEXT_ITEM_CLASS: &'static str = "mdc-deprecated-list-item__secondary-text";
    pub const SELECTION_CLASS: &'static str = "mdc-deprecated-list-item--selected";
    pub const TEXT_ITEM_CLASS: &'static str = "mdc-deprecated-list-item__text";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <li class = Self::CLASS />
            },
        }
    }

    pub fn simple_link(href: impl Into<AttrValue>) -> Self {
        Self {
            html: html! {
                <a class = Self::CLASS href = href.into() />
            },
        }
    }

    pub fn new() -> Self {
        Self::simple().interactive().ripple(true)
    }

    pub fn link(href: impl Into<AttrValue>) -> Self {
        Self::simple_link(href).interactive().ripple(true)
    }

    pub fn interactive(mut self) -> Self {
        let root = self.root_tag_mut();
        if root.find_child_contains_class_idx(Self::RIPPLE_CLASS).is_none() {
            root.add_child(html! { <span class = Self::RIPPLE_CLASS></span> })
        }
        self
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        let root = self.root_tag_mut();

        if enabled {
            root.set_attr(AUTO_INIT_ATTR, ripple::mdc::TYPE_NAME);
        } else {
            root.remove_attr(AUTO_INIT_ATTR);
        }
        self
    }

    pub fn tab_index(self, index: isize) -> Self {
        self.attr("tabindex", format!("{}", index))
    }

    pub fn text(mut self, text: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();

        if let Some(idx) = root.find_child_contains_class_idx(Self::TEXT_ITEM_CLASS) {
            let mut primary = root.children.remove(idx);
            primary.remove_class(Self::TEXT_ITEM_CLASS);
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
            self.html.remove_class(Self::SELECTION_CLASS);
        } else if !is_already_selected && selected {
            self.html.add_class(Self::SELECTION_CLASS);
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
        root.children.insert(idx, html! {
            <span class = class>{ tile }</span>
        });
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
            <label class = Self::TEXT_ITEM_CLASS>{ label }</label>
        };
        let root = self.root_tag_mut();

        if let Some(id) = root
            .find_child_tag_recursively("input")
            .and_then(|input| input.attr("id"))
        {
            label.set_attr("for", id.clone());
        }

        let idx = root
            .find_child_contains_class_idx(Self::LAST_TILE_CLASS)
            .unwrap_or_else(|| root.children.len());

        root.children.insert(idx, label);
        self
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }
}

impl MdcWidget for ListItem {
    const NAME: &'static str = stringify!(ListItem);

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
    pub const CLASS: &'static str = "mdc-deprecated-list";
    pub const TWO_LINE_CLASS: &'static str = "mdc-deprecated-list--two-line";
    pub const DENSE_CLASS: &'static str = "mdc-deprecated-list--dense";
    pub const AVATAR_LIST_CLASS: &'static str = "mdc-deprecated-list--avatar-list";
    pub const TEXTUAL_LIST_CLASS: &'static str = "mdc-deprecated-list--textual-list";
    pub const ICON_LIST_CLASS: &'static str = "mdc-deprecated-list--icon-list";
    pub const IMAGE_LIST_CLASS: &'static str = "mdc-deprecated-list--image-list";
    pub const THUMBNAIL_LIST_CLASS: &'static str = "mdc-deprecated-list--thumbnail-list";
    pub const VIDEO_LIST_CLASS: &'static str = "mdc-deprecated-list--video-list";
    pub const DIVIDER_CLASS: &'static str = "mdc-deprecated-list-divider";
    pub const DIVIDER_INSET_LEADING_CLASS: &'static str = "mdc-deprecated-list-divider--inset-leading";
    pub const DIVIDER_INSET_TRAILING_CLASS: &'static str = "mdc-deprecated-ist-divider--inset-trailing";
    pub const DIVIDER_INSET_PADDING_CLASS: &'static str = "mdc-deprecated-list-divider--inset-padding";
    pub const GROUP_SUBHEADER_CLASS: &'static str = "mdc-deprecated-list-group__subheader";
    const LIST_VAR_NAME: &'static str = "list";

    pub fn simple_ul() -> Self {
        Self {
            html: html! { <ul class = Self::CLASS></ul> },
        }
    }

    pub fn simple_nav() -> Self {
        Self {
            html: html! { <nav class = Self::CLASS></nav> },
        }
    }

    pub fn ul() -> Self {
        Self::simple_ul().attr(AUTO_INIT_ATTR, mdc::TYPE_NAME)
    }

    pub fn nav() -> Self {
        Self::simple_nav().attr(AUTO_INIT_ATTR, mdc::TYPE_NAME)
    }

    pub fn root_id(&self) -> &str {
        self.root_tag()
            .attr("id")
            .expect("The List widget must have ID")
            .as_ref()
    }

    pub fn single_selection(self) -> Self {
        let statement = format!("{}.{}.singleSelection = true;", Self::LIST_VAR_NAME, mdc::TYPE_NAME);
        self.add_script_statement(statement)
    }

    pub fn wrap_focus(self) -> Self {
        let statement = format!("{}.{}.wrapFocus = true;", Self::LIST_VAR_NAME, mdc::TYPE_NAME);
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
                    if ({list}.{mdc_type} === undefined) {{
                        window.mdc.autoInit({list}.parentElement);
                    }}
                    {statement}
                }}",
                list = Self::LIST_VAR_NAME,
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

    /// Modifier to style list with two lines (primary and secondary lines).
    pub fn two_line(self) -> Self {
        self.class(Self::TWO_LINE_CLASS)
    }

    /// Styles the density of the list, making it appear more compact.
    pub fn dense(self) -> Self {
        self.class(Self::DENSE_CLASS)
    }

    /// configures the leading tile of each row to display avatars.
    pub fn avatar(self) -> Self {
        self.class(Self::AVATAR_LIST_CLASS)
    }

    /// Configures lists that start with text (e.g., do not have a leading tile).
    pub fn textual(self) -> Self {
        self.class(Self::TEXTUAL_LIST_CLASS)
    }

    /// Configures the leading tile of each row to display icons.
    pub fn icon(self) -> Self {
        self.class(Self::ICON_LIST_CLASS)
    }

    /// Configures the leading tile of each row to display images.
    pub fn image(self) -> Self {
        self.class(Self::IMAGE_LIST_CLASS)
    }

    /// Configures the leading tile of each row to display smaller images (this is analogous to
    /// an avatar list but the image will not be rounded).
    pub fn thumbnail(self) -> Self {
        self.class(Self::THUMBNAIL_LIST_CLASS)
    }

    /// Configures the leading tile of each row to display videos.
    pub fn video(self) -> Self {
        self.class(Self::VIDEO_LIST_CLASS)
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
            <li role = "separator" class = Self::DIVIDER_CLASS></li>
        });
        self
    }

    /// Increases the leading margin of the divider so that it does not intersect
    /// the graphics column.
    pub fn divider_inset_leading(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = classes!(Self::DIVIDER_CLASS, Self::DIVIDER_INSET_LEADING_CLASS)></li>
        });
        self
    }

    /// Increases the trailing margin of the divider so that it coincides with the
    /// item's padding.
    pub fn divider_inset_trailing(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = classes!(Self::DIVIDER_CLASS, Self::DIVIDER_INSET_TRAILING_CLASS)></li>
        });
        self
    }

    /// Alters the inset to correspond to the item's padding rather than the leading
    /// graphics column.
    pub fn divider_inset_padding(mut self) -> Self {
        self.root_tag_mut().children.push(html! {
            <li role = "separator" class = classes!(Self::DIVIDER_CLASS, Self::DIVIDER_INSET_PADDING_CLASS)></li>
        });
        self
    }

    pub fn subheader(self, subheader: impl Into<Html>) -> Self {
        let mut subheader = subheader.into();
        subheader.add_class(Self::GROUP_SUBHEADER_CLASS);
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
    const NAME: &'static str = stringify!(List);

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
