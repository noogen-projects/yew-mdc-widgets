use std::ops::{Deref, DerefMut};

use yew::{html, Html};

use crate::{utils::VTagExt, MdcWidget};

#[derive(Debug, Clone)]
pub struct CardContent {
    html: Html,
}

impl CardContent {
    pub const ACTIONS_CLASS: &'static str = "mdc-card__actions";
    pub const ACTIONS_FULL_BLEED_CLASS: &'static str = "mdc-card__actions--full-bleed";
    pub const ACTION_BUTTONS_CLASS: &'static str = "mdc-card__action-buttons";
    pub const ACTION_BUTTON_CLASSES: &'static str = "mdc-card__action mdc-card__action--button";
    pub const ACTION_CLASS: &'static str = "mdc-card__action";
    pub const ACTION_ICONS_CLASS: &'static str = "mdc-card__action-icons";
    pub const ACTION_ICON_CLASSES: &'static str = "mdc-card__action mdc-card__action--icon";
    pub const MEDIA_16_9_CLASS: &'static str = "mdc-card__media--16-9";
    pub const MEDIA_CLASS: &'static str = "mdc-card__media";
    pub const MEDIA_CONTENT_CLASS: &'static str = "mdc-card__media-content";
    pub const MEDIA_SQUARE_CLASS: &'static str = "mdc-card__media--square";
    pub const PRIMARY_ACTION_CLASS: &'static str = "mdc-card__primary-action";

    /// The main tappable area of the card. Typically contains most (or all) card content
    /// *except* `mdc-card__actions`. Only applicable to cards that have a primary action
    /// that the main surface should trigger.
    pub fn primary_action(content: impl Into<Html>) -> Self {
        Self {
            html: html! {
                <div class = Self::PRIMARY_ACTION_CLASS tabindex = "0">{ content }</div>
            },
        }
    }

    /// Media area that displays a custom `background-image` with `background-size: cover`.
    pub fn media() -> Self {
        Self {
            html: html! {
                <div class = Self::MEDIA_CLASS></div>
            },
        }
    }

    /// Automatically scales the media area's height to equal its width.
    pub fn media_square() -> Self {
        Self::media().class(Self::MEDIA_SQUARE_CLASS)
    }

    /// Automatically scales the media area's height according to its width, maintaining
    /// a 16:9 aspect ratio.
    pub fn media_16_9() -> Self {
        Self::media().class(Self::MEDIA_16_9_CLASS)
    }

    /// An absolutely-positioned box the same size as the media area, for displaying a title
    /// or icon on top of the `background-image`.
    pub fn media_content(mut self, content: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        if root.is_contains_class(Self::MEDIA_CLASS) {
            root.add_child(html! { <div class = Self::MEDIA_CONTENT_CLASS>{ content }</div> });
        }
        self
    }

    /// Row containing action buttons and/or icons.
    pub fn actions() -> Self {
        Self {
            html: html! {
                <div class = Self::ACTIONS_CLASS></div>
            },
        }
    }

    /// Removes the action area's padding and causes its only child (an `mdc-card__action` element)
    /// to consume 100% of the action area's width.
    pub fn actions_full_bleed() -> Self {
        Self::actions().class(Self::ACTIONS_FULL_BLEED_CLASS)
    }

    pub fn actions_content(mut self, content: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        if root.is_contains_class(Self::ACTIONS_CLASS) {
            root.add_child(content.into());
        }
        self
    }

    /// A group of action buttons, displayed on the left side of the card (in LTR),
    /// adjacent to `mdc-card__action-icons`.
    pub fn action_buttons(self, content: impl Into<Html>) -> Self {
        self.actions_content(html! { <div class = Self::ACTION_BUTTONS_CLASS>{ content }</div>})
    }

    /// A group of supplemental action icons, displayed on the right side of the card (in LTR),
    /// adjacent to `mdc-card__action-buttons`.
    pub fn action_icons(self, content: impl Into<Html>) -> Self {
        self.actions_content(html! { <div class = Self::ACTION_ICONS_CLASS>{ content }</div>})
    }
}

impl MdcWidget for CardContent {
    const NAME: &'static str = "CardContent";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for CardContent {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for CardContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<CardContent> for Html {
    fn from(widget: CardContent) -> Self {
        widget.html
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    html: Html,
}

impl Card {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        let card = Self {
            html: html! {
                <div id = id class = "mdc-card">
                </div>
            },
        };
        card.ripple(true)
    }

    pub fn ripple(mut self, enabled: bool) -> Self {
        let root = self.root_tag_mut();
        if enabled {
            if !root.is_last_child("script") {
                if let Some(id) = root.attributes.get("id") {
                    root.children.push(html! {
                        <script>{
                            format!(r"{{const ripples = [].map.call(
                                document.querySelectorAll('#{} .{}'),
                                function(el) {{ return new mdc.ripple.MDCRipple(el); }}
                            );}}", id, CardContent::PRIMARY_ACTION_CLASS)
                        }</script>
                    });
                }
            }
        } else {
            root.remove_child_tag("script");
        }
        self
    }

    /// Removes the shadow and displays a hairline outline instead.
    pub fn outlined(self) -> Self {
        self.class("mdc-card--outlined")
    }

    pub fn content(mut self, content: impl Into<Html>) -> Self {
        let root = self.root_tag_mut();
        let idx = root.find_child_tag_idx("script").unwrap_or_else(|| root.children.len());
        root.children.insert(idx, content.into());
        self
    }
}

impl MdcWidget for Card {
    const NAME: &'static str = stringify!(Card);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for Card {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Card {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Card> for Html {
    fn from(widget: Card) -> Self {
        widget.html
    }
}
