#![recursion_limit = "512"]

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
            <div>
                <ul class = "mdc-list">
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#buttons").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Buttons" }</span>
                    </li>
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#text_fields").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Text fields" }</span>
                    </li>

                    <h2 class="mdc-typography--headline2"><a name="buttons"></a>{ "Buttons" }</h2>
                    <div class = "widget-list">
                        <div>
                            { Button::new().style(ButtonStyle::Text).text("Text button").build() }
                        </div>
                        <div>
                            { Button::new().style(ButtonStyle::Outlined).text("Outlined button").build() }
                        </div>
                        <div>
                            { Button::new().style(ButtonStyle::Raised).text("Raised button").build() }
                        </div>
                        <div>
                            { Button::new().style(ButtonStyle::Unelevated).text("Unelevated button").build() }
                        </div>
                    </div>
                    <h2 class="mdc-typography--headline2"><a name="text_fields"></a>{ "Text fields" }</h2>
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
