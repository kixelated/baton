use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::State;

/// The sending half.
/// Sends new values to the receiving half, potentially dropping them if too fast.
#[derive(Clone)]
pub struct Send<T> {
    state: Arc<Mutex<State<T>>>,
    latest: Arc<T>,
}

impl<T> Send<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        let latest = state.lock().unwrap().value();
        Self { latest, state }
    }

    /// Set a new value.
    ///
    /// Returns [Err] if the [crate::Recv] half is dropped.
    pub fn send(&mut self, value: T) -> Result<(), T> {
        self.latest = self.state.lock().unwrap().send(value)?;
        Ok(())
    }
}

impl<T> Deref for Send<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.latest
    }
}

impl<T> Drop for Send<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
