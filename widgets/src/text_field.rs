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
    utils::{dom, IntoWidgetWithVList, VTagExt},
    Element, MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    use wasm_bindgen::prelude::*;

    use crate::Element;

    pub const TYPE_NAME: &str = "MDCTextField";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCTextField, js_namespace = ["mdc", "textField"])]
        pub type TextField;

        #[wasm_bindgen(constructor, js_class = MDCTextField, js_namespace = ["mdc", "textField"])]
        pub fn new(element: Element) -> TextField;

        /// Returns the input's value.
        #[wasm_bindgen(method, getter)]
        pub fn value(this: &TextField) -> String;

        /// Sets the input's value.
        #[wasm_bindgen(method, setter)]
        pub fn set_value(this: &TextField, value: &str);

        /// Returns whether or not the input is disabled.
        #[wasm_bindgen(method, getter)]
        pub fn disabled(this: &TextField) -> bool;

        /// Updates the input's disabled state.
        #[wasm_bindgen(method, setter)]
        pub fn set_disabled(this: &TextField, disabled: bool);

        /// Focuses the `input` or `textarea` element.
        #[wasm_bindgen(method)]
        pub fn focus(this: &TextField);

        /// Adjusts the dimensions and positions for all sub-elements.
        #[wasm_bindgen(method)]
        pub fn layout(this: &TextField);
    }
}

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
    fn base_html() -> Html {
        let mut html = html! {
            <label class = "mdc-text-field">
                <input class = "mdc-text-field__input" type = "text"/>
            </label>
        };
        html.root_tag_mut().unwrap().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        html
    }

    pub fn filled() -> Self {
        let text_field = Self {
            html: Self::base_html(),
            style: TextFieldStyle::Filled,
        };
        text_field.ripple(true).class(TextFieldStyle::Filled.class())
    }

    pub fn outlined() -> Self {
        let mut text_field = Self {
            html: Self::base_html(),
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

    pub fn fullwidth() -> Self {
        let text_field = Self {
            html: Self::base_html(),
            style: TextFieldStyle::FilledFullWidth,
        };
        text_field.ripple(true).class(TextFieldStyle::FilledFullWidth.class())
    }

    pub fn mdc_object(id: impl AsRef<str>) -> mdc::TextField {
        mdc::TextField::new(dom::get_exist_element_by_id::<Element>(id.as_ref()))
    }

    /// Returns the input's value.
    pub fn value(id: impl AsRef<str>) -> String {
        Self::mdc_object(id).value()
    }

    /// Sets the input's value.
    pub fn set_value(id: impl AsRef<str>, value: impl AsRef<str>) {
        Self::mdc_object(id).set_value(value.as_ref());
    }

    /// Returns whether or not the input is disabled.
    pub fn is_disabled(id: impl AsRef<str>) -> bool {
        Self::mdc_object(id).disabled()
    }

    /// Updates the input's disabled state.
    pub fn set_disabled(id: impl AsRef<str>, disabled: bool) {
        Self::mdc_object(id).set_disabled(disabled);
    }

    /// Focuses the `input` or `textarea` element.
    pub fn focus(id: impl AsRef<str>) {
        Self::mdc_object(id).focus();
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if self.style != TextFieldStyle::Outlined {
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
                self.root_tag_mut().children.insert(idx, html! {
                    <span class = "mdc-floating-label" id = label_id>{ label }</span>
                });
                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    input_tag.set_attr("aria-labelledby", label_id);
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
                    input_tag.set_attr("aria-labelledby", label_id);
                }
            },
            TextFieldStyle::FilledFullWidth => {
                if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
                    if let Html::VText(label) = label.into() {
                        input_tag.set_attr("placeholder", &label.text);
                        input_tag.set_attr("aria-label", label.text);
                    }
                }
            },
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

        if let Some(helper_line_div) = self
            .html_mut()
            .find_child_contains_class_mut("mdc-text-field-helper-line")
        {
            helper_line_div.children.insert(0, html! {
                <div class = "mdc-text-field-helper-text" id = helper_id aria-hidden = "true">{ helper_text }</div>
            });
        } else {
            self = self.into_widget_with_v_list();
            self.html_mut().add_child(html! {
                <div class = "mdc-text-field-helper-line">
                    <div class = "mdc-text-field-helper-text" id = helper_id aria-hidden = "true">{ helper_text }</div>
                </div>
            });
        }
        self
    }

    pub fn char_counter(mut self, max_length: usize) -> Self {
        let helper_string = format!("0 / {}", max_length);

        if let Some(input_tag) = self.root_tag_mut().find_child_tag_mut("input") {
            input_tag.set_attr("maxlength", format!("{}", max_length));
        }
        if let Some(helper_line_div) = self
            .html_mut()
            .find_child_contains_class_mut("mdc-text-field-helper-line")
        {
            helper_line_div.children.push(html! {
                <div class = "mdc-text-field-character-counter">{ helper_string }</div>
            });
        } else {
            self = self.into_widget_with_v_list();
            self.html_mut().add_child(html! {
                <div class="mdc-text-field-helper-line">
                    <div class = "mdc-text-field-character-counter">{ helper_string }</div>
                </div>
            });
        }
        self
    }

    pub fn root_id(&self) -> &str {
        self.root_tag()
            .attributes
            .get("id")
            .expect("The TextField widget must have ID")
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }

    pub fn on_input(mut self, callback: impl Into<Callback<InputData>>) -> Self {
        if let Some(input) = self.root_tag_mut().find_child_tag_recursively_mut("input") {
            input.add_listener(Rc::new(oninput::Wrapper::new(callback.into())));
        }
        self
    }
}

impl MdcWidget for TextField {
    const NAME: &'static str = stringify!(TextField);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl IntoWidgetWithVList for TextField {
    fn into_widget_with_v_list(mut self) -> Self {
        if !matches!(self.html, Html::VList(_)) {
            self.html = html! { <>{ self.html }</> }
        }
        self
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
    fn from(widget: TextField) -> Self {
        widget.html
    }
}
