use std::ops::{Deref, DerefMut};

use yew::{html, services::ConsoleService, Callback, Html, MouseEvent};

use crate::{
    utils::{labeled_on_click, root_and_input_child_disabled, MdcWidget, VTagExt},
    AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct Switch {
    html: Html,
}

impl Switch {
    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class = "mdc-switch">
                    <div class = "mdc-switch__track"></div>
                    <div class = "mdc-switch__thumb-underlay">
                        <div class = "mdc-switch__thumb"></div>
                        <input type = "checkbox" class = "mdc-switch__native-control" role = "switch" aria-checked = "false" />
                    </div>
                </div>
            },
        }
    }

    pub fn new() -> Self {
        let mut switch = Self::simple();
        switch.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCSwitch");
        switch
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        if let Some(input_id) = self
            .root_tag()
            .find_child_tag_recursively("input")
            .and_then(|input| input.attributes.get("id"))
        {
            let label = html! {
                <label for = input_id>{ label }</label>
            };
            if let Html::VTag(_) = &self.html {
                self.html = html! { <>{ self.html }</> }
            }
            if let Html::VList(list) = &mut self.html {
                list.children.insert(1, label);
            }
        } else {
            ConsoleService::error("Could not find input tag's id attribute");
        }
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        root_and_input_child_disabled(&mut self, "mdc-switch--disabled", disabled);
        self
    }

    pub fn on(mut self) -> Self {
        let root = self.root_tag_mut();
        root.add_class("mdc-switch--checked");
        if let Some(input) = root.find_child_tag_recursively_mut("input") {
            input.checked = true;
            input.set_attr("aria-checked", "true");
        }
        self
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        labeled_on_click(&mut self, callback);
        self
    }
}

impl MdcWidget for Switch {
    const NAME: &'static str = "Switch";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }

    fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        let input_id = format!("{}-input", id);

        let root = self.root_tag_mut();
        root.set_attr("id", id);
        if let Some(input) = root.find_child_tag_recursively_mut("input") {
            input.set_attr("id", input_id);
        };
        self
    }
}

impl Deref for Switch {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Switch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Switch> for Html {
    fn from(widget: Switch) -> Self {
        widget.html
    }
}
