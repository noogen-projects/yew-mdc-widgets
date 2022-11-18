use yew::{html, Html};
use yew_mdc_widgets::{Button, IconButton, MdcWidget, Snackbar};

pub fn view() -> Html {
    let baseline = Snackbar::new()
        .id("baseline-snackbar")
        .label("Baseline snackbar test banner")
        .dismiss(IconButton::new().icon("close"));
    let open_baseline_button = Button::raised()
        .class(Snackbar::DEMO_BUTTON_CLASS)
        .label("Baseline")
        .on_click(|_| Snackbar::open_existing("baseline-snackbar"));
    let leading = Snackbar::leading()
        .id("leading-snackbar")
        .label("Leading snackbar test banner")
        .dismiss(IconButton::new().icon("close"));
    let open_leading_button = Button::raised()
        .class(Snackbar::DEMO_BUTTON_CLASS)
        .label("Leading")
        .on_click(|_| Snackbar::open_existing("leading-snackbar"));
    let stacked = Snackbar::stacked()
        .id("stacked-snackbar")
        .label("Stacked snackbar test banner with a button")
        .action(Button::new().label("Retry"))
        .dismiss(IconButton::new().icon("close"));
    let open_stacked_button = Button::raised()
        .class(Snackbar::DEMO_BUTTON_CLASS)
        .label("Stacked")
        .on_click(|_| Snackbar::open_existing("stacked-snackbar"));

    html! {
        <div>
            <div>
                <span class = "demo-item">
                    { open_baseline_button }
                    { baseline }
                </span>
                <span class = "demo-item">
                    { open_leading_button }
                    { leading }
                </span>
                <span class = "demo-item">
                    { open_stacked_button }
                    { stacked }
                </span>
            </div>
        </div>
    }
}
