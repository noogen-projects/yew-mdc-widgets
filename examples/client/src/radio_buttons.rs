use yew::{html, Html};
use yew_mdc_widgets::{MdcWidget, Radio};

pub fn view() -> Html {
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
