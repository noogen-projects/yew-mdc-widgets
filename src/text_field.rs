use yew::{html, Callback, Html, MouseEvent};

pub fn text_field(id: impl AsRef<str>, label: impl AsRef<str>) -> Html {
    let id = id.as_ref();
    let input_id = format!("{}-input", id);
    let label = label.as_ref();
    let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", id);

    html! {
        <div id = id class = "mdc-text-field mdc-text-field--outlined">
            <input id = input_id class = "mdc-text-field__input"/>
            <div class = "mdc-notched-outline">
                <div class = "mdc-notched-outline__leading"></div>
                <div class = "mdc-notched-outline__notch">
                    <label for = input_id class = "mdc-floating-label">{ label }</label>
                </div>
                <div class = "mdc-notched-outline__trailing"></div>
            </div>
            <script>{ mdc_init }</script>
        </div>
    }
}
