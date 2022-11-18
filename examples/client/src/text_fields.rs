use yew::{html, Html};
use yew_mdc_widgets::{HelperText, MdcWidget, TextField};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Default" }</h3>
                <span class = "demo-item">
                    { TextField::filled() }
                </span>
                <span class = "demo-item">
                    { TextField::outlined() }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Labeled" }</h3>
                <span class = "demo-item">
                    { TextField::filled().id("text-field-filled-labeled").label("Filled text field") }
                </span>
                <span class = "demo-item">
                    { TextField::outlined().id("text-field-outlined-labeled").label("Outlined text field") }
                </span>
                <span class = "demo-item">
                    { TextField::outlined().id("text-field-prefilled-labeled").label("Outlined text field").value("Pre-filled value") }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Helper text" }</h3>
                <span class = "demo-item">
                    {
                        TextField::filled()
                            .id("text-field-filled-labeled-helpertext")
                            .label("Filled text field")
                            .helper_text(HelperText::new("Helper text"))
                    }
                </span>
                <span class = "demo-item">
                    {
                        TextField::outlined()
                            .id("text-field-outlined-labeled-helpertext")
                            .label("Outlined text field")
                            .helper_text(HelperText::new("Helper text").persistent())
                    }
                </span>
                <span class = "demo-item">
                    {
                        TextField::outlined()
                            .id("text-field-prefilled-helpertext")
                            .label("Outlined text field")
                            .value("Pre-filled value")
                            .helper_text(HelperText::new("Helper text"))
                    }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Char counter" }</h3>
                <span class = "demo-item">
                    {
                        TextField::filled()
                            .id("text-field-filled-labeled-charcounter")
                            .label("Filled text field")
                            .char_counter(20)
                            .helper_text(HelperText::new("Helper text").persistent())
                    }
                </span>
                <span class = "demo-item">
                    {
                        TextField::outlined()
                            .id("text-field-outlined-labeled-charcounter")
                            .label("Outlined text field")
                            .char_counter(20)
                            .helper_text(HelperText::new("Helper text"))
                    }
                </span>
                <span class = "demo-item">
                    {
                        TextField::filled()
                            .id("text-field-prefilled-charcounter")
                            .label("Filled text field")
                            .value("Pre-filled value")
                            .char_counter(20)
                            .helper_text(HelperText::new("Helper text"))
                    }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "With icon" }</h3>
                <span class = "demo-item">
                    { TextField::filled().id("text-field-filled-icon").label("Filled text field").leading_icon("event") }
                </span>
                <span class = "demo-item">
                    { TextField::outlined().id("text-field-outlined-icon").label("Outlined text field").leading_icon("event").trailing_icon("delete") }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Disabled" }</h3>
                <span class = "demo-item">
                    { TextField::filled().id("text-field-filled-labeled-disabled").label("Filled disabled text field").disabled() }
                </span>
                <span class = "demo-item">
                    { TextField::outlined().id("text-field-outlined-labeled-disabled").label("Outlined disabled text field").disabled() }
                </span>
                <span class = "demo-item" style = "width: 100%">
                    { TextField::outlined().id("text-field-prefilled-disabled").label("Outlined disabled text field").value("Pre-filled value").disabled() }
                </span>
            </div>
        </div>
    }
}
