#![recursion_limit = "4096"]

use yew::{classes, html, Component, Context, Html};
use yew_mdc_widgets::{
    auto_init,
    dom::{self, existing::JsObjectAccess},
    drawer, Drawer, Element, IconButton, List, ListItem, MdcWidget, TopAppBar,
};

mod buttons;
mod cards;
mod checkboxes;
mod chips;
mod data_tables;
mod dialog;
mod fabs;
mod icon_buttons;
mod linear_progress;
mod lists;
mod menu;
mod radio_buttons;
mod snackbars;
mod switch;
mod tabs;
mod text_fields;

struct Root;

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let contents = vec![
            ListItem::link("#buttons").text("Buttons").tab_index(0),
            ListItem::link("#icon_buttons").text("Icon buttons"),
            ListItem::link("#fabs").text("Floating Action Buttons"),
            ListItem::link("#checkboxes").text("Checkboxes"),
            ListItem::link("#radio_buttons").text("Radio buttons"),
            ListItem::link("#switch").text("Switch"),
            ListItem::link("#chips").text("Chips"),
            ListItem::link("#snackbars").text("Snackbars"),
            ListItem::link("#linear_progress").text("LinearProgress"),
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
                            let drawer =
                                dom::existing::get_element_by_id::<Element>("app-drawer").get(drawer::mdc::TYPE_NAME);
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
                let drawer = dom::existing::get_element_by_id::<Element>("app-drawer").get(drawer::mdc::TYPE_NAME);
                let opened = drawer.get("open").as_bool().unwrap_or(false);
                drawer.set("open", !opened);
            });

        html! {
            <>
                { drawer }
                <div class = "mdc-drawer-scrim"></div>

                <div class = { classes!("app-content", Drawer::APP_CONTENT_CLASS) }>
                    { top_app_bar }

                    <div class = { TopAppBar::FIXED_ADJUST_CLASS }>
                        <div class = "demo-content">
                            <h1 class = "demo-title mdc-typography--headline5">{ "Material design components" }</h1>
                            { List::nav().items(contents) }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "buttons"></a>{ "Buttons" }</h2>
                            { buttons::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "icon_buttons"></a>{ "Icon buttons" }</h2>
                            { icon_buttons::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "fabs"></a>{ "Floating Action Buttons" }</h2>
                            { fabs::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "checkboxes"></a>{ "Checkboxes" }</h2>
                            { checkboxes::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "radio_buttons"></a>{ "Radio buttons" }</h2>
                            { radio_buttons::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "switch"></a>{ "Switch" }</h2>
                            { switch::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "chips"></a>{ "Chips" }</h2>
                            { chips::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "snackbars"></a>{ "Snackbars" }</h2>
                            { snackbars::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "linear_progress"></a>{ "LinearProgress" }</h2>
                            { linear_progress::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "text_fields"></a>{ "Text fields" }</h2>
                            { text_fields::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "lists"></a>{ "Lists" }</h2>
                            { lists::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "menu"></a>{ "Menu" }</h2>
                            { menu::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "data_tables"></a>{ "Data tables" }</h2>
                            { data_tables::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "dialog"></a>{ "Dialog" }</h2>
                            { dialog::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "tabs"></a>{ "Tabs" }</h2>
                            { tabs::view() }

                            <h2 class = "demo-title mdc-typography--headline6"><a href = "#" name = "cards"></a>{ "Cards" }</h2>
                            { cards::view() }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        auto_init();
    }
}

fn main() {
    let root = dom::existing::get_element_by_id::<Element>("root");
    yew::Renderer::<Root>::with_root(root).render();
}
