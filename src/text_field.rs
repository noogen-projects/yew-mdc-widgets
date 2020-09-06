use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use yew::{
    html,
    html::{onclick, oninput},
    Callback, Html, InputData, MouseEvent,
};

use crate::{
    utils::{MdcWidget, VTagExt},
    Text,
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
        let text_field = Self {
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
        text_field.root_tag_mut().children.insert(
            1,
            html! {
                <span class="mdc-notched-outline">
                    <span class="mdc-notched-outline__leading"></span>
                    <span class="mdc-notched-outline__notch"></span>
                    <span class="mdc-notched-outline__trailing"></span>
                </span>
            },
        );
        text_field.class(TextFieldStyle::Outlined.class())
    }

    pub fn fullwidth<'a>(id: impl Into<Text<'a>>) -> Self {
        let text_field = Self {
            html: Self::base_html(id),
            style: TextFieldStyle::FilledFullWidth,
        };
        text_field.ripple(true).class(TextFieldStyle::FilledFullWidth.class())
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if self.style != TextFieldStyle::Outlined {
            if enabled {
                if !self.root_tag().is_some_child_contains_class("mdc-text-field__ripple") {
                    self.root_tag_mut().children.insert(
                        0,
                        html! {
                            <span class = "mdc-text-field__ripple"></span>
                        },
                    );
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

    pub fn label(mut self, label: impl Into<Html>) -> Self {
        let id = self.root_id();
        let label_id = format!("{}-label", id);

        match self.style {
            TextFieldStyle::Filled => {
                let idx = self
                    .root_tag()
                    .find_child_tag_idx("input")
                    .map(|idx| idx + 1)
                    .unwrap_or(0);
                self.root_tag_mut().children.insert(
                    idx,
                    html! {
                        <span class = "mdc-floating-label" id = label_id>{ label }</span>
                    },
                );
                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    input_tag.set_attr("aria-labelledby", label_id);
                }
            }
            TextFieldStyle::Outlined => {
                if let Some(tag) = self.root_tag_mut().find_child_contains_class_mut("mdc-notched-outline") {
                    if let Some(notch) = tag.find_child_contains_class_mut("mdc-notched-outline__notch") {
                        notch.children.push(html! {
                            <span class = "mdc-floating-label" id = label_id>{ label }</span>
                        });
                    }
                }

                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    input_tag.set_attr("aria-labelledby", label_id);
                }
            }
            TextFieldStyle::FilledFullWidth => {
                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    if let Html::VText(label) = label.into() {
                        input_tag.set_attr("placeholder", &label.text);
                        input_tag.set_attr("aria-label", label.text);
                    }
                }
            }
        }
        self
    }

    pub fn disabled(mut self) -> Self {
        self.add_class("mdc-text-field--disabled");
        if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
            input_tag.set_attr("disabled", "");
        }
        self
    }

    pub fn helper_text(mut self, helper_text: impl Into<Html>) -> Self {
        let id = self.root_id();
        let helper_id = format!("{}-helper", id);

        if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
            input_tag.set_attr("aria-controls", helper_id.clone());
            input_tag.set_attr("aria-describedby", helper_id.clone());
        }
        self.html_mut().insert_child(
            1,
            html! {
                <div class="mdc-text-field-helper-line">
                    <div class="mdc-text-field-helper-text" id=helper_id aria-hidden="true">{helper_text}</div>
                </div>
            },
        );
        // match self.style {
        //     TextFieldStyle::Filled => {
        //         if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
        //             input_tag.set_attr("aria-controls", helper_id.clone());
        //             input_tag.set_attr("aria-describedby", helper_id.clone());
        //         }
        //         self.children.push(html! {
        //             <div class="mdc-text-field-helper-line">
        //                 <div class="mdc-text-field-helper-text" id=helper_id aria-hidden="true">{helper_text}</div>
        //             </div>
        //         });
        //     },
        //     TextFieldStyle::Outlined => {
        //         if let Some(tag) = self.root_tag_mut().find_child_contains_class_mut("mdc-notched-outline") {
        //             if let Some(notch) = tag.find_child_contains_class_mut("mdc-notched-outline__notch") {
        //                 notch.children.push(html! {
        //                     <span class = "mdc-floating-label" id = label_id>{ label }</span>
        //                 });
        //             }
        //         }
        //
        //         if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
        //             input_tag.set_attr("aria-labelledby", label_id);
        //         }
        //     },
        //     TextFieldStyle::FilledFullWidth => {
        //         if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
        //             if let Html::VText(label) = label.into() {
        //                 input_tag.set_attr("placeholder", &label.text);
        //                 input_tag.set_attr("aria-label", label.text);
        //             }
        //         }
        //     },
        // }
        self
    }

    pub fn root_id(&self) -> &str {
        self.root_tag()
            .attributes
            .get("id")
            .expect("The TextField widget must have ID")
    }

    pub fn on_click(self, callback: Callback<MouseEvent>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback)))
    }

    pub fn on_input(mut self, callback: Callback<InputData>) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_recursively_mut("input") {
            input.add_listener(Rc::new(oninput::Wrapper::new(callback)));
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
