use std::ops::DerefMut;

use yew::{html, Html};

use crate::Text;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextFieldStyle {
    Filled,
    Outlined,
    FullWidth,
}

impl TextFieldStyle {
    pub fn class(&self) -> &'static str {
        match self {
            TextFieldStyle::Filled => "mdc-text-field mdc-text-field--filled",
            TextFieldStyle::Outlined => "mdc-text-field mdc-text-field--outlined",
            TextFieldStyle::FullWidth => "mdc-text-field mdc-text-field--filled mdc-text-field--fullwidth",
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct AdditionTextFieldStyle {
    no_label: bool,
    disabled: bool,
    helper_text: bool,
    char_counter: bool,
}

impl AdditionTextFieldStyle {
    pub fn new(no_label: bool, disabled: bool, helper_text: bool, char_counter: bool) -> Self {
        Self {
            no_label,
            disabled,
            helper_text,
            char_counter,
        }
    }
}

pub struct TextField<'a> {
    id: Text<'a>,
    label: Text<'a>,
    style: TextFieldStyle,
    add_style: AdditionTextFieldStyle,
}

impl<'a> TextField<'a> {
    pub fn new(id: impl Into<Text<'a>>) -> Self {
        Self {
            id: id.into(),
            label: "".into(),
            style: TextFieldStyle::Filled,
            add_style: AdditionTextFieldStyle::default(),
        }
    }

    pub fn label(mut self, label: impl Into<Text<'a>>) -> Self {
        self.label = label.into();
        self
    }

    pub fn style(mut self, style: TextFieldStyle) -> Self {
        self.style = style;
        self
    }

    pub fn addition_style(mut self, add_style: AdditionTextFieldStyle) -> Self {
        self.add_style = add_style;
        self
    }

    fn filled(self) -> Html {
        let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", self.id);
        let label_id = format!("{}-label", self.id);

        html! {
            <label id = self.id class = self.style.class()>
                <span class = "mdc-text-field__ripple"></span>
                <input class = "mdc-text-field__input" type = "text" aria-labelledby = label_id />
                <span class = "mdc-floating-label" id = label_id>{ &self.label }</span>
                <span class = "mdc-line-ripple"></span>
                <script>{ mdc_init }</script>
            </label>
        }
    }

    fn outlined(self) -> Html {
        let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", self.id);
        let input_id = format!("{}-input", self.id);

        html! {
            <div id = self.id class = self.style.class()>
                <input id = input_id class = "mdc-text-field__input" type="text"/>
                <div class = "mdc-notched-outline">
                    <div class = "mdc-notched-outline__leading"></div>
                    <div class = "mdc-notched-outline__notch">
                        <label for = input_id class = "mdc-floating-label">{ &self.label }</label>
                    </div>
                    <div class = "mdc-notched-outline__trailing"></div>
                </div>
                <script>{ mdc_init }</script>
            </div>
        }
    }

    fn fullwidth(self) -> Html {
        let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", self.id);
        let input_id = format!("{}-input", self.id);

        html! {
            <div id = self.id class = self.style.class()>
                <div class = "mdc-text-field__ripple"></div>
                <input class = "mdc-text-field__input"
                       type = "text"
                       placeholder = &self.label
                       aria-label = &self.label/>
                <div class = "mdc-line-ripple"></div>
                <script>{ mdc_init }</script>
            </div>
        }
    }

    fn disabled(&self, mut html: Html) -> Html {
        if let Html::VTag(html) = &mut html {
            if let Some(class) = html.attributes.get_mut("class") {
                class.push_str("mdc-text-field--disabled");
            }
            for child in html.children.deref_mut() {
                if let Html::VTag(child) = child {
                    if child.tag() == "input" {
                        child.attributes. insert("disabled".to_string(), Default::default());
                    }
                }
            }
        }
        html
    }

    fn nolabel(&self, mut html: Html) -> Html {
        if let Html::VTag(html) = &mut html {
            if let Some(class) = html.attributes.get_mut("class") {
                class.push_str("mdc-text-field--no-label");
            }
            let maybe_label_idx = html
                .children
                .iter()
                .enumerate()
                .find_map(|(idx, child)| match child {
                    Html::VTag(child) if child.tag() == "label" => Some(idx),
                    _ => None,
                });
            if let Some(idx) = maybe_label_idx {
                html.children.remove(idx);
            }
        }
        html
    }

    fn helpertext(&self, mut html: Html, helper_text: &str) -> Html {
        if let Html::VTag(html) = &mut html {
            let html_id = html.attributes.get("id").cloned().expect("Html id expected");
            let input_id = html
                .children
                .iter()
                .find_map(|child| match child {
                    Html::VTag(child) if child.tag() == "input" =>
                        child.attributes.get("id").cloned(),
                    _ => None,
                })
                .expect("Input id expected");

            let helper_id = format!("{}-helper", html_id);
            for child in html.children.deref_mut() {
                if let Html::VTag(child) = child {
                    if child.tag() == "input" {
                        child.attributes. insert("aria-controls".to_string(), helper_id.clone());
                        child.attributes. insert("aria-describedby".to_string(), helper_id.clone());
                    }
                }
            }

            let helper_node = html!{
                <div class="mdc-text-field-helper-line">
                    <div id = helper_id for = input_id class = "mdc-text-field-helper-text" aria-hidden="true">{ &helper_text }</div>
                </div>
            };
            html.children.add_child(helper_node);
        }
        html
    }

    fn charcounter(&self, mut html: Html, max_length: i32) -> Html {
        if let Html::VTag(html) = &mut html {
            let mut is_helper_line_not_exists = true;
            for child in html.children.deref_mut() {
                if let Html::VTag(child) = child {
                    if child.tag() == "input" {
                        child.attributes.insert("maxlength".to_string(), format!{"{}", max_length});
                    }
                    if child.tag() == "div" {
                        if let Some(class) = child.attributes.get("class") {
                            if class == "mdc-text-field-helper-line" {
                                is_helper_line_not_exists = false;
                            }
                        }
                    }
                }
            }
            if is_helper_line_not_exists {
                let div = html! { <div class="mdc-text-field-helper-line"></div> };
                html.children.add_child(div);
            }
            let div = html! { <div class="mdc-text-field-character-counter">{ "0 / " } { max_length }</div> };
            html.children.add_child(div);
        }
        html
    }

    pub fn build(self) -> Html {
        match self.style {
            TextFieldStyle::Filled => self.filled(),
            TextFieldStyle::Outlined => self.outlined(),
            TextFieldStyle::FullWidth => self.fullwidth(),
        }
    }
}
