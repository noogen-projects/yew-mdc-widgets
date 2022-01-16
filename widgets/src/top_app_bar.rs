use std::ops::{Deref, DerefMut};

use const_format::concatcp;
use yew::{html, virtual_dom::AttrValue, Callback, Event, Html};

use crate::{
    utils::{ManageChildren, VTagExt},
    MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    pub const TYPE_NAME: &str = "MDCTopAppBar";
}

#[derive(Debug, Clone)]
pub struct TopAppBar {
    html: Html,
}

impl Default for TopAppBar {
    fn default() -> Self {
        Self::new()
    }
}

impl TopAppBar {
    pub const VAR_NAME: &'static str = "top_app_bar";
    const SCROLLED_CLASS: &'static str = "mdc-top-app-bar--fixed-scrolled";

    /// Class used to style the content below the standard and fixed top app bar to prevent the top
    /// app bar from covering it
    pub const FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--fixed-adjust";

    /// Class used to style the content below the prominent top app bar to prevent the top app bar
    /// from covering it
    pub const PROMINENT_FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--prominent-fixed-adjust";

    /// Class used to style the content below the dense top app bar to prevent the top app bar from
    /// covering it
    pub const DENSE_FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--dense-fixed-adjust";

    /// Class used to style the content below the top app bar when styled as both dense and
    /// prominent, to prevent the top app bar from covering it
    pub const DENSE_PROMINENT_FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--dense-prominent-fixed-adjust";

    pub fn new() -> Self {
        let mut topappbar = Self {
            html: html! {
                <header class = "mdc-top-app-bar">
                    <div class = "mdc-top-app-bar__row">
                        <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                            <span class = "mdc-top-app-bar__title"></span>
                        </section>
                        <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-end" role = "toolbar">
                        </section>
                    </div>
                </header>
            },
        };
        topappbar.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        topappbar
    }

    pub fn title(mut self, title: impl Into<Html>) -> Self {
        if let Some(row) = self.root_tag_mut().first_child_tag_mut() {
            if let Some(start_section) = row.first_child_tag_mut() {
                if let Some(title_tag) = start_section.find_child_contains_class_mut("mdc-top-app-bar__title") {
                    title_tag.clear_children();
                    title_tag.add_child(title.into());
                }
            }
        }
        self
    }

    pub fn navigation_item(mut self, item: impl Into<Html>) -> Self {
        let mut item = item.into();
        item.add_class("mdc-top-app-bar__navigation-icon");

        if let Some(row) = self.root_tag_mut().first_child_tag_mut() {
            if let Some(start_section) = row.first_child_tag_mut() {
                let idx = start_section.children().len() - 1;
                start_section.insert_child(idx, item);
            }
        }
        self
    }

    pub fn start_section_item(mut self, item: impl Into<Html>) -> Self {
        if let Some(row) = self.root_tag_mut().first_child_tag_mut() {
            if let Some(start_section) = row.first_child_tag_mut() {
                start_section.add_child(item.into());
            }
        }
        self
    }

    pub fn middle_section(mut self, content: impl Into<Html>) -> Self {
        if let Some(row) = self.root_tag_mut().first_child_tag_mut() {
            let idx = row.children().len() - 1;
            row.insert_child(
                idx,
                html! { <section class = "mdc-top-app-bar__section">{ content }</section> },
            );
        }
        self
    }

    pub fn action_item(mut self, item: impl Into<Html>) -> Self {
        let mut item = item.into();
        item.add_class("mdc-top-app-bar__action-item");

        if let Some(row) = self.root_tag_mut().first_child_tag_mut() {
            if let Some(end_section) = row.last_child_tag_mut() {
                end_section.add_child(item);
            }
        }
        self
    }

    pub fn root_id(&self) -> AttrValue {
        self.root_tag().attr("id").expect("The TopAppBar widget must have ID")
    }

    pub fn shadow_when_scroll_script(&self, factory: impl AsRef<str>) -> String {
        let id = self.root_id();
        format!(
            r#"{{
                const obj = {factory};
                const old_scroll = obj.onscroll;
                obj.onscroll = function() {{
                    if (old_scroll && {{}}.toString.call(old_scroll) === '[object Function]') {{ old_scroll(); }}

                    const obj = {factory};
                    const bar = document.getElementById('{id}');
                    if (obj.pageYOffset > 0 || obj.scrollTop > 0) {{
                        bar.classList.add("{class}");
                    }} else {{
                        bar.classList.remove("{class}");
                    }}
                }}
            }}"#,
            factory = factory.as_ref(),
            id = id,
            class = Self::SCROLLED_CLASS
        )
    }

    pub fn enable_shadow_when_scroll_window(self) -> Self {
        self.enable_shadow_when_scroll("window")
    }

    pub fn enable_shadow_when_scroll(mut self, factory: impl AsRef<str>) -> Self {
        let script = self.shadow_when_scroll_script(factory);
        let root = self.root_tag_mut();
        if root.is_contains_class("mdc-top-app-bar") && !root.is_contains_class(Self::SCROLLED_CLASS) {
            self.add_script_statement(script)
        } else {
            self
        }
    }

    pub fn add_script_statement(mut self, statement: String) -> Self {
        if self.html.find_child_tag("script").is_some() {
            self.html.add_child_script_statement(statement);
        } else {
            let id = self.root_id();
            let script = format!(
                r"{{
                    const {bar} = document.getElementById('{id}');
                    if ({bar}.MDCTopAppBar === undefined) {{
                        window.mdc.autoInit({bar}.parentElement);
                    }}
                    {statement}
                }}",
                bar = Self::VAR_NAME,
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

    /// Style the top app bar as a fixed top app bar
    pub fn fixed(self) -> Self {
        self.class("mdc-top-app-bar--fixed")
    }

    /// Style the top app bar as a prominent top app bar
    pub fn prominent(self) -> Self {
        self.class("mdc-top-app-bar--prominent")
    }

    /// Style the top app bar as a dense top app bar
    pub fn dense(self) -> Self {
        self.class("mdc-top-app-bar--dense")
    }

    pub fn on_navigation(self, callback: impl Into<Callback<Event>>) -> Self {
        self.on_event(concatcp!(mdc::TYPE_NAME, ":nav"), callback)
    }
}

impl MdcWidget for TopAppBar {
    const NAME: &'static str = stringify!(TopAppBar);

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
    fn from(widget: TopAppBar) -> Self {
        widget.html
    }
}
