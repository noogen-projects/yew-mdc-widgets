#![recursion_limit = "2048"]

use yew::{initialize, App, run_loop, utils, html, Component, ComponentLink, Html};
use yew_mdc_widgets::{Button, ButtonStyle, Checkbox, TextField, TextFieldStyle, DataTable, TableCell};

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
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#checkboxes").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Checkboxes" }</span>
                    </li>
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#text_fields").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Text fields" }</span>
                    </li>
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#data_tables").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Data tables" }</span>
                    </li>
                </ul>

                <h2 class = "demo-title mdc-typography--headline6"><a name = "buttons"></a>{ "Buttons" }</h2>
                <div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Text Button" }</h3>
                        <span class = "demo-item">
                            { Button::new("btn-text-default").label("Default") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-text-not-ripple").ripple(false).label("No ripple") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-text-disabled").disabled(true).label("Disabled") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-text-icon").icon("favorite").label("Icon") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-text-trailing-icon").label("Trailing icon").icon("favorite") }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Outlined Button" }</h3>
                        <span class = "demo-item">
                            { Button::new("btn-outlined-default").style(ButtonStyle::Outlined).label("Default") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-outlined-not-ripple").style(ButtonStyle::Outlined).ripple(false).label("No ripple") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-outlined-disabled").style(ButtonStyle::Outlined).disabled(true).label("Disabled") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-outlined-icon").style(ButtonStyle::Outlined).icon("favorite").label("Icon") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-outlined-trailing-icon").style(ButtonStyle::Outlined).label("Trailing icon").icon("favorite") }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Raised Button" }</h3>
                        <span class = "demo-item">
                            { Button::new("btn-raised-default").style(ButtonStyle::Raised).label("Default") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-raised-not-ripple").style(ButtonStyle::Raised).ripple(false).label("No ripple") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-raised-disabled").style(ButtonStyle::Raised).disabled(true).label("Disabled") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-raised-icon").style(ButtonStyle::Raised).icon("favorite").label("Icon") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-raised-trailing-icon").style(ButtonStyle::Raised).label("Trailing icon").icon("favorite") }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Unelevated Button" }</h3>
                        <span class = "demo-item">
                            { Button::new("btn-unelevated-default").style(ButtonStyle::Unelevated).label("Default") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-unelevated-not-ripple").style(ButtonStyle::Unelevated).ripple(false).label("No ripple") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-unelevated-disabled").style(ButtonStyle::Unelevated).disabled(true).label("Disabled") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-unelevated-icon").style(ButtonStyle::Unelevated).icon("favorite").label("Icon") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-unelevated-trailing-icon").style(ButtonStyle::Unelevated).label("Trailing icon").icon("favorite") }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Stylized Button" }</h3>
                        <span class = "demo-item">
                            { Button::new("btn-raised-rounded").style(ButtonStyle::Raised).class("rounded-button").label("Rounded") }
                        </span>
                        <span class = "demo-item">
                            { Button::new("btn-outlined-icon-rounded").style(ButtonStyle::Outlined).class("rounded-button").label("Rounded").icon("favorite") }
                        </span>
                    </div>
                </div>

                <h2 class = "demo-title mdc-typography--headline6"><a name = "checkboxes"></a>{ "Checkboxes" }</h2>
                <div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Unchecked" }</h3>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-unchecked") }
                        </span>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-unchecked-no-ripple").ripple(false) }
                        </span>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-unchecked-disabled").disabled(true) }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Indeterminate" }</h3>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-indeterminate").indeterminate(true) }
                        </span>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-indeterminate-no-ripple").indeterminate(true).ripple(false) }
                        </span>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-indeterminate-disabled").indeterminate(true).disabled(true) }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Checked" }</h3>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-checked").checked(true) }
                        </span>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-checked-no-ripple").checked(true).ripple(false) }
                        </span>
                        <span class = "demo-item">
                            { Checkbox::new("checkbox-checked-disabled").checked(true).disabled(true) }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Checkbox::new("checkbox-label").label("checkbox") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Checkbox::new("checkbox-label-no-ripple").ripple(false).checked(true).label("checkbox") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Checkbox::new("checkbox-label-disabled").disabled(true).label("checkbox") }
                            </div>
                        </span>
                    </div>
                </div>

                <h2 class = "demo-title mdc-typography--headline6"><a name = "text_fields"></a>{ "Text fields" }</h2>
                <div class = "widget-list">
                    <div>
                        { TextField::new("text-field-filled").style(TextFieldStyle::Filled).label("Filled text field").build() }
                    </div>
                </div>

                <h2 class = "demo-title mdc-typography--headline6"><a name = "data_tables"></a>{ "Data tables" }</h2>
                <div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Data Table Standard" }</h3>
                        <div>{
                            DataTable::new("data-table-standard")
                                .head(&[
                                    TableCell::Text("Dessert".into()),
                                    TableCell::Numeric("Carbs (g)".into()),
                                    TableCell::Numeric("Protein (g)".into()),
                                    TableCell::Text("Comments".into()),
                                ])
                                .row(&[
                                    TableCell::Text("Frozen yogurt".into()),
                                    TableCell::Numeric("24".into()),
                                    TableCell::Numeric("4.0".into()),
                                    TableCell::Text("Super tasty".into()),
                                ])
                                .row(&[
                                    TableCell::Text("Ice cream sandwich".into()),
                                    TableCell::Numeric("37".into()),
                                    TableCell::Numeric("4.33333333333".into()),
                                    TableCell::Text("I like ice cream more".into()),
                                ])
                                .row(&[
                                    TableCell::Text("Eclair".into()),
                                    TableCell::Numeric("24".into()),
                                    TableCell::Numeric("6.0".into()),
                                    TableCell::Text("New filing flavor".into()),
                                ])
                        }</div>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Data Table with Row Selection" }</h3>
                        <div>{
                            DataTable::new("data-table-selection")
                                .row_selection(true)
                                .head(&[
                                    TableCell::Text("Dessert".into()),
                                    TableCell::Numeric("Carbs (g)".into()),
                                    TableCell::Numeric("Protein (g)".into()),
                                    TableCell::Text("Comments".into()),
                                ])
                                .row(&[
                                    TableCell::Text("Frozen yogurt".into()),
                                    TableCell::Numeric("24".into()),
                                    TableCell::Numeric("4.0".into()),
                                    TableCell::Text("Super tasty".into()),
                                ])
                                .row(&[
                                    TableCell::Text("Ice cream sandwich".into()),
                                    TableCell::Numeric("37".into()),
                                    TableCell::Numeric("4.33333333333".into()),
                                    TableCell::Text("I like ice cream more".into()),
                                ])
                                .row(&[
                                    TableCell::Text("Eclair".into()),
                                    TableCell::Numeric("24".into()),
                                    TableCell::Numeric("6.0".into()),
                                    TableCell::Text("New filing flavor".into()),
                                ])
                        }</div>
                    </div>
                </div>
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
