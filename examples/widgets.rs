#![recursion_limit = "4096"]

use std::iter::FromIterator;

use yew::{html, initialize, run_loop, utils, App, Component, ComponentLink, Html};
use yew_mdc_widgets::{
    auto_init, Button, ButtonStyle, Card, CardContent, Checkbox, DataTable, Drawer, IconButton, List, ListItem,
    MdcWidget, Menu, Radio, TableCell, TextField, TopAppBar,
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
        let contents = vec![
            ListItem::link("#buttons").text("Buttons").attr("tabindex", "0"),
            ListItem::link("#icon_buttons").text("Icon buttons"),
            ListItem::link("#checkboxes").text("Checkboxes"),
            ListItem::link("#radio_buttons").text("Radio buttons"),
            ListItem::link("#text_fields").text("Text fields"),
            ListItem::link("#lists").text("Lists"),
            ListItem::link("#menu").text("Menu"),
            ListItem::link("#data_tables").text("Data tables"),
            ListItem::link("#cards").text("Cards"),
        ];

        let drawer = Drawer::new("app-drawer")
            .title(html! { <h3 tabindex = 0>{ "Widgets" }</h3> })
            .modal()
            .content(List::nav("main-menu").items(contents.clone()).markup_only());
        let attaching_drawer = Drawer::get_attaching_script("app-drawer");

        let top_app_bar = TopAppBar::new("top-app-bar")
            .title("Yew MDC Widgets")
            .navigation_item(IconButton::new().icon("menu"))
            .enable_shadow_when_scroll_window()
            .add_navigation_event("drawer.open = !drawer.open;");

        html! {
            <>
                { drawer }
                <div class="mdc-drawer-scrim"></div>

                <div class = vec!["app-content", Drawer::APP_CONTENT_CLASS]>
                    { top_app_bar }
                    <script>{ format!(r"
                        const drawer = {};
                        const listEl = document.querySelector('.mdc-drawer .mdc-list');                    
                        listEl.addEventListener('click', (event) => {{
                            drawer.open = false;
                        }});
                    ", attaching_drawer) }</script>

                    <div class = "mdc-top-app-bar--fixed-adjust">
                        <div class = "demo-content">
                            <h1 class = "demo-title mdc-typography--headline5">{ "Material design components" }</h1>
                            { List::nav("contents").items(contents) }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "buttons"></a>{ "Buttons" }</h2>
                            { self.view_buttons() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "icon_buttons"></a>{ "Icon buttons" }</h2>
                            { self.view_icon_buttons() }

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

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "cards"></a>{ "Cards" }</h2>
                            { self.view_cards() }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        auto_init();
    }
}

impl Root {
    fn view_buttons(&self) -> Html {
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
                        { Button::new().disabled(true).label("Disabled") }
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
                        { Button::new().style(ButtonStyle::Outlined).label("Default") }
                    </span>
                    <span class = "demo-item">
                        { Button::simple().style(ButtonStyle::Outlined).label("No ripple") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Outlined).disabled(true).label("Disabled") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Outlined).icon("favorite").label("Icon") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Outlined).label("Trailing icon").icon("favorite") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Raised Button" }</h3>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Raised).label("Default") }
                    </span>
                    <span class = "demo-item">
                        { Button::simple().style(ButtonStyle::Raised).label("No ripple") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Raised).disabled(true).label("Disabled") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Raised).icon("favorite").label("Icon") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Raised).label("Trailing icon").icon("favorite") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Unelevated Button" }</h3>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Unelevated).label("Default") }
                    </span>
                    <span class = "demo-item">
                        { Button::simple().style(ButtonStyle::Unelevated).label("No ripple") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Unelevated).disabled(true).label("Disabled") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Unelevated).icon("favorite").label("Icon") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Unelevated).label("Trailing icon").icon("favorite") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Stylized Button" }</h3>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Raised).class("rounded-button").label("Rounded") }
                    </span>
                    <span class = "demo-item">
                        { Button::new().style(ButtonStyle::Outlined).class("rounded-button").label("Rounded").icon("favorite") }
                    </span>
                </div>
            </div>
        }
    }

    fn view_icon_buttons(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Icon Button" }</h3>
                    <span class = "demo-item">
                        { IconButton::new().icon("wifi") }
                    </span>
                    <span class = "demo-item">
                        { IconButton::new().ripple(false).icon("wifi") }
                    </span>
                    <span class = "demo-item">
                        { IconButton::new().disabled(true).icon("wifi") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Icon Toggle Button" }</h3>
                    <span class = "demo-item">
                        { IconButton::new().toggle("favorite", "favorite_border") }
                    </span>
                    <span class = "demo-item">
                        { IconButton::new().toggle("favorite", "favorite_border").on() }
                    </span>
                    <span class = "demo-item">
                        { IconButton::new().toggle("favorite", "favorite_border").disabled(true) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Svg Icon Toggle Button" }</h3>
                    <span class = "demo-item">
                        {
                            IconButton::new()
                                .toggle_on(html! {
                                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = "24" height = "24">
                                        <path d = "M12.97 2.59a1.5 1.5 0 00-1.94 0l-7.5 6.363A1.5 1.5 0 003 10.097V19.5A1.5 1.5 0 \
                                                004.5 21h4.75a.75.75 0 00.75-.75V14h4v6.25c0 .414.336.75.75.75h4.75a1.5 1.5 0 \
                                                001.5-1.5v-9.403a1.5 1.5 0 00-.53-1.144l-7.5-6.363z">
                                        </path>
                                    </svg>
                                })
                                .toggle_off(html! {
                                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = "24" height = "24">
                                        <path fill-rule = "evenodd" d = "M11.03 2.59a1.5 1.5 0 011.94 0l7.5 6.363a1.5 1.5 0 01.53 \
                                                1.144V19.5a1.5 1.5 0 01-1.5 1.5h-5.75a.75.75 0 01-.75-.75V14h-2v6.25a.75.75 0 \
                                                01-.75.75H4.5A1.5 1.5 0 013 19.5v-9.403c0-.44.194-.859.53-1.144l7.5-6.363zM12 3.734l-7.5 \
                                                6.363V19.5h5v-6.25a.75.75 0 01.75-.75h3.5a.75.75 0 01.75.75v6.25h5v-9.403L12 3.734z">
                                        </path>
                                    </svg>
                                })
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            IconButton::new()
                                .toggle_on(html! {
                                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = "24" height = "24">
                                        <path d = "M12.97 2.59a1.5 1.5 0 00-1.94 0l-7.5 6.363A1.5 1.5 0 003 10.097V19.5A1.5 1.5 0 \
                                                004.5 21h4.75a.75.75 0 00.75-.75V14h4v6.25c0 .414.336.75.75.75h4.75a1.5 1.5 0 \
                                                001.5-1.5v-9.403a1.5 1.5 0 00-.53-1.144l-7.5-6.363z">
                                        </path>
                                    </svg>
                                })
                                .toggle_off(html! {
                                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = "24" height = "24">
                                        <path fill-rule = "evenodd" d = "M11.03 2.59a1.5 1.5 0 011.94 0l7.5 6.363a1.5 1.5 0 01.53 \
                                                1.144V19.5a1.5 1.5 0 01-1.5 1.5h-5.75a.75.75 0 01-.75-.75V14h-2v6.25a.75.75 0 \
                                                01-.75.75H4.5A1.5 1.5 0 013 19.5v-9.403c0-.44.194-.859.53-1.144l7.5-6.363zM12 3.734l-7.5 \
                                                6.363V19.5h5v-6.25a.75.75 0 01.75-.75h3.5a.75.75 0 01.75.75v6.25h5v-9.403L12 3.734z">
                                        </path>
                                    </svg>
                                })
                                .disabled(true)
                                .on()
                        }
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
                        { Checkbox::new() }
                    </span>
                    <span class = "demo-item">
                        { Checkbox::new().ripple(false) }
                    </span>
                    <span class = "demo-item">
                        { Checkbox::new().disabled(true) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Indeterminate" }</h3>
                    <span class = "demo-item">
                        { Checkbox::new().indeterminate(true) }
                    </span>
                    <span class = "demo-item">
                        { Checkbox::new().indeterminate(true).ripple(false) }
                    </span>
                    <span class = "demo-item">
                        { Checkbox::new().indeterminate(true).disabled(true) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Checked" }</h3>
                    <span class = "demo-item">
                        { Checkbox::new().checked(true) }
                    </span>
                    <span class = "demo-item">
                        { Checkbox::new().checked(true).ripple(false) }
                    </span>
                    <span class = "demo-item">
                        { Checkbox::new().checked(true).disabled(true) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Checkbox::new().id("checkbox-label").label("checkbox") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Checkbox::new().id("checkbox-label-no-ripple").ripple(false).checked(true).label("checkbox") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Checkbox::new().id("checkbox-label-disabled").disabled(true).label("checkbox") }
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
                        { Radio::new().name_of_set("default-set") }
                    </span>
                    <span class = "demo-item">
                        { Radio::new().name_of_set("default-set").checked(true) }
                    </span>
                    <span class = "demo-item">
                        { Radio::new().name_of_set("no-ripple-set").ripple(false) }
                    </span>
                    <span class = "demo-item">
                        { Radio::new().name_of_set("no-ripple-set").ripple(false).checked(true) }
                    </span>
                    <span class = "demo-item">
                        { Radio::new().name_of_set("disabled-set").disabled(true) }
                    </span>
                    <span class = "demo-item">
                        { Radio::new().name_of_set("disabled-set").disabled(true).checked(true) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("label-radio-1").name_of_set("label-set").label("Radio 1") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("label-radio-2").name_of_set("label-set").checked(true).label("Radio 2") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("radio-label-no-ripple-1").name_of_set("label-no-ripple-set").ripple(false).label("No ripple 1") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("radio-label-no-ripple-2").name_of_set("label-no-ripple-set").ripple(false).checked(true).label("No ripple 2") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("radio-label-disabled-1").name_of_set("dlabel-isabled-set").disabled(true).label("Disabled 1") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("radio-label-disabled-2").name_of_set("label-disabled-set").disabled(true).checked(true).label("Disabled 2") }
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
                    <h3 class = "mdc-typography--subtitle1">{ "Default" }</h3>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth("text-field-fullwidth") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled-labeled").label("Filled text field") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined-labeled").label("Outlined text field") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth("text-field-fullwidth-labeled").label("Fullwidth text field") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Helper text" }</h3>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled-labeled-helpertext").label("Filled text field").helper_text("Helper text") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined-labeled-helpertext").label("Outlined text field").helper_text("Helper text") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth("text-field-fullwidth-helpertext").label("Fullwidth text field").helper_text("Helper text") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Char counter" }</h3>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled-labeled-charcounter").label("Filled text field").char_counter(20).helper_text("help") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined-labeled-charcounter").label("Outlined text field").char_counter(20).helper_text("help") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth("text-field-fullwidth-charcounter").label("Fullwidth text field").char_counter(20).helper_text("help") }
                    </span>
                </div>
                 <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Disabled" }</h3>
                    <span class = "demo-item">
                        { TextField::filled("text-field-filled-labeled-disabled").label("Filled disabled text field").disabled() }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined("text-field-outlined-labeled-disabled").label("Outlined disabled text field").disabled() }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth("text-field-fullwidth-disabled").label("Fullwidth disabled text field").disabled() }
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
                            List::ul("single-line-list").items(vec![
                                ListItem::default().text("List Item"),
                                ListItem::default().text("List Item"),
                                ListItem::default().text("List Item"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("single-line-list-icon").items(vec![
                                ListItem::default().icon("wifi").text("Leading Icon"),
                                ListItem::default().icon("bluetooth").text("Leading Icon"),
                                ListItem::default().icon("data_usage").text("Leading Icon"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("single-line-list-trailing-icon").items(vec![
                                ListItem::default().text("Trailing Icon").icon("info"),
                                ListItem::default().text("Trailing Icon").icon("info"),
                                ListItem::default().text("Trailing Icon").icon("info"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("single-line-list-checkbox").items(vec![
                                ListItem::default()
                                    .tile(Checkbox::simple().ripple(false))
                                    .label("Checkbox Item"),
                                ListItem::default()
                                    .tile(Checkbox::simple().ripple(false))
                                    .label("Checkbox Item"),
                                ListItem::default()
                                    .tile(Checkbox::simple().ripple(false))
                                    .label("Checkbox Item"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("single-line-list-radio").items(vec![
                                ListItem::default()
                                    .label("Radio Item")
                                    .tile(Radio::simple().ripple(false).name_of_set("single-line-list-radio")),
                                ListItem::default()
                                    .label("Radio Item")
                                    .tile(Radio::simple().ripple(false).name_of_set("single-line-list-radio")),
                                ListItem::default()
                                    .label("Radio Item")
                                    .tile(Radio::simple().ripple(false).name_of_set("single-line-list-radio")),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("single-line-no-ripple-list").items(vec![
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
                            List::ul("two-line-list").two_line().items(vec![
                                ListItem::default().text("List Item").text("Secondary text"),
                                ListItem::default().text("List Item").text("Secondary text"),
                                ListItem::default().text("List Item").text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("two-line-list-icon").two_line().items(vec![
                                ListItem::default().icon("wifi").text("Leading Icon").text("Secondary text"),
                                ListItem::default().icon("bluetooth").text("Leading Icon").text("Secondary text"),
                                ListItem::default().icon("data_usage").text("Leading Icon").text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("two-line-list-trailing-icon").two_line().items(vec![
                                ListItem::default().text("Trailing Icon").text("Secondary text").icon("info"),
                                ListItem::default().text("Trailing Icon").text("Secondary text").icon("info"),
                                ListItem::default().text("Trailing Icon").text("Secondary text").icon("info"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("two-line-list-checkbox").two_line().items(vec![
                                ListItem::default()
                                    .tile(Checkbox::simple().ripple(false))
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                                ListItem::default()
                                    .tile(Checkbox::simple().ripple(false))
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                                ListItem::default()
                                    .tile(Checkbox::simple().ripple(false))
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("two-line-list-radio").two_line().items(vec![
                                ListItem::default()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::simple().ripple(false).name_of_set("two-line-list-radio")),
                                ListItem::default()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::simple().ripple(false).name_of_set("two-line-list-radio")),
                                ListItem::default()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::simple().ripple(false).name_of_set("two-line-list-radio")),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul("two-line-no-ripple-list").two_line().items(vec![
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
                        List::ul("list-group-1").items(vec![
                            ListItem::default().text("List Item"),
                            ListItem::default().text("List Item"),
                        ])
                    }
                    <h3 class = "mdc-list-group__subheader">{ "List 2" }</h3>
                    {
                        List::ul("list-group-2").items(vec![
                            ListItem::default().text("List Item"),
                            ListItem::default().text("List Item"),
                        ])
                    }
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Two-Line with Leading and Trailing Icon and Divider" }</h3>
                    <span class = "demo-item bordered-list demo-panel">
                        {
                            List::ul("complex-list")
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
                            List::ul("activated-list").single_selection().items(vec![
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
                                Button::new()
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

    fn view_cards(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Regular" }</h3>
                    <span class = "demo-item">
                        {
                            Card::new("card-regular")
                                .class("demo-card regular")
                                .content(CardContent::primary_action(html! {
                                    <>
                                        {
                                            CardContent::media_16_9()
                                                .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                        }
                                        <div class = "demo-card__primary">
                                            <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                            <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                        </div>
                                        <div class = "demo-card__secondary mdc-typography mdc-typography--body2">
                                            { "Visit ten places on our planet that are undergoing the biggest changes today." }
                                        </div>
                                    </>
                                }))
                                .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                    ]))
                                    .action_icons(Html::from_iter(vec![
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                    ]))
                                )
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Text over media" }</h3>
                    <span class = "demo-item">
                        {
                            Card::new("card-over-media")
                                .class("demo-card over-media")
                                .content(CardContent::primary_action(html! {
                                    <>
                                        {
                                            CardContent::media_16_9()
                                                .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                                .media_content(html! {
                                                    <div class = "demo-card__primary">
                                                        <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                                        <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                                    </div>
                                                })
                                        }
                                        <div class = "demo-card__secondary mdc-typography mdc-typography--body2">
                                            { "Visit ten places on our planet that are undergoing the biggest changes today." }
                                        </div>
                                    </>
                                }))
                                .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                    ]))
                                    .action_icons(Html::from_iter(vec![
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                    ]))
                                )
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "With header" }</h3>
                    <span class = "demo-item">
                        {
                            Card::new("card-header")
                                .class("demo-card")
                                .content(html! {
                                    <div class = "demo-card__primary">
                                        <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                        <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                    </div>
                                })
                                .content(CardContent::primary_action(html! {
                                    <>
                                        {
                                            CardContent::media_16_9()
                                                .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                        }
                                        <div class = "demo-card__secondary mdc-typography mdc-typography--body2">
                                            { "Visit ten places on our planet that are undergoing the biggest changes today." }
                                        </div>
                                    </>
                                }))
                                .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                    ]))
                                    .action_icons(Html::from_iter(vec![
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                    ]))
                                )
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Horizontal" }</h3>
                    <span class = "demo-item">
                        {
                            Card::new("card-horizontal")
                                .class("demo-card")
                                .content(CardContent::primary_action(html! {
                                    <>
                                        {
                                            CardContent::media_square()
                                                .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                        }
                                        <div class = "demo-card__primary">
                                            <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                            <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                        </div>
                                    </>
                                }).class("primary-action-horizontal"))
                                .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                        Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                    ]))
                                    .action_icons(Html::from_iter(vec![
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                        IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                    ]))
                                )
                        }
                    </span>
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
