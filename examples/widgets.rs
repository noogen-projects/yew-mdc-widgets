#![recursion_limit = "1024"]

use yew::{initialize, App, run_loop, utils, html, Component, ComponentLink, Html};
use yew_mdc_widgets::{Button, ButtonStyle, TextField, TextFieldStyle, AdditionTextFieldStyle};

struct Root {
    link: ComponentLink<Self>,
}

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class = "demo-content">
                <ul class = "mdc-list">
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#buttons").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Buttons" }</span>
                    </li>
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#text_fields").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Text fields" }</span>
                    </li>

                    <h2 class = "demo-title mdc-typography--headline6"><a name = "buttons"></a>{ "Buttons" }</h2>
                    <div>
                        <div>
                            <h3 class = "mdc-typography--subtitle1">{ "Text Button" }</h3>
                            <span class = "demo-item">
                                { Button::new("btn-text-default").label("Default").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-text-not-ripple").ripple(false).label("No ripple").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-text-disabled").disabled(true).label("Disabled").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-text-icon").icon("favorite").label("Icon").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-text-trailing-icon").trailing_icon("favorite").label("Trailing icon").build() }
                            </span>
                        </div>
                        <div>
                            <h3 class = "mdc-typography--subtitle1">{ "Outlined Button" }</h3>
                            <span class = "demo-item">
                                { Button::new("btn-outlined-default").style(ButtonStyle::Outlined).label("Default").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-outlined-not-ripple").style(ButtonStyle::Outlined).ripple(false).label("No ripple").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-outlined-disabled").style(ButtonStyle::Outlined).disabled(true).label("Disabled").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-outlined-icon").style(ButtonStyle::Outlined).icon("favorite").label("Icon").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-outlined-trailing-icon").style(ButtonStyle::Outlined).trailing_icon("favorite").label("Trailing icon").build() }
                            </span>
                        </div>
                        <div>
                            <h3 class = "mdc-typography--subtitle1">{ "Raised Button" }</h3>
                            <span class = "demo-item">
                                { Button::new("btn-raised-default").style(ButtonStyle::Raised).label("Default").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-raised-not-ripple").style(ButtonStyle::Raised).ripple(false).label("No ripple").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-raised-disabled").style(ButtonStyle::Raised).disabled(true).label("Disabled").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-raised-icon").style(ButtonStyle::Raised).icon("favorite").label("Icon").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-raised-trailing-icon").style(ButtonStyle::Raised).trailing_icon("favorite").label("Trailing icon").build() }
                            </span>
                        </div>
                        <div>
                            <h3 class = "mdc-typography--subtitle1">{ "Unelevated Button" }</h3>
                            <span class = "demo-item">
                                { Button::new("btn-unelevated-default").style(ButtonStyle::Unelevated).label("Default").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-unelevated-not-ripple").style(ButtonStyle::Unelevated).ripple(false).label("No ripple").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-unelevated-disabled").style(ButtonStyle::Unelevated).disabled(true).label("Disabled").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-unelevated-icon").style(ButtonStyle::Unelevated).icon("favorite").label("Icon").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-unelevated-trailing-icon").style(ButtonStyle::Unelevated).trailing_icon("favorite").label("Trailing icon").build() }
                            </span>
                        </div>
                        <div>
                            <h3 class = "mdc-typography--subtitle1">{ "Stylized Button" }</h3>
                            <span class = "demo-item">
                                { Button::new("btn-raised-rounded").style(ButtonStyle::Raised).classes("rounded-button").label("Default").build() }
                            </span>
                            <span class = "demo-item">
                                { Button::new("btn-outlined-icon-rounded").style(ButtonStyle::Outlined).classes("rounded-button").icon("favorite").label("Icon").build() }
                            </span>
                        </div>
                    </div>

                    <h2 class = "demo-title mdc-typography--headline6"><a name = "text_fields"></a>{ "Text fields" }</h2>
                    <div class = "widget-list">
                        <div>
                            { TextField::new("text-field-filled").style(TextFieldStyle::Filled).label("Filled text field").build() }
                        </div>
                    </div>
                </ul>
            </div>
        }
    }
}

fn main() {
    initialize();
    let root = utils::document()
        .query_selector("#root")
        .expect("Can't get root node for rendering")
        .expect("Can't unwrap root node");
    App::<Root>::new().mount_with_props(root, ());
    run_loop();
}
