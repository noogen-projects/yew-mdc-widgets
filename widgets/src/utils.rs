pub use self::ext::*;

use std::rc::Rc;

use wasm_dom::UnwrapThrowExt;
use web_sys::Node;
use yew::{html, html::onclick, Callback, Classes, Html, MouseEvent};

use crate::MdcWidget;

pub mod ext;

pub fn raw_html(tag: impl AsRef<str>, inner_html: impl AsRef<str>) -> Html {
    let html = wasm_dom::existing::document()
        .create_element(tag.as_ref())
        .expect_throw("Tag should be created");
    html.set_inner_html(inner_html.as_ref());

    Html::VRef(Node::from(html))
}

pub(crate) trait IntoWidgetWithVList: MdcWidget {
    fn into_widget_with_v_list(self) -> Self;
}

pub(crate) fn add_input_label<W: IntoWidgetWithVList>(mut widget: W, label: impl Into<Html>) -> Result<W, W> {
    if let Some(input_id) = widget
        .root_tag()
        .find_child_tag_recursively("input")
        .and_then(|input| input.attr("id"))
    {
        let label = html! {
            <label for = { input_id }>{ label }</label>
        };
        widget = widget.into_widget_with_v_list();
        if let Html::VList(list) = widget.html_mut() {
            list.insert_child(1, label);
        }
        Ok(widget)
    } else {
        Err(widget)
    }
}

pub(crate) fn ripple_element(widget: &mut impl MdcWidget, ripple_class: impl Into<Classes>, enabled: bool) {
    let ripple_class = ripple_class.into();
    let root = widget.root_tag_mut();
    if enabled {
        if !root.is_some_child_contains_class(&ripple_class.to_string()) {
            let idx = root.children().len().saturating_sub(1);
            root.insert_child(idx, html! {
                <div class = { ripple_class }></div>
            });
        }
    } else {
        root.remove_child_contains_class(&ripple_class.to_string());
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
            input.set_attr("disabled", "disabled");
        } else {
            input.remove_attr_or_prop("disabled");
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
