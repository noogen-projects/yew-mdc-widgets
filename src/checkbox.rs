use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{html, html::onclick, Callback, Html, MouseEvent};

use crate::{
    utils::{ripple, root_and_input_child_disabled, MdcWidget, VTagExt},
    Text,
};

#[derive(Debug, Clone)]
pub struct Checkbox {
    html: Html,
    input_id: String,
}

impl Checkbox {
    const RIPPLE_CLASS: &'static str = "mdc-checkbox__ripple";

    pub fn new<'a>(id: impl Into<Text<'a>>) -> Self {
        let id = id.into();
        let input_id = format!("{}-input", id);
        Self {
            html: html! {
                <div id = id class = "mdc-checkbox">
                    <input type = "checkbox" id = input_id class = "mdc-checkbox__native-control" />
                    <div class = "mdc-checkbox__background">
                        <svg class = "mdc-checkbox__checkmark" viewBox = "0 0 24 24">
                            <path class = "mdc-checkbox__checkmark-path" fill = "none" d = "M1.73,12.91 8.1,19.28 22.79,4.59" />
                        </svg>
                        <div class = "mdc-checkbox__mixedmark"></div>
                    </div>
                    <div class = Self::RIPPLE_CLASS></div>
                    <script>{ format!("mdc.checkbox.MDCCheckbox.attachTo(document.getElementById('{}'));", id) }</script>
                </div>
            },
            input_id,
        }
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

    pub fn labeled_by(mut self, labeled_by: impl Into<String>) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            input.set_attr("aria-labelledby", labeled_by);
        }
        self
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        ripple(&mut self, Self::RIPPLE_CLASS, enabled);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        root_and_input_child_disabled(&mut self, "mdc-checkbox--disabled", disabled);
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            if indeterminate {
                input.set_attr("data-indeterminate", "true");
            } else {
                input.remove_attr("data-indeterminate");
            }
        }
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

    pub fn markup_only(mut self) -> Self {
        let root = self.root_tag_mut();
        root.remove_child_contains_class(Self::RIPPLE_CLASS);
        root.remove_child_tag("script");
        self
    }
}

impl MdcWidget for Checkbox {
    const NAME: &'static str = "Checkbox";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Checkbox {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Checkbox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Checkbox> for Html {
    fn from(checkbox: Checkbox) -> Self {
        checkbox.html
    }
}
