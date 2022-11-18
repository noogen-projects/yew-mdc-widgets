use yew::{html, Html};
use yew_mdc_widgets::{LinearProgress, MdcWidget};

pub fn view() -> Html {
    let determinate_linear_progress = LinearProgress::new()
        .id("linear-progress-determinate-bar")
        .progress(0.3)
        .buffer(0.6);

    let indeterminate_linear_progress = LinearProgress::new()
        .id("linear-progress-indeterminate-bar")
        .indeterminate();

    html! {
        <div>
            <div>
                <span class = "demo-item" style = "width: 100%">
                    { determinate_linear_progress }
                </span>
                <span class = "demo-item" style = "width: 100%">
                    { indeterminate_linear_progress }
                </span>
            </div>
        </div>
    }
}
