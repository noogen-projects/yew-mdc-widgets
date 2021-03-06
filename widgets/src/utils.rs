use std::rc::Rc;

use yew::{html, html::onclick, Callback, Html, MouseEvent};

pub use self::ext::*;
use crate::MdcWidget;

pub mod dom;
pub mod ext;

pub(crate) trait IntoWidgetWithVList: MdcWidget {
    fn into_widget_with_v_list(self) -> Self;
}

pub(crate) fn add_input_label<W: IntoWidgetWithVList>(mut widget: W, label: impl Into<Html>) -> Result<W, W> {
    if let Some(input_id) = widget
        .root_tag()
        .find_child_tag_recursively("input")
        .and_then(|input| input.attributes.get("id"))
    {
        let label = html! {
            <label for = input_id>{ label }</label>
        };
        widget = widget.into_widget_with_v_list();
        if let Html::VList(list) = widget.html_mut() {
            list.children.insert(1, label);
        }
        Ok(widget)
    } else {
        Err(widget)
    }
}

pub(crate) fn ripple_element(widget: &mut impl MdcWidget, ripple_class: impl AsRef<str>, enabled: bool) {
    let ripple_class = ripple_class.as_ref();
    let root = widget.root_tag_mut();
    if enabled {
        if !root.is_some_child_contains_class(ripple_class) {
            let idx = root.children.len().saturating_sub(1);
            root.children.insert(idx, html! {
                <div class = ripple_class></div>
            });
        }
    } else {
        root.remove_child_contains_class(ripple_class);
    }
}

pub(crate) fn root_and_input_child_disabled(
    widget: &mut impl MdcWidget,
    disabled_class: impl AsRef<str>,
    disabled: bool,
) {
    let disabled_class = disabled_class.as_ref();
    if disabled {
        widget.root_tag_mut().add_class(disabled_class);
    } else {
        widget.root_tag_mut().remove_class(disabled_class);
    }

    if let Some(input) = widget.root_tag_mut().find_child_tag_recursively_mut("input") {
        if disabled {
            input.attributes.insert("disabled".into(), "disabled".into());
        } else {
            input.attributes.remove("disabled");
        }
    }
}

pub(crate) fn labeled_on_click<W: MdcWidget>(widget: &mut W, callback: impl Into<Callback<MouseEvent>>) {
    let listener = Rc::new(onclick::Wrapper::new(callback.into()));
    let root = widget.root_tag_mut();
    if let Some(label) = root.find_child_tag_mut("label") {
        label.add_listener(listener.clone());
    }
    root.add_listener(listener);
}
