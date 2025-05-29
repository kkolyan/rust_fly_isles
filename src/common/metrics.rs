use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use macroquad::prelude::Vec2;

#[derive(Default, Debug)]
pub struct Metrics {
    pub metrics: RefCell<HashMap<&'static str, String>>,
    pub metrics_order: RefCell<Vec<&'static str>>,
    pub enabled: Cell<bool>,
}

impl Metrics {
    pub fn clear(&self) {
        self.metrics.borrow_mut().clear();
        self.metrics_order.borrow_mut().clear();
    }

    pub fn set_enabled(&self, value: bool) {
        self.enabled.set(value);
    }

    pub fn record<T: Display>(&self, name: &'static str, value: T) {
        if self.enabled.get() && self.metrics.borrow_mut().insert(name, value.to_string()).is_none() {
            self.metrics_order.borrow_mut().push(name);
        }
    }
}