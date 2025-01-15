use std::sync::{Arc, Mutex};

use crate::{Recv, State};

/// The sending half.
/// Sends new values to the receiving half, potentially dropping them if too fast.
pub struct Send<T: Clone> {
    state: Arc<Mutex<State<T>>>,
}

impl<T: Clone> Send<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        Self { state }
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

impl<T: Clone> Drop for Send<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
