use std::sync::{Arc, Mutex};

use crate::State;

/// The receiving half.
/// Returns or waits for each update, potentially skipping them if too slow.
pub struct Recv<T> {
    state: Arc<Mutex<State<T>>>,
    latest: T,
    seen: bool,
}

impl<T> Recv<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        let latest = state.lock().unwrap().take().unwrap();
        Self {
            latest,
            state,
            seen: false,
        }
    }

    /// Return or wait for a new value.
    /// Returns [None] when the [Send] half is dropped with no new value.
    ///
    /// This only consumes the latest value, so some values may be skipped.
    /// If you want every value, use one of the many channel implementations.
    pub async fn recv(&mut self) -> Option<&T> {
        if !self.seen {
            self.seen = true;
            return Some(&self.latest);
        }

        loop {
            let notify = {
                let mut state = self.state.lock().unwrap();
                match state.recv() {
                    Ok(value) => {
                        self.latest = value;
                        self.seen = true;
                        return Some(&self.latest);
                    }
                    Err(Some(notify)) => notify,
                    Err(None) => return None,
                }
            };

            notify.notified().await;
        }
    }

    /// Return the latest value returned by [Self::recv].
    ///
    /// NOTE: If [Self::recv] has not been called yet, then this will return the initial value.
    /// I think that's better behavior than returning None, right?
    pub fn latest(&self) -> &T {
        &self.latest
    }
}

impl<T> Drop for Recv<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
