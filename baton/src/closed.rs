use std::sync::{Arc, Mutex};

use crate::State;

pub struct Closed<T> {
    state: Arc<Mutex<State<T>>>,
}

impl<T> Closed<T> {
    pub fn new(state: Arc<Mutex<State<T>>>) -> Arc<Self> {
        Arc::new(Self { state })
    }
}

impl<T> Drop for Closed<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
