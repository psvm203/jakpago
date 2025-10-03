use std::{cell::RefCell, rc::Rc};
use sycamore::{
    prelude::*,
    web::events::{EventDescriptor, EventHandler},
};
use web_sys::{Event, HtmlInputElement, wasm_bindgen::JsCast};

pub trait ViewVecExt {
    fn join<F>(self, separator_fn: F) -> Vec<View>
    where
        F: Fn() -> View;
}

impl ViewVecExt for Vec<View> {
    fn join<F>(self, separator_fn: F) -> Vec<View>
    where
        F: Fn() -> View,
    {
        if self.is_empty() {
            return Self::new();
        }

        let mut result = Self::with_capacity(self.len() * 2 - 1);

        for (i, view) in self.into_iter().enumerate() {
            if i > 0 {
                result.push(separator_fn());
            }
            result.push(view);
        }

        result
    }
}

pub struct Callback {
    cb: Rc<RefCell<dyn FnMut(Event)>>,
}

impl<F> From<F> for Callback
where
    F: FnMut(Event) + 'static,
{
    fn from(function: F) -> Self {
        Self {
            cb: Rc::new(RefCell::new(function)),
        }
    }
}

impl Clone for Callback {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
        }
    }
}

impl<E> EventHandler<E> for Callback
where
    E: EventDescriptor,
{
    fn call(&mut self, event: E::EventTy) {
        let generic_event: Event = event.into();
        if let Ok(mut callback) = self.cb.try_borrow_mut() {
            callback(generic_event);
        }
    }
}

pub trait EventParser {
    fn parse(&self) -> Option<u32>;
}

impl EventParser for Event {
    fn parse(&self) -> Option<u32> {
        let target = self.target()?;
        let input = target.dyn_into::<HtmlInputElement>().ok()?;
        input.value().parse().ok()
    }
}
