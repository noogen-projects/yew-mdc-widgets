use yew::{html, Html};
use yew_mdc_widgets::{Checkbox, List, ListItem, MdcWidget, Radio};

pub fn view() -> Html {
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
                            ListItem::simple().interactive().text("No ripple"),
                            ListItem::simple().interactive().text("No ripple"),
                            ListItem::simple().interactive().text("No ripple"),
                        ])
                    }
                </span>
                <span class = "demo-item demo-list">
                    {
                        List::simple_ul().items(vec![
                            ListItem::simple().text("Fixed"),
                            ListItem::simple().text("Fixed"),
                            ListItem::simple().text("Fixed"),
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
                            ListItem::simple().interactive().text("No ripple").text("Secondary text"),
                            ListItem::simple().interactive().text("No ripple").text("Secondary text"),
                            ListItem::simple().interactive().text("No ripple").text("Secondary text"),
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
