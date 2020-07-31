use std::{
    rc::Rc, ops::{Deref, DerefMut},
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    Text,
    utils::{VTagExt, MdcWidget, ripple, root_and_input_child_disabled},
};

pub struct Radio {
    html: Html,
    input_id: String,
}

impl Radio {
    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        let input_id = format!("{}-input", id);
        Self {
            html: html! {
                <div id = id class="mdc-radio">
                    <input type = "radio" id = input_id class = "mdc-radio__native-control" />
                    <div class = "mdc-radio__background">
                        <div class = "mdc-radio__outer-circle"></div>
                        <div class = "mdc-radio__inner-circle"></div>
                    </div>
                    <div class = "mdc-radio__ripple"></div>
                    <script>{ format!("mdc.radio.MDCRadio.attachTo(document.getElementById('{}'));", id) }</script>
                </div>
            },
            input_id,
        }
    }

    pub fn name_of_set<'a>(mut self, name: impl Into<String>) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            input.set_attr("name", name);
        }
        self
    }

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        if let Html::VTag(_) = &self.html {
            self.html = html! { <>{ self.html }</> }
        }
        if let Html::VList(list) = &mut self.html {
            list.children.insert(1, html! {
                <label for = self.input_id>{ label }</label>
            });
        }
        self
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        ripple(&mut self, "mdc-radio__ripple", enabled);
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

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        let listener = Rc::new(onclick::Wrapper::new(callback));
        if let Some(label) = self.html_mut().find_child_tag_mut("label") {
            label.add_listener(listener.clone());
        }
        self.root_tag_mut().add_listener(listener);
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
    fn from(radio: Radio) -> Self {
        radio.html
    }
}