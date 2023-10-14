use std::borrow::Cow;

pub use gloo::events::{EventListener as GlooEventListener, EventListenerOptions};
use wasm_bindgen::JsValue;
pub use web_sys::{CustomEvent, Element, Event, EventTarget};
pub use yew::virtual_dom::{Listener, ListenerKind};
pub use yew::Callback;

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
