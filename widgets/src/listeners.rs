use std::marker::PhantomData;

pub use gloo::events::{EventListener as GlooEventListener, EventListenerOptions};
use wasm_bindgen::JsValue;
pub use web_sys::{CustomEvent, Element, Event, EventTarget};
pub use yew::{virtual_dom::Listener, Callback};

pub trait EventCallback<E>: Clone + 'static {
    fn call(&self, element: Element, event: E);
}

impl<E, F> EventCallback<E> for F
where
    F: Fn(Element, E) + Clone + 'static,
{
    fn call(&self, element: Element, event: E) {
        self(element, event)
    }
}

impl<E: 'static> EventCallback<E> for Callback<E> {
    fn call(&self, _element: Element, event: E) {
        self.emit(event)
    }
}

impl<E: 'static> EventCallback<E> for Callback<(Element, E)> {
    fn call(&self, element: Element, event: E) {
        self.emit((element, event))
    }
}

#[derive(Clone, Debug)]
pub struct EventListener<E, C> {
    event_name: &'static str,
    callback: C,
    _phantom: PhantomData<E>,
}

impl<E, C> EventListener<E, C> {
    pub fn new(event_name: &'static str, callback: C) -> Self {
        Self {
            event_name,
            callback,
            _phantom: Default::default(),
        }
    }
}

impl<E, C> Listener for EventListener<E, C>
where
    E: From<JsValue> + Clone,
    C: EventCallback<E>,
{
    fn kind(&self) -> &'static str {
        self.event_name
    }

    fn attach(&self, element: &Element) -> GlooEventListener {
        let this = element.clone();
        let callback = self.callback.clone();

        let listener = move |event: &Event| {
            let event: E = JsValue::from(event).into();
            callback.call(this.clone(), event);
        };

        let options = EventListenerOptions::enable_prevent_default();
        GlooEventListener::new_with_options(&EventTarget::from(element.clone()), self.event_name, options, listener)
    }
}
