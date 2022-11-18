use yew::{html, Html};
use yew_mdc_widgets::IconButton;

pub fn view() -> Html {
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
