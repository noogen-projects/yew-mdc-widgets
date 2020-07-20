#![recursion_limit = "256"]

use yew::{html, Component, ComponentLink, Html};
use yew_mdc_widgets::{Button, ButtonStyle};

struct Root;

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
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
                <ul class = "catalog">
                    <li class = "catalog-item">
                        <a class = "catalog-link" href = "#buttons">{ "Buttons" }</a>
                    </li>
                    <h2><a name="buttons"></a>{ "Buttons" }</h2>
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
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Root>();
}
