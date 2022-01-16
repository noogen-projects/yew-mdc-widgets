pub use gloo::events::{EventListener as GlooEventListener, EventListenerOptions};
pub use web_sys::{CustomEvent, Element, Event, EventTarget};
pub use yew::{
    virtual_dom::{Listener, ListenerKind},
    Callback,
};

use std::borrow::Cow;

use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct EventListener<E> {
    event_type: &'static str,
    callback: Callback<E>,
}

impl<E> EventListener<E> {
    pub fn new(event_type: &'static str, callback: Callback<E>) -> Self {
        Self { event_type, callback }
    }
}

impl<E> Listener for EventListener<E>
where
    E: From<JsValue> + Clone,
{
    fn kind(&self) -> ListenerKind {
        ListenerKind::other(Cow::Borrowed(self.event_type))
    }

    fn handle(&self, event: Event) {
        let event: E = JsValue::from(event).into();
        self.callback.emit(event);
    }

    fn passive(&self) -> bool {
        false
    }
}
