//! Baton is a simple channel that only keeps the latest value.
//! If you try to [Send] too quickly, then oops the [Recv] side will drop the baton.
//!
//! Comes with a useful [Baton] macro to create a [Send] and [Recv] half for an entire struct.
//! This allows you to send and receive updates independently for each field so it's clear what changed.
//!
//! The API is inspired by `tokio::sync::watch` but with a simpler design given the 1:1 nature.
//! Notably, [Recv::recv] will return a reference to the next value so you don't have to fumble around with `changed()` state.
//! Additionally, there's no way to accidentally deadlock like with `tokio::sync::watch::Ref`.

// Required for derive to work.
extern crate self as baton;

mod recv;
mod send;

pub use recv::*;
pub use send::*;

#[cfg(feature = "derive")]
mod derive;
#[cfg(feature = "derive")]
pub use derive::*;

use tokio::sync::watch;

/// Create a new channel with a [Send] and [Recv] half.
/// This is a simple channel with no back-pressure and an initial value.
pub fn channel<T: Clone>(value: T) -> (Send<T>, Recv<T>) {
    let (send, recv) = watch::channel(value);
    (Send::new(send), Recv::new(recv))
}
