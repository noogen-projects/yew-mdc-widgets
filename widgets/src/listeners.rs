pub use gloo::events::{EventListener, EventListenerOptions};
use wasm_bindgen::JsValue;
pub use web_sys::{CustomEvent, Element, Event, EventTarget};
pub use yew::{virtual_dom::Listener, Callback};

#[derive(Clone, Debug)]
pub struct CustomEventListener {
    event_name: &'static str,
    callback: Callback<CustomEvent>,
}

impl CustomEventListener {
    pub fn new(event_name: &'static str, callback: Callback<CustomEvent>) -> Self {
        Self { event_name, callback }
    }
}

impl Listener for CustomEventListener {
    fn kind(&self) -> &'static str {
        self.event_name
    }

    fn attach(&self, element: &Element) -> EventListener {
        let callback = self.callback.clone();
        let listener = move |event: &Event| {
            let event: CustomEvent = JsValue::from(event).into();
            callback.emit(event);
        };
        let options = EventListenerOptions::enable_prevent_default();
        EventListener::new_with_options(&EventTarget::from(element.clone()), self.event_name, options, listener)
    }
}
