#![recursion_limit = "4096"]

use std::iter::FromIterator;

use yew::{html, initialize, run_loop, utils, App, Component, ComponentLink, Html};
use yew_mdc_widgets::{
    auto_init,
    utils::dom::{get_exist_element_by_id, JsObjectAccess},
    Button, ButtonStyle, Card, CardContent, Checkbox, Chip, ChipSet, DataTable, Dialog, Drawer, Element, Fab,
    IconButton, List, ListItem, MdcWidget, Menu, Radio, Switch, Tab, TabBar, TableCell, TextField, TopAppBar,
};

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
        let contents = vec![
            ListItem::link("#buttons").text("Buttons").attr("tabindex", "0"),
            ListItem::link("#icon_buttons").text("Icon buttons"),
            ListItem::link("#fabs").text("Floating Action Buttons"),
            ListItem::link("#checkboxes").text("Checkboxes"),
            ListItem::link("#radio_buttons").text("Radio buttons"),
            ListItem::link("#switch").text("Switch"),
            ListItem::link("#chips").text("Chips"),
            ListItem::link("#text_fields").text("Text fields"),
            ListItem::link("#lists").text("Lists"),
            ListItem::link("#menu").text("Menu"),
            ListItem::link("#data_tables").text("Data tables"),
            ListItem::link("#dialog").text("Dialog"),
            ListItem::link("#tabs").text("Tabs"),
            ListItem::link("#cards").text("Cards"),
        ];

        let drawer = Drawer::new()
            .id("app-drawer")
            .title(html! { <h3 tabindex = 0>{ "Widgets" }</h3> })
            .modal()
            .content(
                List::nav()
                    .items(contents.clone().into_iter().map(|item| {
                        item.on_click(|_| {
                            let drawer = get_exist_element_by_id::<Element>("app-drawer").get(Drawer::MDC_TYPE_NAME);
                            drawer.set("open", false);
                        })
                    }))
                    .markup_only(),
            );

        let top_app_bar = TopAppBar::new()
            .id("top-app-bar")
            .title("Yew MDC Widgets")
            .navigation_item(IconButton::new().icon("menu"))
            .enable_shadow_when_scroll_window()
            .on_navigation(|_| {
                let drawer = get_exist_element_by_id::<Element>("app-drawer").get(Drawer::MDC_TYPE_NAME);
                let opened = drawer.get("open").as_bool().unwrap_or(false);
                drawer.set("open", !opened);
            });

        html! {
            <>
                { drawer }
                <div class="mdc-drawer-scrim"></div>

                <div class = vec!["app-content", Drawer::APP_CONTENT_CLASS]>
                    { top_app_bar }

                    <div class = "mdc-top-app-bar--fixed-adjust">
                        <div class = "demo-content">
                            <h1 class = "demo-title mdc-typography--headline5">{ "Material design components" }</h1>
                            { List::nav().items(contents) }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "buttons"></a>{ "Buttons" }</h2>
                            { self.view_buttons() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "icon_buttons"></a>{ "Icon buttons" }</h2>
                            { self.view_icon_buttons() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "fabs"></a>{ "Floating Action Buttons" }</h2>
                            { self.view_fabs() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "checkboxes"></a>{ "Checkboxes" }</h2>
                            { self.view_checkboxes() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "radio_buttons"></a>{ "Radio buttons" }</h2>
                            { self.view_radio_buttons() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "switch"></a>{ "Switch" }</h2>
                            { self.view_switch() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "chips"></a>{ "Chips" }</h2>
                            { self.view_chips() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "text_fields"></a>{ "Text fields" }</h2>
                            { self.view_text_fields() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "lists"></a>{ "Lists" }</h2>
                            { self.view_lists() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "menu"></a>{ "Menu" }</h2>
                            { self.view_menu() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "data_tables"></a>{ "Data tables" }</h2>
                            { self.view_data_tables() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "dialog"></a>{ "Dialog" }</h2>
                            { self.view_dialog() }

                            <h2 class = "demo-title mdc-typography--headline6"><a name = "tabs"></a>{ "Tabs" }</h2>
                            { self.view_tabs() }

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
                        { Button::outlined().label("Default") }
                    </span>
                    <span class = "demo-item">
                        { Button::simple().style(ButtonStyle::Outlined).label("No ripple") }
                    </span>
                    <span class = "demo-item">
                        { Button::outlined().disabled(true).label("Disabled") }
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
                        { Button::raised().disabled(true).label("Disabled") }
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
                        { Button::unelevated().disabled(true).label("Disabled") }
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

    fn view_fabs(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Regular FAB" }</h3>
                    <span class = "demo-item">
                        { Fab::new().icon("add") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().ripple(false).icon("add") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().id("exited_fab").icon("add").on_click(|_| {
                            get_exist_element_by_id::<Element>("exited_fab").class_list().add_1(Fab::EXITED_CLASS).ok();
                        }) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Mini FAB" }</h3>
                    <span class = "demo-item">
                        { Fab::new().mini().icon("add") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().ripple(false).mini().icon("add") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().mini().id("exited_fab_mini").icon("add").on_click(|_| {
                            get_exist_element_by_id::<Element>("exited_fab_mini").class_list().add_1(Fab::EXITED_CLASS).ok();
                        }) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Extended FAB" }</h3>
                    <span class = "demo-item">
                        { Fab::new().label("favorite") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().icon("favorite_border").label("favorite") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().label("favorite").icon("favorite_border") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().ripple(false).icon("favorite_border").label("favorite") }
                    </span>
                    <span class = "demo-item">
                        { Fab::new().id("exited_fab_extended").icon("favorite_border").label("favorite").on_click(|_| {
                            get_exist_element_by_id::<Element>("exited_fab_extended").class_list().add_1(Fab::EXITED_CLASS).ok();
                        }) }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Svg Fab" }</h3>
                    <span class = "demo-item">
                        {
                            Fab::new()
                                .item(html! {
                                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = "24" height = "24">
                                        <path d = "M12.97 2.59a1.5 1.5 0 00-1.94 0l-7.5 6.363A1.5 1.5 0 003 10.097V19.5A1.5 1.5 0 \
                                                004.5 21h4.75a.75.75 0 00.75-.75V14h4v6.25c0 .414.336.75.75.75h4.75a1.5 1.5 0 \
                                                001.5-1.5v-9.403a1.5 1.5 0 00-.53-1.144l-7.5-6.363z">
                                        </path>
                                    </svg>
                                })
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            Fab::new()
                                .item(html! {
                                    <svg xmlns = "http://www.w3.org/2000/svg" viewBox = "0 0 24 24" width = "24" height = "24">
                                        <path d = "M12.97 2.59a1.5 1.5 0 00-1.94 0l-7.5 6.363A1.5 1.5 0 003 10.097V19.5A1.5 1.5 0 \
                                                004.5 21h4.75a.75.75 0 00.75-.75V14h4v6.25c0 .414.336.75.75.75h4.75a1.5 1.5 0 \
                                                001.5-1.5v-9.403a1.5 1.5 0 00-.53-1.144l-7.5-6.363z">
                                        </path>
                                    </svg>
                                })
                                .label("Labeled")
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
                            { Radio::new().id("radio-label-1").name_of_set("label-set").label("Radio 1") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Radio::new().id("radio-label-2").name_of_set("label-set").checked(true).label("Radio 2") }
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

    fn view_switch(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Default" }</h3>
                    <span class = "demo-item">
                        { Switch::new() }
                    </span>
                    <span class = "demo-item">
                        { Switch::new().on() }
                    </span>
                    <span class = "demo-item">
                        { Switch::new().disabled() }
                    </span>
                    <span class = "demo-item">
                        { Switch::new().disabled().on() }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Switch::new().id("switch-label-1").label("Switch 1") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Switch::new().id("switch-label-2").on().label("Switch 2") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Switch::new().id("switch-label-disabled-1").disabled().label("Disabled 1") }
                        </div>
                    </span>
                    <span class = "demo-item">
                        <div class = "mdc-form-field">
                            { Switch::new().id("switch-label-disabled-2").disabled().on().label("Disabled 2") }
                        </div>
                    </span>
                </div>
            </div>
        }
    }

    fn view_chips(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Basic" }</h3>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .chip(Chip::simple().tab_index(0).text("Chip One"))
                                .chip(Chip::simple().text("Chip Two"))
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "With icons" }</h3>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .chip(Chip::simple().tab_index(0).icon("bookmark").text("Chip One"))
                                .chip(Chip::simple().icon("face").text("Chip Two"))
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .chip(Chip::simple().tab_index(0).text("Chip One").icon("cancel"))
                                .chip(Chip::simple().text("Chip Two").icon("close"))
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Choice" }</h3>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .choice()
                                .chip(Chip::simple().tab_index(0).text("Chip One"))
                                .chip(Chip::simple().text("Chip Two"))
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .choice()
                                .chip(Chip::simple().tab_index(0).icon("event").text("Chip One"))
                                .chip(Chip::simple().text("Chip Two").icon("cancel"))
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Filter" }</h3>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .filter()
                                .chip(Chip::simple().tab_index(0).checkmark().text("Chip One"))
                                .chip(Chip::simple().checkmark().text("Chip Two"))
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .filter()
                                .chip(Chip::simple().tab_index(0).icon("event").checkmark().text("Chip One"))
                                .chip(Chip::simple().checkmark().text("Chip Two").icon("cancel"))
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Selected" }</h3>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .filter()
                                .chip(Chip::simple().tab_index(0).checkmark().selected().text("Chip One"))
                                .chip(Chip::simple().checkmark().text("Chip Two"))
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .filter()
                                .chip(Chip::simple().tab_index(0).selected().icon("event").checkmark().text("Chip One"))
                                .chip(Chip::simple().icon("face").checkmark().text("Chip Two"))
                        }
                    </span>
                    <span class = "demo-item">
                        {
                            ChipSet::new()
                                .choice()
                                .chip(Chip::simple().tab_index(0).icon("event").text("Chip One").selected())
                                .chip(Chip::simple().text("Chip Two").icon("cancel"))
                        }
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
                        { TextField::filled() }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined() }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth() }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                    <span class = "demo-item">
                        { TextField::filled().id("text-field-filled-labeled").label("Filled text field") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined().id("text-field-outlined-labeled").label("Outlined text field") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth().id("text-field-fullwidth-labeled").label("Fullwidth text field") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Helper text" }</h3>
                    <span class = "demo-item">
                        { TextField::filled().id("text-field-filled-labeled-helpertext").label("Filled text field").helper_text("Helper text") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined().id("text-field-outlined-labeled-helpertext").label("Outlined text field").helper_text("Helper text") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth().id("text-field-fullwidth-helpertext").label("Fullwidth text field").helper_text("Helper text") }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Char counter" }</h3>
                    <span class = "demo-item">
                        { TextField::filled().id("text-field-filled-labeled-charcounter").label("Filled text field").char_counter(20).helper_text("help") }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined().id("text-field-outlined-labeled-charcounter").label("Outlined text field").char_counter(20).helper_text("help") }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth().id("text-field-fullwidth-charcounter").label("Fullwidth text field").char_counter(20).helper_text("help") }
                    </span>
                </div>
                 <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Disabled" }</h3>
                    <span class = "demo-item">
                        { TextField::filled().id("text-field-filled-labeled-disabled").label("Filled disabled text field").disabled() }
                    </span>
                    <span class = "demo-item">
                        { TextField::outlined().id("text-field-outlined-labeled-disabled").label("Outlined disabled text field").disabled() }
                    </span>
                    <span class = "demo-item" style = "width: 100%">
                        { TextField::fullwidth().id("text-field-fullwidth-disabled").label("Fullwidth disabled text field").disabled() }
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
                            List::ul().items(vec![
                                ListItem::new().text("List Item"),
                                ListItem::new().text("List Item"),
                                ListItem::new().text("List Item"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().items(vec![
                                ListItem::new().icon("wifi").text("Leading Icon"),
                                ListItem::new().icon("bluetooth").text("Leading Icon"),
                                ListItem::new().icon("data_usage").text("Leading Icon"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().items(vec![
                                ListItem::new().text("Trailing Icon").icon("info"),
                                ListItem::new().text("Trailing Icon").icon("info"),
                                ListItem::new().text("Trailing Icon").icon("info"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().items(vec![
                                ListItem::new()
                                    .tile(Checkbox::simple().ripple(false))
                                    .label("Checkbox Item"),
                                ListItem::new()
                                    .tile(Checkbox::simple().ripple(false))
                                    .label("Checkbox Item"),
                                ListItem::new()
                                    .tile(Checkbox::simple().ripple(false))
                                    .label("Checkbox Item"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().items(vec![
                                ListItem::new()
                                    .label("Radio Item")
                                    .tile(Radio::simple().ripple(false).name_of_set("single-line-list-radio")),
                                ListItem::new()
                                    .label("Radio Item")
                                    .tile(Radio::simple().ripple(false).name_of_set("single-line-list-radio")),
                                ListItem::new()
                                    .label("Radio Item")
                                    .tile(Radio::simple().ripple(false).name_of_set("single-line-list-radio")),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().items(vec![
                                ListItem::simple().text("No ripple"),
                                ListItem::simple().text("No ripple"),
                                ListItem::simple().text("No ripple"),
                            ])
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Two-Line" }</h3>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().two_line().items(vec![
                                ListItem::new().text("List Item").text("Secondary text"),
                                ListItem::new().text("List Item").text("Secondary text"),
                                ListItem::new().text("List Item").text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().two_line().items(vec![
                                ListItem::new().icon("wifi").text("Leading Icon").text("Secondary text"),
                                ListItem::new().icon("bluetooth").text("Leading Icon").text("Secondary text"),
                                ListItem::new().icon("data_usage").text("Leading Icon").text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().two_line().items(vec![
                                ListItem::new().text("Trailing Icon").text("Secondary text").icon("info"),
                                ListItem::new().text("Trailing Icon").text("Secondary text").icon("info"),
                                ListItem::new().text("Trailing Icon").text("Secondary text").icon("info"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().two_line().items(vec![
                                ListItem::new()
                                    .tile(Checkbox::simple().ripple(false))
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                                ListItem::new()
                                    .tile(Checkbox::simple().ripple(false))
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                                ListItem::new()
                                    .tile(Checkbox::simple().ripple(false))
                                    .text("Checkbox Item")
                                    .text("Secondary text"),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().two_line().items(vec![
                                ListItem::new()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::simple().ripple(false).name_of_set("two-line-list-radio")),
                                ListItem::new()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::simple().ripple(false).name_of_set("two-line-list-radio")),
                                ListItem::new()
                                    .text("Radio Item")
                                    .text("Secondary text")
                                    .tile(Radio::simple().ripple(false).name_of_set("two-line-list-radio")),
                            ])
                        }
                    </span>
                    <span class = "demo-item demo-list">
                        {
                            List::ul().two_line().items(vec![
                                ListItem::simple().text("No ripple").text("Secondary text"),
                                ListItem::simple().text("No ripple").text("Secondary text"),
                                ListItem::simple().text("No ripple").text("Secondary text"),
                            ])
                        }
                    </span>
                </div>
                <h3 class = "mdc-typography--subtitle1">{ "List Groups" }</h3>
                <div class = "demo-item mdc-list-group demo-group-list demo-panel">
                    <h3 class = "mdc-list-group__subheader">{ "List 1" }</h3>
                    {
                        List::ul().items(vec![
                            ListItem::new().text("List Item"),
                            ListItem::new().text("List Item"),
                        ])
                    }
                    <h3 class = "mdc-list-group__subheader">{ "List 2" }</h3>
                    {
                        List::ul().items(vec![
                            ListItem::new().text("List Item"),
                            ListItem::new().text("List Item"),
                        ])
                    }
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Two-Line with Leading and Trailing Icon and Divider" }</h3>
                    <span class = "demo-item bordered-list demo-panel">
                        {
                            List::ul()
                                .two_line()
                                .avatar()
                                .item(ListItem::new().icon("folder").text("Dog Photos").text("9 Jan 2018").icon("info"))
                                .divider_inset_leading()
                                .item(ListItem::new().icon("folder").text("Cat Photos").text("22 Dec 2017").icon("info"))
                                .divider()
                                .items(vec![
                                    ListItem::new().icon("folder").text("Potatoes").text("30 Noc 2017").icon("info"),
                                    ListItem::new().icon("folder").text("Carrots").text("17 Oct 2017").icon("info"),
                                ])
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "List with activated item" }</h3>
                    <span class = "demo-item bordered-list demo-panel">
                        {
                            List::ul().id("activated-list").single_selection().items(vec![
                                ListItem::new().icon("inbox").text("Inbox"),
                                ListItem::new().icon("star").text("Star").selected(true),
                                ListItem::new().icon("send").text("Send"),
                                ListItem::new().icon("drafts").text("Drafts"),
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
                                    .on_click(|_| Menu::open_existing("simple-menu"))
                            }
                            {
                                Menu::new().id("simple-menu").items(vec![
                                    ListItem::new().text("Menu Item"),
                                    ListItem::new().text("Menu Item"),
                                    ListItem::new().text("Menu Item"),
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

    fn view_dialog(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Alert" }</h3>
                    <span class = "demo-item demo-list">
                        <div id = "demo-menu" class = Menu::ANCHOR_CLASS>
                            {
                                 Button::new()
                                    .label("Open Alert Dialog")
                                    .on_click(|_| Dialog::open_existing("alert-dialog"))
                            }
                            {
                                Dialog::new()
                                    .id("alert-dialog")
                                    .content_item("Discard draft?")
                                    .action(
                                        Button::new()
                                            .label("Cancel")
                                            .class(Dialog::BUTTON_CLASS)
                                            .on_click(|_| Dialog::close_existing("alert-dialog")),
                                    )
                                    .action(
                                        Button::new()
                                            .label("Discard")
                                            .class(Dialog::BUTTON_CLASS)
                                            .on_click(|_| Dialog::close_existing("alert-dialog")),
                                    )
                            }
                        </div>
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Simple" }</h3>
                    <span class = "demo-item demo-list">
                        <div id = "demo-menu" class = Menu::ANCHOR_CLASS>
                            {
                                 Button::new()
                                    .label("Open Simple Dialog")
                                    .on_click(|_| Dialog::open_existing("simple-dialog"))
                            }
                            {
                                Dialog::new()
                                    .id("simple-dialog")
                                    .title(html! { <h2> { "Dialog header" } </h2> })
                                    .content_item("Choose item")
                                    .content_item(
                                        List::ul().items(vec![
                                            ListItem::new().text("List Item 1"),
                                            ListItem::new().text("List Item 2"),
                                            ListItem::new().text("List Item 3"),
                                        ])
                                    )
                                    .action(
                                        Button::new()
                                            .label("Cancel")
                                            .class(Dialog::BUTTON_CLASS)
                                            .on_click(|_| Dialog::close_existing("simple-dialog")),
                                    )
                            }
                        </div>
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Confirmation" }</h3>
                    <span class = "demo-item demo-list">
                        <div id = "demo-menu" class = Menu::ANCHOR_CLASS>
                            {
                                 Button::new()
                                    .label("Open Confirmation Dialog")
                                    .on_click(|_| Dialog::open_existing("confirmation-dialog"))
                            }
                            {
                                Dialog::new()
                                    .id("confirmation-dialog")
                                    .title(html! { <h2> { "Dialog header" } </h2> })
                                    .content_item("Choose item")
                                    .content_item(
                                        List::ul().items(vec![
                                            ListItem::new()
                                                .label("Radio Item 1")
                                                .tile(Radio::simple().ripple(false).name_of_set("dialog-list-radio")),
                                            ListItem::new()
                                                .label("Radio Item 2")
                                                .tile(Radio::simple().ripple(false).name_of_set("dialog-list-radio")),
                                            ListItem::new()
                                                .label("Radio Item 3")
                                                .tile(Radio::simple().ripple(false).name_of_set("dialog-list-radio")),
                                        ])
                                    )
                                    .action(
                                        Button::new()
                                            .label("Cancel")
                                            .class(Dialog::BUTTON_CLASS)
                                            .on_click(|_| Dialog::close_existing("confirmation-dialog")),
                                    )
                                    .action(
                                        Button::new()
                                            .label("Ok")
                                            .class(Dialog::BUTTON_CLASS)
                                            .on_click(|_| Dialog::close_existing("confirmation-dialog")),
                                    )
                            }
                        </div>
                    </span>
                </div>
            </div>
        }
    }

    fn view_tabs(&self) -> Html {
        html! {
            <div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Basic" }</h3>
                    <span class = "demo-item">
                        {
                            TabBar::new().id("tab-bar-basic")
                                .tab(Tab::new().label("Tab One").tab_index(0))
                                .tab(Tab::new().label("Tab Two"))
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Leading Icon" }</h3>
                    <span class = "demo-item" style = "width:800px">
                        {
                            TabBar::new().id("tab-bar-icons")
                                .tab(Tab::new().icon("star").label("Tab One").tab_index(0))
                                .tab(Tab::new().icon("favorite").label("Tab Two"))
                                .tab(Tab::new().icon("bookmark").label("Tab Three"))
                                .tab(Tab::new().icon("near_me").label("Tab Four"))
                                .tab(Tab::new().icon("free_breakfast").label("Tab Five"))
                                .tab(Tab::new().icon("watch_later").label("Tab Six"))
                                .tab(Tab::new().icon("event").label("Tab Seven"))
                        }
                    </span>
                </div>
                <div>
                    <h3 class = "mdc-typography--subtitle1">{ "Stacked Icon" }</h3>
                    <span class = "demo-item">
                        {
                            TabBar::new().id("tab-bar-stacked")
                                .tab(Tab::new().icon("star").label("Tab One").stacked().tab_index(0))
                                .tab(Tab::new().icon("favorite").label("Tab Two").stacked())
                                .tab(Tab::new().icon("bookmark").label("Tab Three").stacked())
                                .tab(Tab::new().icon("near_me").label("Tab Four").stacked())
                                .tab(Tab::new().icon("free_breakfast").label("Tab Five").stacked())
                                .tab(Tab::new().icon("watch_later").label("Tab Six").stacked())
                        }
                    </span>
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
