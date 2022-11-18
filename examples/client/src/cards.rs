use yew::{html, Html};
use yew_mdc_widgets::{Button, Card, CardContent, IconButton, MdcWidget};

pub fn view() -> Html {
    html! {
        <div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Regular" }</h3>
                <span class = "demo-item">
                    {
                        Card::new("card-regular")
                            .class("demo-card regular")
                            .content(CardContent::primary_action(html! {
                                <>
                                    {
                                        CardContent::media_16_9()
                                            .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                    }
                                    <div class = "demo-card__primary">
                                        <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                        <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                    </div>
                                    <div class = "demo-card__secondary mdc-typography mdc-typography--body2">
                                        { "Visit ten places on our planet that are undergoing the biggest changes today." }
                                    </div>
                                </>
                            }))
                            .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                ]))
                                .action_icons(Html::from_iter(vec![
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                ]))
                            )
                    }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Text over media" }</h3>
                <span class = "demo-item">
                    {
                        Card::new("card-over-media")
                            .class("demo-card over-media")
                            .content(CardContent::primary_action(html! {
                                <>
                                    {
                                        CardContent::media_16_9()
                                            .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                            .media_content(html! {
                                                <div class = "demo-card__primary">
                                                    <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                                    <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                                </div>
                                            })
                                    }
                                    <div class = "demo-card__secondary mdc-typography mdc-typography--body2">
                                        { "Visit ten places on our planet that are undergoing the biggest changes today." }
                                    </div>
                                </>
                            }))
                            .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                ]))
                                .action_icons(Html::from_iter(vec![
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                ]))
                            )
                    }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "With header" }</h3>
                <span class = "demo-item">
                    {
                        Card::new("card-header")
                            .class("demo-card")
                            .content(html! {
                                <div class = "demo-card__primary">
                                    <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                    <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                </div>
                            })
                            .content(CardContent::primary_action(html! {
                                <>
                                    {
                                        CardContent::media_16_9()
                                            .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                    }
                                    <div class = "demo-card__secondary mdc-typography mdc-typography--body2">
                                        { "Visit ten places on our planet that are undergoing the biggest changes today." }
                                    </div>
                                </>
                            }))
                            .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                ]))
                                .action_icons(Html::from_iter(vec![
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                ]))
                            )
                    }
                </span>
            </div>
            <div>
                <h3 class = "mdc-typography--subtitle1">{ "Horizontal" }</h3>
                <span class = "demo-item">
                    {
                        Card::new("card-horizontal")
                            .class("demo-card")
                            .content(CardContent::primary_action(html! {
                                <>
                                    {
                                        CardContent::media_square()
                                            .attr("style", r#"background-image: url("https://material-components.github.io/material-components-web-catalog/static/media/photos/3x2/2.jpg");"#)
                                    }
                                    <div class = "demo-card__primary">
                                        <h2 class = "demo-card__title mdc-typography mdc-typography--headline6">{ "Our Changing Planet" }</h2>
                                        <h3 class = "demo-card__subtitle mdc-typography mdc-typography--subtitle2">{ "by Kurt Wagner" }</h3>
                                    </div>
                                </>
                            }).class("primary-action-horizontal"))
                            .content(CardContent::actions().action_buttons(Html::from_iter(vec![
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Read"),
                                    Button::new().class(CardContent::ACTION_BUTTON_CLASSES).label("Bookmark"),
                                ]))
                                .action_icons(Html::from_iter(vec![
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).toggle("favorite", "favorite_border"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("share"),
                                    IconButton::new().class(CardContent::ACTION_ICON_CLASSES).icon("more_vert"),
                                ]))
                            )
                    }
                </span>
            </div>
        </div>
    }
}
