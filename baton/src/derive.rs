/// Derive a struct, creatig a `XxxSend` and `XxxRecv` struct.
/// Each field has its own channel, allowing you to send and receive updates independently.
///
/// Example:
/// ```rust
/// use baton::Baton;
///
/// #[derive(Baton)]
/// struct MyStruct {
///     pub field1: i32,
///     pub field2: String,
/// }
///
/// /* expands to:
///  *
///  * struct MyStructSend {
///  *     pub field1: baton::Send<i32>,
///  *     pub field2: baton::Send<String>,
///  * }
///  *
///  * struct MyStructRecv {
///  *     pub field1: baton::Recv<i32>,
///  *     pub field2: baton::Recv<String>,
///  * }
///  *
///  * impl MyStruct {
///  *     fn baton(self) -> (MyStructSend, MyStructRecv) {
///  *         let field1 = baton::channel(self.field1);
///  *         let field2 = baton::channel(self.field2);
///  *
///  *         (MyStructSend {
///  *            field1: field1.0,
///  *            field2: field2.0,
///  *         }, MyStructRecv {
///  *            field1: field1.1,
///  *            field2: field2.1,
///  *         })
///  *     }
///  * }
///  */
/// ```
pub use baton_derive::Baton;

#[cfg(test)]
mod test {
    use super::*;
    use futures::FutureExt;

    #[derive(Baton)]
    struct MyStruct {
        pub field1: i32,
        pub field2: String,
    }

    #[test]
    fn simple() {
        let (mut send, mut recv) = MyStruct {
            field1: 42,
            field2: "hello".to_string(),
        }
        .baton();

        // Receive the default values first.
        assert_eq!(recv.field1.recv().now_or_never().unwrap().unwrap(), &42);
        assert_eq!(recv.field2.recv().now_or_never().unwrap().unwrap(), "hello");

        // We block if we try to receive again.
        assert!(recv.field1.recv().now_or_never().is_none());
        assert!(recv.field2.recv().now_or_never().is_none());

        // Send new values.
        send.field1.send(69).ok();
        send.field2.send("world".to_string()).ok();

        // Overwrite one of them before receiving.
        send.field1.send(420).ok();

        assert_eq!(recv.field1.recv().now_or_never().unwrap().unwrap(), &420);
        assert_eq!(recv.field2.recv().now_or_never().unwrap().unwrap(), "world");

        // Drop the sender to close the channel.
        // But first insert a pending update.
        send.field2.send("goodbye".to_string()).ok();
        drop(send);

        // We still get the pending update.
        assert_eq!(
            recv.field2.recv().now_or_never().unwrap().unwrap(),
            "goodbye"
        );

        // The receiver should return None from now on.
        assert!(recv.field1.recv().now_or_never().unwrap().is_none());
        assert!(recv.field2.recv().now_or_never().unwrap().is_none());
    }
}
