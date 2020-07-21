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

#[derive(Debug, Clone)]
pub struct Button<'a> {
    id: Text<'a>,
    text: Text<'a>,
    style: ButtonStyle,
    ripple: bool,
    disabled: bool,
    before_label: Vec<Html>,
    after_label: Vec<Html>,
    on_click: Callback<MouseEvent>,
}

impl<'a> Button<'a> {
    pub fn new(id: impl Into<Text<'a>>) -> Self {
        Self {
            id: id.into(),
            text: "Ok".into(),
            style: ButtonStyle::Text,
            ripple: true,
            disabled: false,
            before_label: vec![],
            after_label: vec![],
            on_click: Callback::default(),
        }
    }

    pub fn text(mut self, text: impl Into<Text<'a>>) -> Self {
        self.text = text.into();
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn ripple(mut self, ripple: bool) -> Self {
        self.ripple = ripple;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn add_before_label(mut self, item: impl Into<Html>) -> Self {
        self.before_label.push(item.into());
        self
    }

    pub fn add_after_label(mut self, item: impl Into<Html>) -> Self {
        self.after_label.push(item.into());
        self
    }

    pub fn icon<'b>(mut self, icon: impl Into<Text<'b>>) -> Self {
        self.add_before_label(html! {
            <i class = "material-icons mdc-button__icon" aria-hidden = "true">{ icon.into() }</i>
        })
    }

    pub fn trailing_icon<'b>(mut self, trailing_icon: impl Into<Text<'b>>) -> Self {
        self.add_after_label(html! {
            <i class = "material-icons mdc-button__icon" aria-hidden = "true">{ trailing_icon.into() }</i>
        })
    }

    pub fn on_click(mut self, callback: Callback<MouseEvent>) -> Self {
        self.on_click = callback;
        self
    }

    pub fn build(self) -> Html {
        let Self {
            id,
            text,
            style,
            ripple,
            disabled,
            before_label,
            after_label,
            on_click,
        } = self;

        let mut button = html! {
            <button id = id class = style.class() onclick = on_click>
                <span class = "mdc-button__label">{ text }</span>
            </button>
        };

        if let Html::VTag(button_tag) = &mut button {
            if !before_label.is_empty() {
                let mut children = before_label;
                children.extend(button_tag.children.drain(..));
                button_tag.children.children = children;
            }

            if !after_label.is_empty() {
                button_tag.children.extend(after_label);
            }

            if self.ripple {
                button_tag.children.insert(0, html! {
                    <div class = "mdc-button__ripple"></div>
                });
                button_tag.children.push(html! {
                    <script>{ format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id) }</script>
                });
            }

            if self.disabled {
                button_tag.attributes.insert("disabled".to_string(), "".to_string());
            }
        }
        button
    }
}

impl From<Button<'_>> for Html {
    fn from(button: Button<'_>) -> Self {
        button.build()
    }
}