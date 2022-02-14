use std::ops::{Deref, DerefMut};

use yew::{classes, html, Html};

use crate::{
    utils::{ManageChildren, VTagExt},
    MdcWidget, AUTO_INIT_ATTR,
};

pub mod mdc {
    use crate::Element;
    use wasm_bindgen::prelude::*;

    pub const TYPE_NAME: &str = "MDCLinearProgress";

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = MDCLinearProgress)]
        pub type LinearProgress;

        #[wasm_bindgen(constructor, js_class = MDCLinearProgress, js_namespace = ["mdc", "linear_progress"])]
        pub fn new(element: Element) -> LinearProgress;

        /// Toggles the component between the determinate and indeterminate state
        #[wasm_bindgen(method, setter)]
        pub fn set_determinate(this: &LinearProgress, is_determinate: bool);

        /// Sets the progress bar to this value. Value should be between [0, 1]
        #[wasm_bindgen(method, setter)]
        pub fn set_progress(this: &LinearProgress, value: f64);

        /// Sets the buffer bar to this value. Value should be between [0, 1]
        #[wasm_bindgen(method, setter)]
        pub fn set_buffer(this: &LinearProgress, value: f64);

        /// Reverses the direction of the linear progress indicator
        #[wasm_bindgen(method, setter)]
        pub fn set_reverse(this: &LinearProgress, is_reverse: bool);

        /// Puts the component in the open state
        #[wasm_bindgen(method)]
        pub fn open(this: &LinearProgress);

        /// Puts the component in the closed state
        #[wasm_bindgen(method)]
        pub fn close(this: &LinearProgress);
    }
}

#[derive(Debug, Clone)]
pub struct LinearProgress {
    html: Html,
}

impl Default for LinearProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearProgress {
    pub const VAR_NAME: &'static str = "linear-progress";

    pub const CLASS: &'static str = "mdc-linear-progress";

    /// Puts the linear progress indicator in an indeterminate state
    pub const INDETERMINATE_CLASS: &'static str = "mdc-linear-progress--indeterminate";

    ///
    pub const BUFFER_CLASS: &'static str = "mdc-linear-progress__buffer";

    ///
    pub const BUFFER_BAR_CLASS: &'static str = "mdc-linear-progress__buffer-bar";

    ///
    pub const BUFFER_DOTS_CLASS: &'static str = "mdc-linear-progress__buffer-dots";

    ///
    pub const BAR_CLASS: &'static str = "mdc-linear-progress__bar";

    ///
    pub const PRIMARY_BAR_CLASS: &'static str = "mdc-linear-progress__primary-bar";

    ///
    pub const SECONDARY_BAR_CLASS: &'static str = "mdc-linear-progress__secondary-bar";

    ///
    pub const INNER_BAR_CLASS: &'static str = "mdc-linear-progress__bar-inner";

    /// Hides the linear progress indicator.
    pub const CLOSED_CLASS: &'static str = "mdc-linear-progress--closed";

    pub fn new() -> Self {
        let mut linear_progress = Self {
            html: html! {
                <div role = "progressbar" class = { Self::CLASS }
                        aria-valuemin = "0" aria-valuemax = "1" aria-valuenow = "0">
                    <div class = { Self::BUFFER_CLASS }>
                        <div class = { Self::BUFFER_BAR_CLASS }></div>
                        <div class = { Self::BUFFER_DOTS_CLASS }></div>
                    </div>
                    <div class = { classes!(Self::BAR_CLASS, Self::PRIMARY_BAR_CLASS) }>
                        <span class = { Self::INNER_BAR_CLASS }></span>
                    </div>
                    <div class = { classes!(Self::BAR_CLASS, Self::SECONDARY_BAR_CLASS) }>
                        <span class = { Self::INNER_BAR_CLASS }></span>
                    </div>
                </div>
            },
        };
        linear_progress.root_tag_mut().set_attr(AUTO_INIT_ATTR, mdc::TYPE_NAME);
        linear_progress
    }

    pub fn indeterminate(mut self) -> Self {
        self.root_tag_mut().add_class(Self::INDETERMINATE_CLASS);
        self
    }

    pub fn closed(mut self) -> Self {
        self.root_tag_mut().add_class(Self::CLOSED_CLASS);
        self
    }

    pub fn progress(mut self, value: f32) -> Self {
        let root = self.root_tag_mut();
        root.set_attr("aria-valuenow", value.to_string());
        if let Some(primary_bar) = root.find_child_contains_class_mut(Self::PRIMARY_BAR_CLASS) {
            primary_bar.set_attr("style", format!("transform: scaleX({});", value));
        }
        self
    }

    pub fn buffer(mut self, value: f32) -> Self {
        if let Some(primary_bar) = self
            .root_tag_mut()
            .find_child_contains_class_recursively_mut(Self::BUFFER_BAR_CLASS)
        {
            primary_bar.set_attr("style", format!("flex-basis: {}%;", value * 100.0));
        }
        self
    }
}

impl MdcWidget for LinearProgress {
    const NAME: &'static str = stringify!(LinearProgress);

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }
}

impl Deref for LinearProgress {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for LinearProgress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<LinearProgress> for Html {
    fn from(widget: LinearProgress) -> Self {
        widget.html
    }
}
