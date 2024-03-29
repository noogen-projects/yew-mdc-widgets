use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use yew::html::{onclick, oninput};
use yew::virtual_dom::{AttrValue, VTag};
use yew::{classes, html, Callback, Html, InputEvent, MouseEvent, ToHtml};

use crate::floating_label::FloatingLabel;
use crate::notched_outline::NotchedOutline;
use crate::utils::{IntoWidgetWithVList, ManageChildren, VTagExt};
use crate::{line_ripple, MdcObject, MdcWidget, AUTO_INIT_ATTR};

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
}

impl TextFieldStyle {
    pub fn class(&self) -> &'static str {
        match self {
            TextFieldStyle::Filled => "mdc-text-field--filled",
            TextFieldStyle::Outlined => "mdc-text-field--outlined",
        }
    }

    pub fn classes() -> [&'static str; 2] {
        [TextFieldStyle::Filled.class(), TextFieldStyle::Outlined.class()]
    }
}

#[derive(Debug, Clone)]
pub struct TextField {
    html: Html,
    style: TextFieldStyle,
}

impl TextField {
    pub const CLASS: &'static str = "mdc-text-field";
    pub const INPUT_CLASS: &'static str = "mdc-text-field__input";
    pub const RIPPLE_CLASS: &'static str = "mdc-text-field__ripple";
    pub const DISABLED_CLASS: &'static str = "mdc-text-field--disabled";
    pub const HELPER_LINE_CLASS: &'static str = "mdc-text-field-helper-line";
    pub const CHARACTER_COUNTER_CLASS: &'static str = "mdc-text-field-character-counter";
    pub const WITH_LEADING_ICON_CLASS: &'static str = "mdc-text-field--with-leading-icon";
    pub const WITH_TRAILING_ICON_CLASS: &'static str = "mdc-text-field--with-trailing-icon";
    pub const ICON_CLASS: &'static str = "mdc-text-field__icon";
    pub const LEADING_ICON_CLASS: &'static str = "mdc-text-field__icon--leading";
    pub const TRAILING_ICON_CLASS: &'static str = "mdc-text-field__icon--trailing";
    pub const NO_LABEL_CLASS: &'static str = "mdc-text-field--no-label";
    pub const WITH_LABEL_FLOATING_CLASS: &'static str = "mdc-text-field--label-floating";

    fn simple() -> Html {
        let mut html = html! {
            <label class = { Self::CLASS }>
                <input class = { Self::INPUT_CLASS } type = "text"/>
            </label>
        };
        html.root_tag_mut().unwrap(/* root tag already exists */).set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        html
    }

    pub fn filled() -> Self {
        let text_field = Self {
            html: Self::simple(),
            style: TextFieldStyle::Filled,
        };
        text_field.ripple(true).class(TextFieldStyle::Filled.class())
    }

    pub fn outlined() -> Self {
        let mut text_field = Self {
            html: Self::simple(),
            style: TextFieldStyle::Outlined,
        };
        text_field.root_tag_mut().add_child(NotchedOutline::new().into());
        text_field.class(TextFieldStyle::Outlined.class())
    }

    /// Returns the input's value.
    pub fn get_value(id: impl AsRef<str>) -> String {
        Self::get_mdc_object(id).value()
    }

    /// Sets the input's value.
    pub fn set_value(id: impl AsRef<str>, value: impl AsRef<str>) {
        Self::get_mdc_object(id).set_value(value.as_ref());
    }

    /// Returns whether or not the input is disabled.
    pub fn is_disabled(id: impl AsRef<str>) -> bool {
        Self::get_mdc_object(id).disabled()
    }

    /// Updates the input's disabled state.
    pub fn set_disabled(id: impl AsRef<str>, disabled: bool) {
        Self::get_mdc_object(id).set_disabled(disabled);
    }

