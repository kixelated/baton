use std::sync::{Arc, Mutex};

use crate::State;

/// The sending half.
/// Sends new values to the receiving half, potentially dropping them if too fast.
pub struct Send<T> {
    state: Arc<Mutex<State<T>>>,
}

impl<T> Send<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        Self { state }
    }

    /// Send a new value.
    ///
    /// Returns [Err] if the [crate::Recv] half is dropped.
    pub fn send(&mut self, value: T) -> Result<(), T> {
        let mut state = self.state.lock().unwrap();
        state.send(value)
    }
}

impl<T> Drop for Send<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
