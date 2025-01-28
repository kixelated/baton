use tokio::sync::watch;

/// The receiving half.
/// Returns or waits for each update, potentially skipping them if too slow.
#[derive(Clone)]
pub struct Recv<T: Clone> {
    latest: T,
    watch: watch::Receiver<T>,
}

impl<T: Clone> Recv<T> {
    pub(crate) fn new(watch: watch::Receiver<T>) -> Self {
        let latest = watch.borrow().clone();
        Self { latest, watch }
    }

    /// Wait for an unseen value, not including the initial value.
    /// Returns [None] when the [Send] half is dropped with no new value.
    ///
    /// This only returns the latest value so some values may be skipped.
    /// If you want every value, use one of the many channel implementations.
    pub async fn next(&mut self) -> Option<&T> {
        self.watch.changed().await.ok()?;
        self.latest = self.watch.borrow_and_update().clone();
        Some(&self.latest)
    }

    /// Return the initial value or the latest value returned by [Self::next].
    pub fn get(&self) -> &T {
        &self.latest
    }
}