    /// Focuses the `input` or `textarea` element.
    pub fn focus(id: impl AsRef<str>) {
        Self::get_mdc_object(id).focus();
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        if self.style != TextFieldStyle::Outlined {
            if let Some(list) = self
                .root_tag_mut()
                .children_mut()
                .map(|children| children.to_vlist_mut())
            {
                if enabled {
                    if !list.is_some_child_contains_class(Self::RIPPLE_CLASS) {
                        list.insert(0, html! {
                            <span class = { Self::RIPPLE_CLASS }></span>
                        });
                    }
                    if !list.is_some_child_contains_class(line_ripple::mdc::CLASS) {
                        list.push(html! {
                            <span class = { line_ripple::mdc::CLASS }></span>
                        });
                    }
                } else {
                    if let Some(idx) = list.find_child_contains_class_idx(Self::RIPPLE_CLASS) {
                        let _ = list.remove(idx);
                    }
                    if let Some(idx) = list.find_child_contains_class_idx(line_ripple::mdc::CLASS) {
                        let _ = list.remove(idx);
                    }
                }
            }
        }
        self
    }

    pub fn label(self, label: impl Into<Html>) -> Self {
        self.floating_label(FloatingLabel::new(label))
    }

    pub fn floating_label(mut self, label: FloatingLabel) -> Self {
        let label_id = label.get_id().unwrap_or_else(|| format!("{}-label", self.root_id()));

        match self.style {
            TextFieldStyle::Filled => {
                let idx = self
                    .root_tag()
                    .find_child_tag_idx("input")
                    .map(|idx| idx + 1)
                    .unwrap_or(0);
                self.root_tag_mut().insert_child(idx, label);
                if let Some(input_tag) = self.input_tag_mut() {
                    input_tag.set_attr("aria-labelledby", label_id);
                }
            },
            TextFieldStyle::Outlined => {
                if let Some(tag) = self.root_tag_mut().find_child_contains_class_mut(NotchedOutline::CLASS) {
                    if let Some(notch) = tag.find_child_contains_class_mut(NotchedOutline::NOTCH_CLASS) {
                        notch.add_child(label.into());
                    }
                }

                if let Some(input_tag) = self.input_tag_mut() {
                    input_tag.set_attr("aria-labelledby", label_id);
                }
            },
        }
        self
    }

    pub fn value(mut self, value: impl Into<AttrValue>) -> Self {
        self.add_class_if_needed(Self::WITH_LABEL_FLOATING_CLASS);
        if let Some(label) = self.find_child_contains_class_recursively_mut(FloatingLabel::CLASS) {
            label.add_class_if_needed(FloatingLabel::FLOAT_ABOVE_CLASS);
        }
        if let Some(notched) = self.find_child_contains_class_recursively_mut(NotchedOutline::CLASS) {
            notched.add_class_if_needed(NotchedOutline::NOTCHED_CLASS);
        }
        if let Some(input) = self.input_tag_mut() {
            input.set_value(Some(value.into()));
        }
        self
    }

    pub fn leading_tile(mut self, tile: impl Into<Html>) -> Self {
        self.add_class(Self::WITH_LEADING_ICON_CLASS);
        let root = self.root_tag_mut();
        let index = root.find_child_tag_idx("input").unwrap_or_default();
        root.insert_child(index, tile);
        self
    }

    pub fn trailing_tile(mut self, tile: impl Into<Html>) -> Self {
        self.add_class(Self::WITH_TRAILING_ICON_CLASS);
        let root = self.root_tag_mut();
        let index = root
            .find_child_tag_idx("input")
            .map(|index| index + 1)
            .unwrap_or_default();
        root.insert_child(index, tile);
        self
    }

    pub fn leading_icon(self, name: impl Into<String>) -> Self {
        self.leading_tile(
            html! { <i class = { classes!("material-icons", Self::ICON_CLASS, Self::LEADING_ICON_CLASS) }>{ name.into() }</i> },
        )
    }

    pub fn trailing_icon(self, name: impl Into<String>) -> Self {
        self.trailing_tile(
            html! { <i class = { classes!("material-icons", Self::ICON_CLASS, Self::TRAILING_ICON_CLASS) }>{ name.into() }</i> },
        )
    }

