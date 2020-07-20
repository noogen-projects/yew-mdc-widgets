use yew::{html, Callback, Html, MouseEvent};

use crate::Text;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonStyle {
    Text,
    Outlined,
    Raised,
    Unelevated,
}

impl ButtonStyle {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonStyle::Text => "mdc-button",
            ButtonStyle::Outlined => "mdc-button mdc-button--outlined",
            ButtonStyle::Raised => "mdc-button mdc-button--raised",
            ButtonStyle::Unelevated => "mdc-button mdc-button--unelevated",
        }
    }
}

pub struct Button<'a> {
    id: Text<'a>,
    text: Text<'a>,
    style: ButtonStyle,
    on_click: Callback<MouseEvent>,
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        Self {
            id: "".into(),
            text: "Ok".into(),
            style: ButtonStyle::Text,
            on_click: Callback::default(),
        }
    }

    pub fn id(mut self, id: impl Into<Text<'a>>) -> Self {
        self.id = id.into();
        self
    }

    pub fn text(mut self, text: impl Into<Text<'a>>) -> Self {
        self.text = text.into();
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        self.on_click = callback;
        self
    }

    pub fn build(&self) -> Html {
        let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", self.id);

        html! {
            <button id = self.id class = self.style.class() onclick = &self.on_click>
                <span class = "mdc-button__ripple"></span>
                { &self.text }
                <script>{ mdc_init }</script>
            </button>
        }
    }
}
