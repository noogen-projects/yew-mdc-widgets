use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, services::ConsoleService, Callback, Html, MouseEvent};

use crate::{
    utils::{ripple_element, root_and_input_child_disabled, MdcWidget, VTagExt},
    Text, AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct Radio {
    html: Html,
}

impl Radio {
    const RIPPLE_CLASS: &'static str = "mdc-radio__ripple";

    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class="mdc-radio">
                    <input type = "radio" class = "mdc-radio__native-control" />
                    <div class = "mdc-radio__background">
                        <div class = "mdc-radio__outer-circle"></div>
                        <div class = "mdc-radio__inner-circle"></div>
                    </div>
                    <div class = Self::RIPPLE_CLASS></div>
                </div>
            },
        }
    }

    pub fn new() -> Self {
        let mut radio = Self::simple();
        radio.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCRadio");
        radio
    }

    pub fn id<'a>(mut self, id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        let input_id = format!("{}-input", id);
        let root = self.root_tag_mut();
        root.set_attr("id", id);
        if let Some(input) = root.find_child_tag_mut("input") {
            input.set_attr("id", input_id);
        };
        self
    }

    pub fn name_of_set(mut self, name: impl Into<String>) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            input.set_attr("name", name);
        }
        self
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        if let Some(input_id) = self
            .root_tag()
            .find_child_tag("input")
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

    pub fn ripple(mut self, enabled: bool) -> Self {
        ripple_element(&mut self, Self::RIPPLE_CLASS, enabled);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        root_and_input_child_disabled(&mut self, "mdc-radio--disabled", disabled);
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            input.checked = checked;
        }
        self
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        let listener = Rc::new(onclick::Wrapper::new(callback));
        let root = self.root_tag_mut();
        if let Some(label) = root.find_child_tag_mut("label") {
            label.add_listener(listener.clone());
        }
        root.add_listener(listener);
        self
    }
}

impl MdcWidget for Radio {
    const NAME: &'static str = "Radio";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Radio {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Radio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Radio> for Html {
    fn from(widget: Radio) -> Self {
        widget.html
    }
}
