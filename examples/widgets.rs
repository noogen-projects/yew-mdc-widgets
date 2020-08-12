#![recursion_limit = "4096"]

use yew::{initialize, App, run_loop, utils, html, Component, ComponentLink, Html};
use yew_mdc_widgets::{
    Button, ButtonStyle, Checkbox, Radio, TextField, DataTable, TableCell, List, ListItem, Menu,
};

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
                <h1 class = "demo-title mdc-typography--headline5">{ "Material design components" }</h1>
                {
                    List::new("contents").items(vec![
                        ListItem::default()
                            .text("Buttons")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#buttons").unwrap())),
                        ListItem::default()
                            .text("Checkboxes")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#checkboxes").unwrap())),
                        ListItem::default()
                            .text("Radio buttons")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#radio_buttons").unwrap())),
                        ListItem::default()
                            .text("Text fields")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#text_fields").unwrap())),
                        ListItem::default()
                            .text("Lists")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#lists").unwrap())),
                        ListItem::default()
                            .text("Menu")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#menu").unwrap())),
                        ListItem::default()
                            .text("Data tables")
                            .on_click(self.link.callback(|_| utils::window().location().set_href("#data_tables").unwrap())),
                    ])
                }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "buttons"></a>{ "Buttons" }</h2>
                { self.view_buttons() }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "checkboxes"></a>{ "Checkboxes" }</h2>
                { self.view_checkboxes() }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "radio_buttons"></a>{ "Radio buttons" }</h2>
                { self.view_radio_buttons() }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "text_fields"></a>{ "Text fields" }</h2>
                { self.view_text_fields() }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "lists"></a>{ "Lists" }</h2>
                { self.view_lists() }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "menu"></a>{ "Menu" }</h2>
                { self.view_menu() }

                <h2 class = "demo-title mdc-typography--headline6"><a name = "data_tables"></a>{ "Data tables" }</h2>
                { self.view_data_tables() }
            </div>
        }
    }
}

impl Root {
    fn view_buttons(&self) -> Html {
        html! {
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
        }
    }

    fn view_checkboxes(&self) -> Html {
        html! {
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
        }
    }

    fn view_radio_buttons(&self) -> Html {
        html! {
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
        }
    }

