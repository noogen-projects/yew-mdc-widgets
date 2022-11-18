use yew::{html, Html};
use yew_mdc_widgets::{MdcWidget, Switch};

pub fn view() -> Html {
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
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Icons" }</h3>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Switch::new().id("switch-icons-label-1").label("Switch 1").icon_on_default().icon_off_default() }
                    </div>
                </span>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Switch::new().id("switch-icons-label-2").on().label("Switch 2").icon_on_default().icon_off_default() }
                    </div>
                </span>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Switch::new().id("switch-icons-label-disabled-1").disabled().label("Disabled 1").icon_on_default().icon_off_default() }
                    </div>
                </span>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Switch::new().id("switch-icons-label-disabled-2").disabled().on().label("Disabled 2").icon_on_default().icon_off_default() }
                    </div>
                </span>
            </div>
        </div>
    }
}
