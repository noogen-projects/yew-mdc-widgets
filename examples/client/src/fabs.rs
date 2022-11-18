use yew::{html, Html};
use yew_mdc_widgets::{dom, Element, Fab, MdcWidget};

pub fn view() -> Html {
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
                        dom::existing::get_element_by_id::<Element>("exited_fab").class_list().add_1(Fab::EXITED_CLASS).ok();
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
                        dom::existing::get_element_by_id::<Element>("exited_fab_mini").class_list().add_1(Fab::EXITED_CLASS).ok();
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
                        dom::existing::get_element_by_id::<Element>("exited_fab_extended").class_list().add_1(Fab::EXITED_CLASS).ok();
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
