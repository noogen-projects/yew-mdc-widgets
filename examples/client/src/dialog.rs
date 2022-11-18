use yew::{html, Html};
use yew_mdc_widgets::{Button, Dialog, List, ListItem, MdcWidget, Menu, Radio};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Alert" }</h3>
                <span class = "demo-item demo-list">
                    <div id = "demo-menu" class = { Menu::ANCHOR_CLASS }>
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
                    <div id = "demo-menu" class = { Menu::ANCHOR_CLASS }>
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
                    <div id = "demo-menu" class = { Menu::ANCHOR_CLASS }>
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
