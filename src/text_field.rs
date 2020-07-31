use std::ops::{Deref, DerefMut};

use yew::{html, Html, virtual_dom::VTag};

use crate::{
    Text,
    utils::{VTagExt, MdcWidget},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextFieldStyle {
    Filled,
    Outlined,
    FilledFullWidth,
}

impl TextFieldStyle {
    pub fn class(&self) -> &'static str {
        match self {
            TextFieldStyle::Filled => "mdc-text-field--filled",
            TextFieldStyle::Outlined => "mdc-text-field--outlined",
            TextFieldStyle::FilledFullWidth => "mdc-text-field--filled mdc-text-field--fullwidth",
        }
    }

    pub fn classes() -> [&'static str; 3] {
        [
            TextFieldStyle::Filled.class(),
            TextFieldStyle::Outlined.class(),
            TextFieldStyle::FilledFullWidth.class(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct TextField {
    html: Html,
    style: TextFieldStyle,
}

impl TextField {
    fn base_html<'a>(id: impl Into<Text<'a>>) -> Html {
        let id = id.into();
        let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", id);
        html! {
                <>
                    <label id = id class = "mdc-text-field">
                        <input class = "mdc-text-field__input" type = "text"/>
                    </label>
                    <script>{ mdc_init }</script>
                </>
            }
    }
    pub fn filled<'a>(id: impl Into<Text<'a>>) -> Self {
        let mut text_field = Self {
            html: Self::base_html(id),
            style: TextFieldStyle::Filled,
        };
        text_field.ripple(true).class(TextFieldStyle::Filled.class())
    }

    pub fn outlined<'a>(id: impl Into<Text<'a>>) -> Self {
        let mut text_field = Self {
            html: Self::base_html(id),
            style: TextFieldStyle::Outlined,
        };
        text_field.root_tag_mut().children.insert(1, html! {
            <span class="mdc-notched-outline">
                <span class="mdc-notched-outline__leading"></span>
                <span class="mdc-notched-outline__notch"></span>
                <span class="mdc-notched-outline__trailing"></span>
            </span>
        });
        text_field.class(TextFieldStyle::Outlined.class())
    }

    pub fn fullwidth<'a>(id: impl Into<Text<'a>>) -> Self {
        let mut text_field = Self {
            html: Self::base_html(id),
            style: TextFieldStyle::FilledFullWidth,
        };
        text_field.ripple(true).class(TextFieldStyle::FilledFullWidth.class())
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if self.style!= TextFieldStyle::Outlined {
            if enabled {
                if !self.root_tag().is_some_child_contains_class("mdc-text-field__ripple") {
                    self.root_tag_mut().children.insert(0, html! {
                        <span class = "mdc-text-field__ripple"></span>
                    });
                }
                if !self.root_tag().is_some_child_contains_class("mdc-line-ripple") {
                    self.root_tag_mut().children.push(html! {
                        <span class = "mdc-line-ripple"></span>
                    });
                }
            } else {
                if let Some(idx) = self.root_tag().find_child_contains_class_idx("mdc-text-field__ripple") {
                    self.root_tag_mut().children.remove(idx);
                }
                if let Some(idx) = self.root_tag().find_child_contains_class_idx("mdc-line-ripple") {
                    self.root_tag_mut().children.remove(idx);
                }
            }
        }
        self
    }
    pub fn class(mut self, class: impl AsRef<str>) -> Self {
        self.root_tag_mut().add_class(class);
        self
    }

    pub fn label<'a>(mut self, label: impl Into<Text<'a>>) -> Self {
        let label = label.into();
        let id = self.root_tag().attributes.get("id").expect("");
        let label_id = format!("{}-label", id.clone());

        match self.style {
            TextFieldStyle::Filled => {
                let idx = self.root_tag().find_child_tag_idx("input").map(|idx| idx + 1).unwrap_or(0);
                self.root_tag_mut().children.insert( idx, html! {
                    <span class = "mdc-floating-label" id = label_id>{ label }</span>
                });
                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    input_tag.attributes.insert("aria-labelledby".to_string(), label_id);
                }
            },
            TextFieldStyle::Outlined => {
                if let Some(tag) = self.root_tag_mut().find_child_contains_class_mut("mdc-notched-outline") {
                    if let Some(notch) = tag.find_child_contains_class_mut("mdc-notched-outline__notch") {
                        notch.children.push(html! {
                            <span class = "mdc-floating-label" id = label_id>{ label }</span>
                        });
                    }
                }

                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    input_tag.attributes.insert("aria-labelledby".to_string(), label_id);
                }
            },
            TextFieldStyle::FilledFullWidth => {
                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    input_tag.attributes.insert("placeholder".to_string(), label.clone().to_string());
                    input_tag.attributes.insert("aria-label".to_string(), label.to_string());
                }
            },
        }
        self
    }
}

impl MdcWidget for TextField {
    const NAME: &'static str = "TextField";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for TextField {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for TextField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<TextField> for Html {
    fn from(text_field: TextField) -> Self {
        text_field.html
    }
}