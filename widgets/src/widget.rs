use std::rc::Rc;

use wasm_bindgen::JsValue;
use yew::{
    virtual_dom::{Listener, VTag},
    Html,
};

use crate::{utils::VTagExt, EventCallback, EventListener};

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
        self.root_tag_mut().add_listener(listener);
        self
    }

    fn on_event<E, C>(mut self, event_name: &'static str, callback: impl Into<C>) -> Self
    where
        E: From<JsValue> + Clone + 'static,
        C: EventCallback<E>,
        Self: Sized,
    {
        let listener = Rc::new(EventListener::new(event_name, callback.into()));
        self.root_tag_mut().add_listener(listener);
        self
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
