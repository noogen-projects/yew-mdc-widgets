use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use const_format::concatcp;
use yew::{classes, html, html::onclick, virtual_dom::AttrValue, Callback, Html, MouseEvent};

use crate::{
    dom,
    utils::{ManageChildren, VTagExt},
    CustomEvent, Element, MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    use wasm_bindgen::prelude::*;

    use crate::Element;

    pub const TYPE_NAME: &str = "MDCTab";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCTab, js_namespace = ["mdc", "tab"])]
        pub type Tab;

        #[wasm_bindgen(constructor, js_class = MDCTab, js_namespace = ["mdc", "tab"])]
        pub fn new(element: Element) -> Tab;

        #[wasm_bindgen(method, setter)]
        pub fn set_focus_on_activate(this: &Tab, focus: bool);

        #[wasm_bindgen(method)]
        pub fn activate(this: &Tab);

        #[wasm_bindgen(method)]
        pub fn deactivate(this: &Tab);
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    html: Html,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    ///
    pub const CLASS: &'static str = "mdc-tab";

    /// Container of tab icon, text label and tab indicator.
    pub const CONTENT_CLASS: &'static str = "mdc-tab__content";

    /// Denotes the ripple surface for the tab.
    pub const RIPPLE_CLASS: &'static str = "mdc-tab__ripple";

    /// Indicates that the tab is active.
    pub const ACTIVE_CLASS: &'static str = "mdc-tab--active";

    pub const INDICATOR_CLASS: &'static str = "mdc-tab-indicator";

    pub const INDICATOR_ACTIVE_CLASS: &'static str = "mdc-tab-indicator--active";

    pub const INDICATOR_CONTENT_CLASS: &'static str = "mdc-tab-indicator__content";

    pub const INDICATOR_CONTENT_UNDERLINE_CLASS: &'static str = "mdc-tab-indicator__content--underline";

    /// Indicates that the tab icon and label should flow vertically instead of horizontally.
    pub const STACKED_CLASS: &'static str = "mdc-tab--stacked";

    /// Indicates that the tab should shrink in size to be as narrow as possible without causing
    /// text to wrap.
    pub const MIN_WIDTH_CLASS: &'static str = "mdc-tab--min-width";

    /// Indicates the text label of the tab.
    pub const LABEL_CLASS: &'static str = "mdc-tab__text-label";

    /// Indicates a leading icon in the tab.
    pub const ICON_CLASS: &'static str = "mdc-tab__icon";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <button class = { Self::CLASS } role = "tab" aria-selected = "false" tabindex = "-1">
                    <span class = { Self::CONTENT_CLASS }></span>
                    <span class = { Self::RIPPLE_CLASS }></span>
                </button>
            },
        }
    }

    pub fn new() -> Self {
        Self::simple().indicator()
    }

    pub fn activate(id: impl AsRef<str>) {
        let tab = mdc::Tab::new(dom::existing::get_element_by_id::<Element>(id.as_ref()));
        tab.activate();
    }

    pub fn deactivate(id: impl AsRef<str>) {
        let tab = mdc::Tab::new(dom::existing::get_element_by_id::<Element>(id.as_ref()));
        tab.deactivate();
    }

    pub fn indicator(mut self) -> Self {
        self.insert_child(1, html! {
            <span class = { Self::INDICATOR_CLASS }>
                <span class = { classes!(Self::INDICATOR_CONTENT_CLASS, Self::INDICATOR_CONTENT_UNDERLINE_CLASS) }></span>
            </span>
        });
        self
    }

    pub fn content_indicator(mut self) -> Self {
        if let Some(content) = self.find_child_tag_mut(Self::CONTENT_CLASS) {
            content.add_child(html! {
                <span class = { Self::INDICATOR_CLASS }>
                    <span class = { classes!(Self::INDICATOR_CONTENT_CLASS, Self::INDICATOR_CONTENT_UNDERLINE_CLASS) }></span>
                </span>
            });
        }
        self
    }

    pub fn tab_index(mut self, index: isize) -> Self {
        self.set_attr("tabindex", format!("{}", index));
        self
    }

    pub fn active(mut self) -> Self {
        let root = self.root_tag_mut();
        root.add_class_if_needed(Self::ACTIVE_CLASS);
        if let Some(indicator) = root.find_child_contains_class_mut(Self::INDICATOR_CLASS) {
            indicator.add_class_if_needed(Self::INDICATOR_ACTIVE_CLASS);
        }
        self
    }

    pub fn stacked(mut self) -> Self {
        let root = self.root_tag_mut();
        root.add_class_if_needed(Self::STACKED_CLASS);
        self
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        let root = self.root_tag_mut();
        if let Some(content) = root.find_child_contains_class_mut(Self::CONTENT_CLASS) {
            content.add_child(html! {
                <span class = { classes!(Self::ICON_CLASS, "material-icons") } aria-hidden = "true">{ name.into() }</span>
            });
        }
        self
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        if let Some(content) = root.find_child_contains_class_mut(Self::CONTENT_CLASS) {
            content.add_child(html! {
                <span class = { Self::LABEL_CLASS }>{ label }</span>
            });
        }
        self
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }

    /// Emitted when the Tab is interacted with, regardless of its active state.
    /// Used by parent components to know which Tab to activate.
    /// event.detail: `{"tabId": string}`
    pub fn on_interaction(self, callback: impl Into<Callback<CustomEvent>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":interacted"), callback)
    }
}

