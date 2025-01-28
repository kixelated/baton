use std::sync::{Arc, Mutex};

use crate::{Recv, State};

/// The sending half.
/// Sends new values to the receiving half, potentially dropping them if too fast.
#[derive(Clone)]
pub struct Send<T: Clone> {
    state: Arc<Mutex<State<T>>>,
    // Close when all send handles are dropped.
    _drop: Arc<SendDrop<T>>,
}

impl<T: Clone> Send<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        Self {
            _drop: Arc::new(SendDrop::new(state.clone())),
            state,
        }
    }

    /// Set a new value.
    pub fn set(&mut self, value: T) {
        self.state.lock().unwrap().set(value);
    }

    /// Return the latest value.
    pub fn get(&self) -> T {
        self.state.lock().unwrap().get()
    }

    /// Create a new receiving half.
    pub fn recv(&self) -> Recv<T> {
        Recv::new(self.state.clone())
    }
}

impl<T: Clone + PartialEq> Send<T> {
    /// Set a new value if it is different from the previous one.
    pub fn set_if(&mut self, value: T) {
        self.state.lock().unwrap().set_if(value);
    }
}

struct SendDrop<T: Clone> {
    state: Arc<Mutex<State<T>>>,
}

impl<T: Clone> SendDrop<T> {
    fn new(state: Arc<Mutex<State<T>>>) -> Self {
        Self { state }
    }
}

impl<T: Clone> Drop for SendDrop<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
