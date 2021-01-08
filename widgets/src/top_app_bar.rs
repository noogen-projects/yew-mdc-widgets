use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{
    utils::{MdcWidget, VTagExt},
    AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct TopAppBar {
    html: Html,
}

impl TopAppBar {
    pub const VAR_NAME: &'static str = "top_app_bar";
    const SCROLLED_CLASS: &'static str = "mdc-top-app-bar--fixed-scrolled";

    /// Class used to style the content below the standard and fixed top app bar to prevent the top app bar from
    /// covering it
    pub const FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--fixed-adjust";

    /// Class used to style the content below the prominent top app bar to prevent the top app bar from covering it
    pub const PROMINENT_FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--prominent-fixed-adjust";

    /// Class used to style the content below the dense top app bar to prevent the top app bar from covering it
    pub const DENSE_FIXED_ADJUST_CLASS: &'static str = "mdc-top-app-bar--dense-fixed-adjust";

    /// Class used to style the content below the top app bar when styled as both dense and prominent, to prevent the
    /// top app bar from covering it
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
        topappbar.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCTopAppBar");
        topappbar
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

    pub fn start_section_item(mut self, item: impl Into<Html>) -> Self {
        if let Some(Html::VTag(row)) = self.root_tag_mut().children.first_mut() {
            if let Some(Html::VTag(start_section)) = row.children.first_mut() {
                start_section.children.push(item.into());
            }
        }
        self
    }

    pub fn middle_section(mut self, content: impl Into<Html>) -> Self {
        if let Some(Html::VTag(row)) = self.root_tag_mut().children.first_mut() {
            let idx = row.children.len() - 1;
            row.children.insert(
                idx,
                html! { <section class = "mdc-top-app-bar__section">{ content }</section> },
            );
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

    pub fn root_id(&self) -> &str {
        self.root_tag()
            .attributes
            .get("id")
            .expect("The TopAppBar widget must have ID")
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

    pub fn add_navigation_event(self, script: impl AsRef<str>) -> Self {
        let statement = format!(
            "{}.MDCTopAppBar.listen('MDCTopAppBar:nav', () => {{ {} }});",
            Self::VAR_NAME,
            script.as_ref()
        );
        self.add_script_statement(statement)
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
    fn from(widget: TopAppBar) -> Self {
        widget.html
    }
}
