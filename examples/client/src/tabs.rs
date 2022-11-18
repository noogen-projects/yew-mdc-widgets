use yew::{html, Html};
use yew_mdc_widgets::{MdcWidget, Tab, TabBar};

pub fn view() -> Html {
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
