use yew::{html, Callback, Html, MouseEvent};

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

pub fn button(id: impl AsRef<str>, text: impl AsRef<str>, style: ButtonStyle, onclick: Callback<MouseEvent>) -> Html {
    let id = id.as_ref();
    let text = text.as_ref();
    let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id);

    html! {
        <button id = id class = style.class() onclick = onclick>
            <span class = "mdc-button__ripple"></span>
            { text }
            <script>{ mdc_init }</script>
        </button>
    }
}
