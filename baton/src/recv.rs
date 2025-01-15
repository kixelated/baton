use std::sync::{Arc, Mutex};

use crate::State;

/// The receiving half.
/// Returns or waits for each update, potentially skipping them if too slow.
#[derive(Clone)]
pub struct Recv<T: Clone> {
    state: Arc<Mutex<State<T>>>,
    epoch: usize,
}

impl<T: Clone> Recv<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        Self { state, epoch: 0 }
    }

    /// Wait for an unseen value, not including the initial value.
    /// Returns [None] when the [Send] half is dropped with no new value.
    ///
    /// This only consumes the latest value, so some values may be skipped.
    /// If you want every value, use one of the many channel implementations.
    pub async fn next(&mut self) -> Option<T> {
        loop {
            let notify = {
                let mut state = self.state.lock().unwrap();
                match state.next(self.epoch) {
                    Ok((epoch, value)) => {
                        self.epoch = epoch;
                        return Some(value);
                    }
                    Err(Some(notify)) => notify,
                    Err(None) => return None,
                }
            };

            notify.notified().await;
        }
    }

    /// Return the latest value without waiting or marking it as seen.
    pub fn get(&self) -> T {
        self.state.lock().unwrap().get()
    }
}
