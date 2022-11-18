use yew::{html, Html};
use yew_mdc_widgets::{Chip, ChipSet};

pub fn view() -> Html {
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
