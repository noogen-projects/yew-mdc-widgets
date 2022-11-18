use yew::{html, Html};
use yew_mdc_widgets::{Button, ButtonStyle, MdcWidget};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Text Button" }</h3>
                <span class = "demo-item">
                    { Button::new().label("Default") }
                </span>
                <span class = "demo-item">
                    { Button::simple().label("No ripple") }
                </span>
                <span class = "demo-item">
                    { Button::new().disabled().label("Disabled") }
                </span>
                <span class = "demo-item">
                    { Button::new().icon("favorite").label("Icon") }
                </span>
                <span class = "demo-item">
                    { Button::new().label("Trailing icon").icon("favorite") }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Outlined Button" }</h3>
                <span class = "demo-item">
                    { Button::outlined().label("Default") }
                </span>
                <span class = "demo-item">
                    { Button::simple().style(ButtonStyle::Outlined).label("No ripple") }
                </span>
                <span class = "demo-item">
                    { Button::outlined().disabled().label("Disabled") }
                </span>
                <span class = "demo-item">
                    { Button::outlined().icon("favorite").label("Icon") }
                </span>
                <span class = "demo-item">
                    { Button::outlined().label("Trailing icon").icon("favorite") }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Raised Button" }</h3>
                <span class = "demo-item">
                    { Button::raised().label("Default") }
                </span>
                <span class = "demo-item">
                    { Button::simple().style(ButtonStyle::Raised).label("No ripple") }
                </span>
                <span class = "demo-item">
                    { Button::raised().disabled().label("Disabled") }
                </span>
                <span class = "demo-item">
                    { Button::raised().icon("favorite").label("Icon") }
                </span>
                <span class = "demo-item">
                    { Button::raised().label("Trailing icon").icon("favorite") }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Unelevated Button" }</h3>
                <span class = "demo-item">
                    { Button::unelevated().label("Default") }
                </span>
                <span class = "demo-item">
                    { Button::simple().style(ButtonStyle::Unelevated).label("No ripple") }
                </span>
                <span class = "demo-item">
                    { Button::unelevated().disabled().label("Disabled") }
                </span>
                <span class = "demo-item">
                    { Button::unelevated().icon("favorite").label("Icon") }
                </span>
                <span class = "demo-item">
                    { Button::unelevated().label("Trailing icon").icon("favorite") }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Stylized Button" }</h3>
                <span class = "demo-item">
                    { Button::raised().class("rounded-button").label("Rounded") }
                </span>
                <span class = "demo-item">
                    { Button::outlined().class("rounded-button").label("Rounded").icon("favorite") }
                </span>
            </div>
        </div>
    }
}
