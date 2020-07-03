use yew::{html, Html};
use crate::{Text, Widget};

pub struct TextField<'a> {
    id: Text<'a>,
    input_id: Text<'a>,
    label: Text<'a>,
}

impl<'a> TextField<'a> {
    pub fn new() -> Self {
        TextField {
            id: "".into(),
            input_id: "".into(),
            label: "".into(),
        }
    }

    pub fn id(mut self, id: impl Into<Text<'a>>) -> Self {
        self.id = id.into();
        self
    }

    pub fn input_id(mut self, input_id: impl Into<Text<'a>>) -> Self {
        self.input_id = input_id.into();
        self
    }

    pub fn label(mut self, label: impl Into<Text<'a>>) -> Self {
        self.label = label.into();
        self
    }
}

impl Widget for TextField<'_> {
    fn build(&self) -> Html {
        let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", self.id);

        html! {
            <div id = self.id class = "mdc-text-field mdc-text-field--outlined">
                <input id = self.input_id class = "mdc-text-field__input"/>
                <div class = "mdc-notched-outline">
                    <div class = "mdc-notched-outline__leading"></div>
                    <div class = "mdc-notched-outline__notch">
                        <label for = self.input_id class = "mdc-floating-label">{ &self.label }</label>
                    </div>
                    <div class = "mdc-notched-outline__trailing"></div>
                </div>
                <script>{ mdc_init }</script>
            </div>
        }
    }
}
