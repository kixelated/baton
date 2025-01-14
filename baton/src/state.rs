use std::sync::Arc;

use tokio::sync::Notify;

pub(crate) struct State<T> {
    value: Arc<T>,
    epoch: usize,
    notify: Arc<Notify>,
    closed: bool,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(value),
            epoch: 0,
            notify: Default::default(),
            closed: false,
        }
    }

    pub fn send(&mut self, value: T) -> Result<Arc<T>, T> {
        if self.closed {
            return Err(value);
        }

        self.value = Arc::new(value);
        self.notify.notify_waiters();

        Ok(self.value.clone())
    }

    pub fn recv(&mut self, epoch: usize) -> Result<(usize, Arc<T>), Option<Arc<Notify>>> {
        if self.epoch > epoch {
            let value = self.value.clone();
            Ok((self.epoch, value))
        } else if self.closed {
            return Err(None);
        } else {
            Err(Some(self.notify.clone()))
        }
    }

    pub fn value(&self) -> Arc<T> {
        self.value.clone()
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.notify.notify_waiters();
    }
}
