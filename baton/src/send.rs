use tokio::sync::watch;

use crate::Recv;

/// The sending half.
/// Sends new values to the receiving half, potentially dropping them if too fast.
#[derive(Clone)]
pub struct Send<T: Clone> {
    watch: watch::Sender<T>,
}

impl<T: Clone> Send<T> {
    pub(crate) fn new(watch: watch::Sender<T>) -> Self {
        Self { watch }
    }

    /// Set a new value.
    pub fn set(&mut self, value: T) {
        self.watch.send_replace(value);
    }

    /// Return the latest value.
    pub fn get(&self) -> T {
        self.watch.borrow().clone()
    }

    /// Create a new receiving half.
    pub fn recv(&self) -> Recv<T> {
        Recv::new(self.watch.subscribe())
    }
}

impl<T: Clone + PartialEq> Send<T> {
    /// Set a new value if it is different from the previous one.
    pub fn update(&mut self, value: T) {
        self.watch.send_if_modified(|watch| {
            if watch != &value {
                *watch = value;
                true
            } else {
                false
            }
        });
    }
}
