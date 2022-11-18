use yew::{html, Html};
use yew_mdc_widgets::{Button, ListItem, MdcWidget, Menu};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Simple" }</h3>
                <span class = "demo-item demo-list">
                    <div id = "demo-menu" class = { Menu::ANCHOR_CLASS }>
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
