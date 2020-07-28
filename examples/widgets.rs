#![recursion_limit = "4096"]

use yew::{initialize, App, run_loop, utils, html, Component, ComponentLink, Html};
use yew_mdc_widgets::{Button, ButtonStyle, Checkbox, Radio, TextField, TextFieldStyle, DataTable, TableCell};

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
                    <li class = "mdc-list-item" onclick = self.link.callback(|_| utils::window().location().set_href("#radio_buttons").unwrap())>
                        <span class = "mdc-list-item__ripple"></span>
                        <span class = "mdc-list-item__text">{ "Radio buttons" }</span>
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

                <h2 class = "demo-title mdc-typography--headline6"><a name = "radio_buttons"></a>{ "Radio buttons" }</h2>
                <div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Default" }</h3>
                        <span class = "demo-item">
                            { Radio::new("radio-default-1").name_of_set("default-set") }
                        </span>
                        <span class = "demo-item">
                            { Radio::new("radio-default-2").name_of_set("default-set").checked(true) }
                        </span>
                        <span class = "demo-item">
                            { Radio::new("radio-no-ripple-1").name_of_set("no-ripple-set").ripple(false) }
                        </span>
                        <span class = "demo-item">
                            { Radio::new("radio-no-ripple-2").name_of_set("no-ripple-set").ripple(false).checked(true) }
                        </span>
                        <span class = "demo-item">
                            { Radio::new("radio-disabled-1").name_of_set("disabled-set").disabled(true) }
                        </span>
                        <span class = "demo-item">
                            { Radio::new("radio-disabled-2").name_of_set("disabled-set").disabled(true).checked(true) }
                        </span>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Radio::new("label-radio-1").name_of_set("label-set").label("Radio 1") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Radio::new("label-radio-2").name_of_set("label-set").checked(true).label("Radio 2") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Radio::new("radio-label-no-ripple-1").name_of_set("label-no-ripple-set").ripple(false).label("No ripple 1") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Radio::new("radio-label-no-ripple-2").name_of_set("label-no-ripple-set").ripple(false).checked(true).label("No ripple 2") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Radio::new("radio-label-disabled-1").name_of_set("dlabel-isabled-set").disabled(true).label("Disabled 1") }
                            </div>
                        </span>
                        <span class = "demo-item">
                            <div class = "mdc-form-field">
                                { Radio::new("radio-label-disabled-2").name_of_set("label-disabled-set").disabled(true).checked(true).label("Disabled 2") }
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
                                .head(vec![
                                    TableCell::text("Dessert"),
                                    TableCell::num("Carbs (g)"),
                                    TableCell::num("Protein (g)"),
                                    TableCell::text("Comments"),
                                ])
                                .row(vec![
                                    TableCell::text("Frozen yogurt"),
                                    TableCell::num("24"),
                                    TableCell::num("4.0"),
                                    TableCell::text("Super tasty"),
                                ])
                                .row(vec![
                                    TableCell::text("Ice cream sandwich"),
                                    TableCell::num("37"),
                                    TableCell::num("4.33333333333"),
                                    TableCell::text("I like ice cream more"),
                                ])
                                .row(vec![
                                    TableCell::text("Eclair"),
                                    TableCell::num("24"),
                                    TableCell::num("6.0"),
                                    TableCell::text("New filing flavor"),
                                ])
                        }</div>
                    </div>
                    <div>
                        <h3 class = "mdc-typography--subtitle1">{ "Data Table with Row Selection" }</h3>
                        <div>{
                            DataTable::new("data-table-selection")
                                .row_selection(true)
                                .head(vec![
                                    TableCell::text("Dessert"),
                                    TableCell::num("Carbs (g)"),
                                    TableCell::num("Protein (g)"),
                                    TableCell::text("Comments"),
                                ])
                                .row(vec![
                                    TableCell::text("Frozen yogurt"),
                                    TableCell::num("24"),
                                    TableCell::num("4.0"),
                                    TableCell::text("Super tasty"),
                                ])
                                .row(vec![
                                    TableCell::text("Ice cream sandwich"),
                                    TableCell::num("37"),
                                    TableCell::num("4.33333333333"),
                                    TableCell::text("I like ice cream more"),
                                ])
                                .row(vec![
                                    TableCell::text("Eclair"),
                                    TableCell::num("24"),
                                    TableCell::num("6.0"),
                                    TableCell::text("New filing flavor"),
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
