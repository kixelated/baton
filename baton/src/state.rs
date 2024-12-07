use std::sync::Arc;

use tokio::sync::Notify;

pub(crate) struct State<T> {
    latest: Option<T>,
    notify: Arc<Notify>,
    closed: bool,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self {
            latest: Some(value),
            notify: Default::default(),
            closed: false,
        }
    }

    pub fn recv(&mut self) -> Result<T, Option<Arc<Notify>>> {
        if let Some(latest) = self.latest.take() {
            Ok(latest)
        } else if self.closed {
            Err(None)
        } else {
            Err(Some(self.notify.clone()))
        }
    }

    pub fn send(&mut self, value: T) -> Result<(), T> {
        if self.closed {
            return Err(value);
        }

        self.latest = Some(value);
        self.notify.notify_one();

        Ok(())
    }

    pub fn take(&mut self) -> Option<T> {
        self.latest.take()
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.notify.notify_one();
    }
}
