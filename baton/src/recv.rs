use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::{Closed, State};

/// The receiving half.
/// Returns or waits for each update, potentially skipping them if too slow.
#[derive(Clone)]
pub struct Recv<T> {
    state: Arc<Mutex<State<T>>>,
    latest: Arc<T>,
    epoch: usize,

    // Close when all (cloned) receivers are dropped.
    _closed: Arc<Closed<T>>,
}

impl<T> Recv<T> {
    pub(crate) fn new(state: Arc<Mutex<State<T>>>) -> Self {
        let latest = state.lock().unwrap().value();
        let _closed = Closed::new(state.clone());

        Self {
            latest,
            state,
            epoch: 0,
            _closed,
        }
    }

    /// Wait for an unseen value, not including the initial value.
    /// Returns [None] when the [Send] half is dropped with no new value.
    ///
    /// This only consumes the latest value, so some values may be skipped.
    /// If you want every value, use one of the many channel implementations.
    pub async fn recv(&mut self) -> Option<&T> {
        loop {
            let notify = {
                let mut state = self.state.lock().unwrap();
                match state.recv(self.epoch) {
                    Ok((epoch, value)) => {
                        self.epoch = epoch;
                        self.latest = value;
                        return Some(self.latest.as_ref());
                    }
                    Err(Some(notify)) => notify,
                    Err(None) => return None,
                }
            };

            notify.notified().await;
        }
    }
}

impl<T> Deref for Recv<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.latest
    }
}
