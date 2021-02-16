use std::ops::{Deref, DerefMut};

use yew::{html, services::ConsoleService, Callback, Html, MouseEvent};

use crate::{
    utils::{add_input_label, labeled_on_click, root_and_input_child_disabled, IntoWidgetWithVList, VTagExt},
    MdcWidget, AUTO_INIT_ATTR,
};

#[derive(Debug, Clone)]
pub struct Switch {
    html: Html,
}

impl Switch {
    pub fn simple() -> Self {
        Self {
            html: html! {
                <div class = "mdc-switch">
                    <div class = "mdc-switch__track"></div>
                    <div class = "mdc-switch__thumb-underlay">
                        <div class = "mdc-switch__thumb"></div>
                        <input type = "checkbox" class = "mdc-switch__native-control" role = "switch" aria-checked = "false" />
                    </div>
                </div>
            },
        }
    }

    pub fn new() -> Self {
        let mut switch = Self::simple();
        switch.root_tag_mut().set_attr(AUTO_INIT_ATTR, "MDCSwitch");
        switch
    }

    pub fn label(self, label: impl Into<Html>) -> Self {
        add_input_label(self, label).unwrap_or_else(|widget| {
            ConsoleService::error(&format!("Could not find input tag's id attribute for {}", Self::NAME));
            widget
        })
    }

    pub fn disabled(self) -> Self {
        self.disable(true)
    }

    pub fn disable(mut self, disable: bool) -> Self {
        root_and_input_child_disabled(&mut self, "mdc-switch--disabled", disable);
        self
    }

    pub fn on(self) -> Self {
        self.turn(true)
    }

    pub fn turn(mut self, on: bool) -> Self {
        let root = self.root_tag_mut();

        if on {
            root.add_class_if_needed("mdc-switch--checked");
        } else {
            root.remove_class("mdc-switch--checked")
        }

        if let Some(input) = root.find_child_tag_recursively_mut("input") {
            input.checked = on;
            input.set_attr("aria-checked", if on { "true" } else { "false" });
        }
        self
    }

    pub fn on_click(mut self, callback: impl Into<Callback<MouseEvent>>) -> Self {
        labeled_on_click(&mut self, callback.into());
        self
    }
}

impl MdcWidget for Switch {
    const NAME: &'static str = "Switch";

    fn html(&self) -> &Html {
        &self.html
    }

    fn html_mut(&mut self) -> &mut Html {
        &mut self.html
    }

    fn id(mut self, id: impl Into<String>) -> Self {
        let id = id.into();
        let input_id = format!("{}-input", id);

        let root = self.root_tag_mut();
        root.set_attr("id", id);
        if let Some(input) = root.find_child_tag_recursively_mut("input") {
            input.set_attr("id", input_id);
        };
        self
    }
}

impl IntoWidgetWithVList for Switch {
    fn into_widget_with_v_list(mut self) -> Self {
        if !matches!(self.html, Html::VList(_)) {
            self.html = html! { <>{ self.html }</> }
        }
        self
    }
}

impl Deref for Switch {
    type Target = Html;

    fn deref(&self) -> &Self::Target {
        &self.html
    }
}

impl DerefMut for Switch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.html
    }
}

impl From<Switch> for Html {
    fn from(widget: Switch) -> Self {
        widget.html
    }
}
