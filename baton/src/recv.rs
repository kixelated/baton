use std::sync::{Arc, Mutex};

use crate::State;

/// The receiving half.
/// Returns or waits for each update, potentially skipping them if too slow.
pub struct Recv<T> {
    state: Arc<Mutex<State<T>>>,
    init: Option<T>,
}

impl<T> Recv<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        let init = state.lock().unwrap().take();
        Self { init, state }
    }

    /// Return or wait for a new value.
    /// Returns [None] when the [Send] half is dropped with no new value.
    ///
    /// This only consumes the latest value, so some values may be skipped.
    /// If you want every value, use one of the many channel implementations.
    pub async fn recv(&mut self) -> Option<T> {
        if let Some(init) = self.init.take() {
            return Some(init);
        }

        loop {
            let notify = {
                let mut state = self.state.lock().unwrap();
                match state.recv() {
                    Ok(value) => return Some(value),
                    Err(Some(notify)) => notify,
                    Err(None) => return None,
                }
            };

            notify.notified().await;
        }
    }
}

impl<T> Drop for Recv<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().close();
    }
}