    fn view_text_fields(&self) -> Html {
        html! {
            <div>
                <div>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled") }
                    </span>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled-labeled").label("Filled text field") }
                    </span>
                </div>
                <div>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined-labeled").label("Outlined text field") }
                    </span>
                </div>
                <div>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth("text-field-fullwidth").label("Fullwidth text field") }
                    </span>
                </div>
            </div>
        }
    }

    fn view_lists(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Single-Line" }</h3>
                    <span class = "demo-item demo-list">
                        {
                            List::new("single-line-list").items(vec![
                                ListItem::default().text("List Item"),
                                ListItem::default().text("List Item"),
                                ListItem::default().text("List Item"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("single-line-list-icon").items(vec![
                                ListItem::default().icon("wifi").text("Leading Icon"),
                                ListItem::default().icon("bluetooth").text("Leading Icon"),
                                ListItem::default().icon("data_usage").text("Leading Icon"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("single-line-list-trailing-icon").items(vec![
                                ListItem::default().text("Trailing Icon").icon("info"),
                                ListItem::default().text("Trailing Icon").icon("info"),
                                ListItem::default().text("Trailing Icon").icon("info"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("single-line-list-checkbox").items(vec![
                                ListItem::default()
                                    .tile(Checkbox::new("single-line-list-checkbox-item-1-checkbox").markup_only())
                                    .label("Checkbox Item"),
                                ListItem::default()
                                    .tile(Checkbox::new("single-line-list-checkbox-item-2-checkbox").markup_only())
                                    .label("Checkbox Item"),
                                ListItem::default()
                                    .tile(Checkbox::new("single-line-list-checkbox-item-3-checkbox").markup_only())
                                    .label("Checkbox Item"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("single-line-list-radio").items(vec![
                                ListItem::default()
                                    .label("Radio Item")
                                    .tile(Radio::new("single-line-list-radio-item-1-radio").name_of_set("single-line-list-radio").markup_only()),
                                ListItem::default()
                                    .label("Radio Item")
                                    .tile(Radio::new("single-line-list-radio-item-2-radio").name_of_set("single-line-list-radio").markup_only()),
                                ListItem::default()
                                    .label("Radio Item")
                                    .tile(Radio::new("single-line-list-radio-item-3-radio").name_of_set("single-line-list-radio").markup_only()),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("single-line-no-ripple-list").items(vec![
                                ListItem::default().ripple(false).text("No ripple"),
                                ListItem::default().ripple(false).text("No ripple"),
                                ListItem::default().ripple(false).text("No ripple"),
                            ])
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Two-Line" }</h3>
                    <span class = "demo-item demo-list">
                        {
                            List::new("two-line-list").two_line().items(vec![
                                ListItem::default().text("List Item").text("Secondary text"),
                                ListItem::default().text("List Item").text("Secondary text"),
                                ListItem::default().text("List Item").text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("two-line-list-icon").two_line().items(vec![
                                ListItem::default().icon("wifi").text("Leading Icon").text("Secondary text"),
                                ListItem::default().icon("bluetooth").text("Leading Icon").text("Secondary text"),
                                ListItem::default().icon("data_usage").text("Leading Icon").text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("two-line-list-trailing-icon").two_line().items(vec![
                                ListItem::default().text("Trailing Icon").text("Secondary text").icon("info"),
                                ListItem::default().text("Trailing Icon").text("Secondary text").icon("info"),
                                ListItem::default().text("Trailing Icon").text("Secondary text").icon("info"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("two-line-list-checkbox").two_line().items(vec![
                                ListItem::default()
                                    .tile(Checkbox::new("two-line-list-checkbox-item-1-checkbox").markup_only())
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                                ListItem::default()
                                    .tile(Checkbox::new("two-line-list-checkbox-item-2-checkbox").markup_only())
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                                ListItem::default()
                                    .tile(Checkbox::new("two-line-list-checkbox-item-3-checkbox").markup_only())
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("two-line-list-radio").two_line().items(vec![
                                ListItem::default()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::new("two-line-list-radio-item-1-radio").name_of_set("two-line-list-radio").markup_only()),
                                ListItem::default()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::new("two-line-list-radio-item-2-radio").name_of_set("two-line-list-radio").markup_only()),
                                ListItem::default()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::new("two-line-list-radio-item-3-radio").name_of_set("two-line-list-radio").markup_only()),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::new("two-line-no-ripple-list").two_line().items(vec![
                                ListItem::default().ripple(false).text("No ripple").text("Secondary text"),
                                ListItem::default().ripple(false).text("No ripple").text("Secondary text"),
                                ListItem::default().ripple(false).text("No ripple").text("Secondary text"),
                            ])
                        }
                    </span>
                </div>
                <h3 class = "mdc-typography--subtitle1">{ "List Groups" }</h3>
                <div class = "demo-item mdc-list-group demo-group-list demo-panel">
                    <h3 class = "mdc-list-group__subheader">{ "List 1" }</h3>
                    {
                        List::new("list-group-1").items(vec![
                            ListItem::default().text("List Item"),
                            ListItem::default().text("List Item"),
                        ])
                    }
                    <h3 class = "mdc-list-group__subheader">{ "List 2" }</h3>
                    {
                        List::new("list-group-2").items(vec![
                            ListItem::default().text("List Item"),
                            ListItem::default().text("List Item"),
                        ])
                    }
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Two-Line with Leading and Trailing Icon and Divider" }</h3>
                    <span class = "demo-item bordered-list demo-panel">
                        {
                            List::new("complex-list")
                                .two_line()
                                .avatar()
                                .item(ListItem::default().icon("folder").text("Dog Photos").text("9 Jan 2018").icon("info"))
                                .divider_inset_leading()
                                .item(ListItem::default().icon("folder").text("Cat Photos").text("22 Dec 2017").icon("info"))
                                .divider()
                                .items(vec![
                                    ListItem::default().icon("folder").text("Potatoes").text("30 Noc 2017").icon("info"),
                                    ListItem::default().icon("folder").text("Carrots").text("17 Oct 2017").icon("info"),
                                ])
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "List with activated item" }</h3>
                    <span class = "demo-item bordered-list demo-panel">
                        {
                            List::new("activated-list").single_selection().items(vec![
                                ListItem::default().icon("inbox").text("Inbox"),
                                ListItem::default().icon("star").text("Star").selected(true),
                                ListItem::default().icon("send").text("Send"),
                                ListItem::default().icon("drafts").text("Drafts"),
                            ])
                        }
                    </span>
                </div>
            </div>
        }
    }

    fn view_menu(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Simple" }</h3>
                    <span class = "demo-item demo-list">
                        <div id = "demo-menu" class = Menu::ANCHOR_CLASS>
                            {
                                Button::new("simple-menu-button")
                                    .label("Open Menu")
                                    .on_click(self.link.callback(|_| Menu::open_existing("simple-menu")))
                            }
                            {
                                Menu::new("simple-menu").items(vec![
                                    ListItem::default().text("Menu Item"),
                                    ListItem::default().text("Menu Item"),
                                    ListItem::default().text("Menu Item"),
                                ])
                            }
                        </div>
                    </span>
                </div>
            </div>
        }
    }

    fn view_data_tables(&self) -> Html {
        html! {
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
