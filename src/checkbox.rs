use std::{
    rc::Rc, ops::{Deref, DerefMut},
};

use yew::{html, html::onclick, Callback, Html, MouseEvent, virtual_dom::VTag};

use crate::{
    Text,
    utils::VTagExt,
};

#[derive(Debug, Clone)]
pub struct Checkbox {
    html: Html,
    input_id: String,
}

impl Checkbox {
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
                    <div class = "mdc-checkbox__ripple"></div>
                    <script>{ format!("mdc.checkbox.MDCCheckbox.attachTo(document.getElementById('{}'));", id) }</script>
                </div>
            },
            input_id,
        }
    }

    pub fn label<'a>(mut self, label: impl Into<Text<'a>>) -> Self {
        if let Html::VTag(_) = &self.html {
            self.html = html! { <>{ self.html }</> }
        }
        if let Html::VList(list) = &mut self.html {
            list.children.insert(1, html! {
                <label for = self.input_id>{ label.into() }</label>
            });
        }
        self
    }

    pub fn labeled_by<'a>(mut self, labeled_by: impl Into<String>) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            input.attributes.insert("aria-labelledby".into(), labeled_by.into());
        }
        self
    }

    pub fn ripple(mut self, ripple: bool) -> Self {
        let ripple_class = "mdc-checkbox__ripple";
        if ripple {
            if !self.root_tag().is_some_child_contains_class(ripple_class) {
                let idx = self.root_tag().children.len() - 1;
                self.root_tag_mut().children.insert(idx, html! {
                    <div class = ripple_class></div>
                });
            }
        } else {
            if let Some(idx) = self.root_tag().find_child_contains_class(ripple_class) {
                self.root_tag_mut().children.remove(idx);
            }
        }
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.root_tag_mut().add_class("mdc-checkbox--disabled");
        } else {
            self.root_tag_mut().remove_any_class(&["mdc-checkbox--disabled"]);
        }

        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            if disabled {
                input.attributes.insert("disabled".into(), "disabled".into());
            } else {
                input.attributes.remove("disabled");
            }
        }
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_mut("input") {
            if indeterminate {
                input.attributes.insert("data-indeterminate".into(), "true".into());
            } else {
                input.attributes.remove("data-indeterminate");
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

    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    fn root_tag(&self) -> &VTag {
        match &self.html {
            Html::VTag(tag) => return tag,
            Html::VList(list) => if let Some(Html::VTag(tag)) = list.children.first() {
                return tag;
            },
            _ => (),
        }
        panic!("The root checkbox element must be a tag!");
    }

    fn root_tag_mut(&mut self) -> &mut VTag {
        match &mut self.html {
            Html::VTag(tag) => return tag,
            Html::VList(list) => if let Some(Html::VTag(tag)) = list.children.first_mut() {
                return tag;
            },
            _ => (),
        }
        panic!("The root checkbox element must be a tag!");
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