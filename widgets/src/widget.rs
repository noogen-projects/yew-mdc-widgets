use std::rc::Rc;

use wasm_bindgen::JsValue;
use yew::{
    virtual_dom::{Listener, VTag},
    Callback, Html,
};

use crate::{utils::VTagExt, EventListener};

pub trait MdcWidget {
    const NAME: &'static str;

    fn html(&self) -> &Html;

    fn html_mut(&mut self) -> &mut Html;

    #[track_caller]
    fn root_tag(&self) -> &VTag {
        self.html()
            .root_tag()
            .unwrap_or_else(|| panic!("The root element of the {} must be a tag!", Self::NAME))
    }

    #[track_caller]
    fn root_tag_mut(&mut self) -> &mut VTag {
        self.html_mut()
            .root_tag_mut()
            .unwrap_or_else(|| panic!("The root element of the {} must be a tag!", Self::NAME))
    }

    fn id(mut self, id: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().set_attr("id", id.into());
        self
    }

    fn listener(mut self, listener: Rc<dyn Listener>) -> Self
    where
        Self: Sized,
    {
        let root = self.root_tag_mut();
        if !root.add_listener(listener.clone()) {
            root.set_listeners([Some(listener)].into());
        }
        self
    }

    fn on_event<E>(self, event_type: &'static str, callback: impl Into<Callback<E>>) -> Self
    where
        E: From<JsValue> + Clone + 'static,
        Self: Sized,
    {
        self.listener(Rc::new(EventListener::new(event_type, callback.into())))
    }

    fn attr(mut self, attr: &'static str, value: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().set_attr(attr, value.into());
        self
    }

    fn class(mut self, class: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().add_class_if_needed(class);
        self
    }

    fn child(mut self, child: impl Into<Html>) -> Self
    where
        Self: Sized,
    {
        self.root_tag_mut().add_child(child.into());
        self
    }
}