    pub fn disabled(mut self) -> Self {
        self.add_class(Self::DISABLED_CLASS);
        if let Some(input_tag) = self.input_tag_mut() {
            input_tag.set_attr("disabled", "");
        }
        self
    }

    pub fn helper_text(mut self, mut helper_text: HelperText) -> Self {
        let id = self.root_id();
        let helper_id = match helper_text.root_tag().attr("id") {
            Some(id) => id.to_string(),
            None => {
                let helper_id = format!("{}-helper", id);
                helper_text = helper_text.id(&helper_id);
                helper_id
            },
        };

        if let Some(input_tag) = self.input_tag_mut() {
            input_tag.set_attr("aria-controls", helper_id.clone());
            input_tag.set_attr("aria-describedby", helper_id);
        }

        if let Some(helper_line) = self.html_mut().find_child_contains_class_mut(Self::HELPER_LINE_CLASS) {
            helper_line.insert_child(0, helper_text);
        } else {
            self = self.into_widget_with_v_list();
            self.html_mut().add_child(html! {
                <div class = { Self::HELPER_LINE_CLASS }>
                    { helper_text }
                </div>
            });
        }
        self
    }

    pub fn char_counter(mut self, max_length: usize) -> Self {
        let helper_string = format!("0 / {}", max_length);

        if let Some(input_tag) = self.input_tag_mut() {
            input_tag.set_attr("maxlength", format!("{}", max_length));
        }
        if let Some(helper_line) = self.html_mut().find_child_contains_class_mut(Self::HELPER_LINE_CLASS) {
            helper_line.add_child(html! {
                <div class = { Self::CHARACTER_COUNTER_CLASS }>{ helper_string }</div>
            });
        } else {
            self = self.into_widget_with_v_list();
            self.html_mut().add_child(html! {
                <div class = { Self::HELPER_LINE_CLASS }>
                    <div class = { Self::CHARACTER_COUNTER_CLASS }>{ helper_string }</div>
                </div>
            });
        }
        self
    }

    pub fn input_tag_mut(&mut self) -> Option<&mut VTag> {
        self.root_tag_mut().find_child_tag_mut("input")
    }

    pub fn root_id(&self) -> AttrValue {
        self.root_tag().attr("id").expect("The TextField widget must have ID")
    }

    pub fn on_click(self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        self.listener(Rc::new(onclick::Wrapper::new(callback.into())))
    }

    pub fn on_input(mut self, callback: impl Into<Callback<InputEvent>>) -> Self {
        if let Some(input) = self.input_tag_mut() {
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

impl MdcObject for TextField {
    const MDC_TYPE_NAME: &'static str = mdc::TYPE_NAME;
    type MdcType = mdc::TextField;
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

impl ToHtml for TextField {
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    fn into_html(self) -> Html {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct HelperText {
    html: Html,
}

impl HelperText {
    pub const CLASS: &'static str = "mdc-text-field-helper-text";
    pub const PERSISTENT_CLASS: &'static str = "mdc-text-field-helper-text--persistent";
    pub const VALIDATION_MSG_CLASS: &'static str = "mdc-text-field-helper-text--validation-msg";

    pub fn new(text: impl Into<Html>) -> Self {
        Self {
            html: html! {
                <div class = { Self::CLASS }>{ text.into() }</div>
            },
        }
    }

    pub fn persistent(mut self) -> Self {
        self.root_tag_mut().add_class(Self::PERSISTENT_CLASS);
        self
    }

    pub fn validation_msg(mut self) -> Self {
        self.root_tag_mut().add_class(Self::VALIDATION_MSG_CLASS);
        self
    }
}

impl MdcWidget for HelperText {
    const NAME: &'static str = stringify!(HelperText);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for HelperText {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for HelperText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<HelperText> for Html {
    fn from(widget: HelperText) -> Self {
        widget.html
    }
}

impl ToHtml for HelperText {
    fn to_html(&self) -> Html {
        self.clone().into()
    }

    fn into_html(self) -> Html {
        self.into()
    }
}
