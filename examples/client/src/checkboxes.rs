use yew::{html, Html};
use yew_mdc_widgets::{Checkbox, MdcWidget};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Unchecked" }</h3>
                <span class = "demo-item">
                    { Checkbox::new() }
                </span>
                <span class = "demo-item">
                    { Checkbox::new().ripple(false) }
                </span>
                <span class = "demo-item">
                    { Checkbox::new().disabled(true) }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Indeterminate" }</h3>
                <span class = "demo-item">
                    { Checkbox::new().indeterminate(true) }
                </span>
                <span class = "demo-item">
                    { Checkbox::new().indeterminate(true).ripple(false) }
                </span>
                <span class = "demo-item">
                    { Checkbox::new().indeterminate(true).disabled(true) }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Checked" }</h3>
                <span class = "demo-item">
                    { Checkbox::new().checked(true) }
                </span>
                <span class = "demo-item">
                    { Checkbox::new().checked(true).ripple(false) }
                </span>
                <span class = "demo-item">
                    { Checkbox::new().checked(true).disabled(true) }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Checkbox::new().id("checkbox-label").label("checkbox") }
                    </div>
                </span>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Checkbox::new().id("checkbox-label-no-ripple").ripple(false).checked(true).label("checkbox") }
                    </div>
                </span>
                <span class = "demo-item">
                    <div class = "mdc-form-field">
                        { Checkbox::new().id("checkbox-label-disabled").disabled(true).label("checkbox") }
                    </div>
                </span>
            </div>
        </div>
    }
}
