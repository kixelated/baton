use std::sync::Arc;

use tokio::sync::Notify;

pub(crate) struct State<T: Clone> {
    value: T,
    epoch: usize,
    notify: Arc<Notify>,
    closed: bool,
}

impl<T: Clone> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            epoch: 0,
            notify: Default::default(),
            closed: false,
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.epoch += 1;
        self.notify.notify_waiters();
    }

    pub fn next(&mut self, epoch: usize) -> Result<(usize, T), Option<Arc<Notify>>> {
        if self.epoch > epoch {
            let value = self.value.clone();
            Ok((self.epoch, value))
        } else if self.closed {
            return Err(None);
        } else {
            Err(Some(self.notify.clone()))
        }
    }

    pub fn get(&self) -> T {
        self.value.clone()
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.notify.notify_waiters();
    }
}