impl MdcWidget for Tab {
    const NAME: &'static str = stringify!(Tab);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Tab {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Tab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Tab> for Html {
    fn from(widget: Tab) -> Self {
        widget.html
    }
}

pub mod bar {
    pub mod mdc {
        pub const TYPE_NAME: &str = "MDCTabBar";
    }
}

#[derive(Debug, Clone)]
pub struct TabBar {
    html: Html,
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

impl TabBar {
    pub const CLASS: &'static str = "mdc-tab-bar";

    pub const SCROLLER_CLASS: &'static str = "mdc-tab-scroller";

    pub const SCROLL_AREA_CLASS: &'static str = "mdc-tab-scroller__scroll-area";

    pub const SCROLL_CONTENT_CLASS: &'static str = "mdc-tab-scroller__scroll-content";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class = { Self::CLASS } role = "tablist">
                    <div class = { Self::SCROLLER_CLASS }>
                        <div class = { Self::SCROLL_AREA_CLASS }>
                            <div class = { Self::SCROLL_CONTENT_CLASS }></div>
                        </div>
                    </div>
                </div>
            },
        }
    }

    pub fn new() -> Self {
        let mut tab_bar = Self::simple();
        tab_bar.root_tag_mut().set_attr(AUTO_INIT_ATTR, bar::mdc::TYPE_NAME);
        tab_bar
    }

    pub fn tab(mut self, tab: impl Into<Html>) -> Self {
        self.find_child_contains_class_recursively_mut(Self::SCROLL_CONTENT_CLASS)
            .expect("Tab should contains the scroll content")
            .add_child(tab.into());
        self
    }

    pub fn tabs(mut self, tabs: impl IntoIterator<Item = impl Into<Html>>) -> Self {
        for tab in tabs {
            self = self.tab(tab);
        }
        self
    }

    #[track_caller]
    pub fn root_id(&self) -> AttrValue {
        self.root_tag().attr("id").expect("The TabBar widget must have ID")
    }

    /// Emitted when a Tab is activated with the index of the activated Tab.
    /// Listen for this to update content when a Tab becomes active.
    /// event.detail: '{"index": number}'
    pub fn on_activated(self, callback: impl Into<Callback<CustomEvent>>) -> Self {
        self.on_event(concatcp!(bar::mdc::TYPE_NAME, ":activated"), callback)
    }
}

impl MdcWidget for TabBar {
    const NAME: &'static str = stringify!(TabBar);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }

    fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        let root = self.root_tag_mut();
        root.set_attr("id", id);
        self
    }
}

impl Deref for TabBar {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for TabBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<TabBar> for Html {
    fn from(widget: TabBar) -> Self {
        widget.html
    }
}
